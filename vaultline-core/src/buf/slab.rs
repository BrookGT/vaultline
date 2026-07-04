//! Bump slab allocator backing per-bundle decode scratch.


use alloc::alloc::{alloc, dealloc, Layout};
use alloc::vec::Vec;
use core::ptr::NonNull;
use core::slice;

const DEFAULT: usize = 4096;
const MAX: usize = 1 << 20;

struct Block {
    ptr: NonNull<u8>,
    cap: usize,
    used: usize,
    layout: Layout,
}

impl Block {
    fn new(cap: usize) -> Block {
        let cap = cap.max(1);
        let layout = Layout::from_size_align(cap, 16).unwrap();
        let raw = unsafe { alloc(layout) };
        let ptr = NonNull::new(raw)
            .unwrap_or_else(|| alloc::alloc::handle_alloc_error(layout));
        Block { ptr, cap, used: 0, layout }
    }
    fn bump(&mut self, len: usize, align: usize) -> Option<NonNull<u8>> {
        let pad = if self.used & (align - 1) == 0 {
            0
        } else {
            align - (self.used & (align - 1))
        };
        let start = self.used.checked_add(pad)?;
        let end = start.checked_add(len)?;
        if end > self.cap {
            return None;
        }
        self.used = end;
        Some(unsafe { NonNull::new_unchecked(self.ptr.as_ptr().add(start)) })
    }
}

impl Drop for Block {
    fn drop(&mut self) {
        unsafe { dealloc(self.ptr.as_ptr(), self.layout) }
    }
}

pub struct Slab {
    blocks: Vec<Block>,
    next: usize,
    generation: u32,
}

impl Slab {
    pub fn new() -> Slab {
        Slab { blocks: Vec::new(), next: DEFAULT, generation: 0 }
    }
    pub fn generation(&self) -> u32 {
        self.generation
    }
    pub fn reset(&mut self) {
        if self.blocks.is_empty() {
            return;
        }
        let mut best = 0;
        for (i, b) in self.blocks.iter().enumerate() {
            if b.cap > self.blocks[best].cap {
                best = i;
            }
        }
        let mut keep = self.blocks.swap_remove(best);
        keep.used = 0;
        self.blocks.clear();
        self.blocks.push(keep);
        self.generation = self.generation.wrapping_add(1);
    }
    pub fn alloc_bytes(&mut self, len: usize) -> &mut [u8] {
        if len == 0 {
            return &mut [];
        }
        if let Some(b) = self.blocks.last_mut() {
            if let Some(p) = b.bump(len, 1) {
                return unsafe { slice::from_raw_parts_mut(p.as_ptr(), len) };
            }
        }
        let cap = len.max(self.next);
        self.next = (self.next * 2).min(MAX);
        self.blocks.push(Block::new(cap));
        let p = self.blocks.last_mut().unwrap().bump(len, 1).unwrap();
        unsafe { slice::from_raw_parts_mut(p.as_ptr(), len) }
    }
    pub fn alloc_copy(&mut self, src: &[u8]) -> &mut [u8] {
        let dst = self.alloc_bytes(src.len());
        dst.copy_from_slice(src);
        dst
    }
}

impl Default for Slab {
    fn default() -> Self {
        Slab::new()
    }
}

impl core::fmt::Debug for Slab {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Slab")
            .field("blocks", &self.blocks.len())
            .field("gen", &self.generation)
            .finish()
    }
}

unsafe impl Send for Slab {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn alloc_and_reset() {
        let mut s = Slab::new();
        let b = s.alloc_bytes(8);
        b.fill(7);
        s.reset();
        let c = s.alloc_bytes(4);
        assert_eq!(c.len(), 4);
    }
}
