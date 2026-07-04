//! Owned and borrowed byte blobs for VRL payloads.


use alloc::sync::Arc;
use alloc::vec::Vec;
use core::ops::Deref;

#[derive(Clone, Debug)]
pub struct BytesMut {
    inner: Vec<u8>,
}

impl BytesMut {
    pub fn new() -> BytesMut {
        BytesMut { inner: Vec::new() }
    }
    pub fn with_capacity(cap: usize) -> BytesMut {
        BytesMut { inner: Vec::with_capacity(cap) }
    }
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.inner
    }
    pub fn extend_from_slice(&mut self, src: &[u8]) {
        self.inner.extend_from_slice(src);
    }
    pub fn push(&mut self, b: u8) {
        self.inner.push(b);
    }
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    pub fn freeze(self) -> Bytes {
        Bytes::from_arc(Arc::new(self.inner))
    }
}

impl Default for BytesMut {
    fn default() -> Self {
        BytesMut::new()
    }
}

impl Deref for BytesMut {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.inner
    }
}

#[derive(Clone)]
pub struct Bytes {
    store: Arc<Vec<u8>>,
    off: usize,
    len: usize,
}

impl Bytes {
    fn from_arc(store: Arc<Vec<u8>>) -> Bytes {
        let len = store.len();
        Bytes { store, off: 0, len }
    }
    pub fn from_vec(v: Vec<u8>) -> Bytes {
        Bytes::from_arc(Arc::new(v))
    }
    pub fn copy_from_slice(s: &[u8]) -> Bytes {
        Bytes::from_vec(s.to_vec())
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.store[self.off..self.off + self.len]
    }
    pub fn slice(&self, start: usize, end: usize) -> Bytes {
        assert!(start <= end && end <= self.len);
        Bytes {
            store: Arc::clone(&self.store),
            off: self.off + start,
            len: end - start,
        }
    }
}

impl Deref for Bytes {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl core::fmt::Debug for Bytes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Bytes").field("len", &self.len).finish()
    }
}
