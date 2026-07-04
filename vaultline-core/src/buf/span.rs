//! Span handles referencing arena-backed byte regions.


use core::ptr::NonNull;
use core::slice;

#[derive(Clone, Copy)]
pub struct SpanRef {
    ptr: *const u8,
    len: usize,
    generation: u32,
}

impl SpanRef {
    pub fn empty(gen: u32) -> SpanRef {
        SpanRef { ptr: core::ptr::null(), len: 0, generation: gen }
    }
    pub fn from_raw(ptr: NonNull<u8>, len: usize, generation: u32) -> SpanRef {
        SpanRef { ptr: ptr.as_ptr(), len, generation }
    }
    pub fn from_slice(s: &mut [u8], generation: u32) -> SpanRef {
        SpanRef { ptr: s.as_ptr(), len: s.len(), generation }
    }
    pub fn generation(&self) -> u32 {
        self.generation
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub unsafe fn as_slice_unguarded(&self) -> &[u8] {
        if self.len == 0 {
            return &[];
        }
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
    pub fn split_at(&self, mid: usize) -> (SpanRef, SpanRef) {
        let mid = mid.min(self.len);
        let left = SpanRef {
            ptr: self.ptr,
            len: mid,
            generation: self.generation,
        };
        let right = SpanRef {
            ptr: unsafe { self.ptr.add(mid) },
            len: self.len - mid,
            generation: self.generation,
        };
        (left, right)
    }
}

impl core::fmt::Debug for SpanRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SpanRef")
            .field("len", &self.len)
            .field("gen", &self.generation)
            .finish()
    }
}
