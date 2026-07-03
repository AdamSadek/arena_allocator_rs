/*
    Alignment testing, ensuring that the alignment is working as expected
    based on the ptr we get back. I'm using alignment of 8 bytes here.
*/
use arena_management::{allocator::AdAllocator, bump::Bump};

#[test]
fn pointer_returned_has_valid_alignment() {
    let arena = AdAllocator { bump: Bump::new() };
    let ptr: *mut u8 = arena.bump.bump(64).unwrap();
    assert!(!ptr.is_null());
    assert_eq!(ptr as usize % 8, 0);
}

#[test]
fn odd_size_aligned_correctly() {
    let arena = AdAllocator { bump: Bump::new() };
    let _ = arena.bump.bump(97).unwrap();
    let ptr: *mut u8 = arena.bump.bump(12).unwrap();

    assert!(!ptr.is_null());
    assert_eq!(ptr as usize % 8, 0);
}