/*
    Alignment testing, ensuring that the alignment is working as expected
    based on the ptr we get back. I'm using alignment of 8 bytes here.
*/
use arena_management::{allocator::AdAllocator, bump::Bump};
use std::alloc::Layout;

#[test]
fn pointer_returned_has_valid_alignment() {
    let arena = AdAllocator { bump: Bump::new() };
    let layout: Layout = Layout::from_size_align(64, 8).unwrap();
    let ptr: *mut u8 = arena.bump.bump(layout).unwrap();
    assert!(!ptr.is_null());
    assert_eq!(ptr as usize % 8, 0);
}

#[test]
fn odd_size_aligned_correctly() {
    let arena = AdAllocator { bump: Bump::new() };
    let odd_layout = Layout::from_size_align(97, 8).unwrap();
    let even_layout = Layout::from_size_align(12, 8).unwrap();
    let _ = arena.bump.bump(odd_layout).unwrap();
    let ptr: *mut u8 = arena.bump.bump(even_layout).unwrap();

    assert!(!ptr.is_null());
    assert_eq!(ptr as usize % 8, 0);
}