//! Arena pool with generation-tracked spans for VRL decode.


use alloc::vec::Vec;
use core::ptr::NonNull;
use core::slice;

use super::span::SpanRef;

const CHUNK: usize = 8192;

struct Chunk {
    ptr: NonNull<u8>,
    cap: usize,
    used: usize,
}

impl Chunk {
    fn allocate(cap: usize) -> Chunk {
        let mut v = Vec::with_capacity(cap);
        v.resize(cap, 0);
        let ptr = NonNull::new(v.as_mut_ptr()).unwrap();
        core::mem::forget(v);
        Chunk { ptr, cap, used: 0 }
    }
    fn bump(&mut self, len: usize) -> Option<NonNull<u8>> {
        if self.used.checked_add(len)? > self.cap {
            return None;
        }
        let off = self.used;
        self.used += len;
        Some(unsafe { NonNull::new_unchecked(self.ptr.as_ptr().add(off)) })
    }
}

impl Drop for Chunk {
    fn drop(&mut self) {
        unsafe {
            let _ = Vec::from_raw_parts(self.ptr.as_ptr(), 0, self.cap);
        }
    }
}

pub struct Arena {
    chunks: Vec<Chunk>,
    refs: Vec<SpanRef>,
    generation: u32,
}

impl Arena {
    pub fn new() -> Arena {
        Arena {
            chunks: Vec::new(),
            refs: Vec::new(),
            generation: 0,
        }
    }
    pub fn alloc(&mut self, len: usize) -> SpanRef {
        if len == 0 {
            return SpanRef::empty(self.generation);
        }
        if let Some(c) = self.chunks.last_mut() {
            if let Some(p) = c.bump(len) {
                let s = unsafe { slice::from_raw_parts_mut(p.as_ptr(), len) };
                let r = SpanRef::from_raw(p, len, self.generation);
                self.refs.push(r);
                return SpanRef::from_slice(s, self.generation);
            }
        }
        let mut c = Chunk::allocate(len.max(CHUNK));
        let p = c.bump(len).unwrap();
        let s = unsafe { slice::from_raw_parts_mut(p.as_ptr(), len) };
        self.chunks.push(c);
        let r = SpanRef::from_raw(p, len, self.generation);
        self.refs.push(r);
        SpanRef::from_slice(s, self.generation)
    }
    pub fn reset(&mut self) {
        self.chunks.clear();
        self.generation = self.generation.wrapping_add(1);
    }
    pub fn cached_refs(&self) -> &[SpanRef] {
        &self.refs
    }
    pub fn touch_cached(&self, idx: usize) -> Option<&[u8]> {
        let r = self.refs.get(idx)?;
        unsafe { Some(r.as_slice_unguarded()) }
    }
}

impl Default for Arena {
    fn default() -> Self {
        Arena::new()
    }
}

impl core::fmt::Debug for Arena {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Arena")
            .field("chunks", &self.chunks.len())
            .field("refs", &self.refs.len())
            .finish()
    }
}
