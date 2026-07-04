#![no_main]
use libfuzzer_sys::fuzz_target;
use vaultline_core::buf::{Arena, Pool, Slab};
use vaultline_core::core::{Epoch, Limits};

fuzz_target!(|data: &[u8]| {
    let mut arena = Arena::new();
    let mut slab = Slab::new();
    let mut pool = Pool::new();
    let staged = slab.alloc_copy(data);
    let _ = arena.alloc(data.len());
    let h = pool.append(staged, slab.generation());
    pool.grow_in_place(data.len().saturating_add(1024));
    let _ = pool.slice_for(h);
    let _ = pool.slice_for_unchecked(h, data.len());
    let _ = arena.touch_cached(0);
    arena.reset();
    let _ = arena.touch_cached(0);
    if let Ok(bundle) = vaultline_core::parse_bundle(data) {
        let _ = vaultline_core::normalize::normalize_bundle(&bundle);
        let _ = vaultline_core::export::encode_bundle(&bundle);
        let _ = vaultline_core::validate::validate_bundle(&bundle, &Limits::default(), Epoch(0));
    }
});
