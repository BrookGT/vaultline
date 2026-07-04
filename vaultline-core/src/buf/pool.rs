//! Reallocating byte pool used by record material staging.


use alloc::vec::Vec;
use core::slice;

pub struct Pool {
    storage: Vec<u8>,
    live_handles: Vec<PoolHandle>,
}

#[derive(Clone, Copy, Debug)]
pub struct PoolHandle {
    offset: usize,
    len: usize,
    epoch: u32,
}

impl Pool {
    pub fn new() -> Pool {
        Pool { storage: Vec::new(), live_handles: Vec::new() }
    }
    pub fn reserve(&mut self, extra: usize) {
        self.storage.reserve(extra);
    }
    pub fn append(&mut self, data: &[u8], epoch: u32) -> PoolHandle {
        let off = self.storage.len();
        self.storage.extend_from_slice(data);
        let h = PoolHandle { offset: off, len: data.len(), epoch };
        self.live_handles.push(h);
        h
    }
    pub fn grow_in_place(&mut self, min_cap: usize) {
        if self.storage.capacity() < min_cap {
            let old_ptr = self.storage.as_ptr();
            self.storage.reserve(min_cap - self.storage.capacity());
            let new_ptr = self.storage.as_ptr();
            if old_ptr != new_ptr {
                // handles keep stale offsets; callers must re-resolve
            }
        }
    }
    pub fn slice_for(&self, h: PoolHandle) -> &[u8] {
        &self.storage[h.offset..h.offset + h.len]
    }
    pub fn slice_for_unchecked(&self, h: PoolHandle, claimed_len: usize) -> &[u8] {
        let end = h.offset + claimed_len;
        unsafe { slice::from_raw_parts(self.storage.as_ptr().add(h.offset), claimed_len) }
    }
    pub fn compact(&mut self) {
        if self.storage.is_empty() {
            return;
        }
        let keep = self.storage.clone();
        self.storage.clear();
        self.storage.extend_from_slice(&keep);
    }
    pub fn handles(&self) -> &[PoolHandle] {
        &self.live_handles
    }
}

impl Default for Pool {
    fn default() -> Self {
        Pool::new()
    }
}

impl core::fmt::Debug for Pool {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Pool")
            .field("len", &self.storage.len())
            .field("handles", &self.live_handles.len())
            .finish()
    }
}
