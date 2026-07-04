/*  
    Testing different alloc cases. 
    Contains coverage on happy paths and boundries (edges).
*/
use arena_management::{allocator::AdAllocator, bump::Bump};
use std::alloc::{GlobalAlloc, Layout};

#[test]
fn allocate_64_bytes() {
    let arena = AdAllocator { bump: Bump::new() };
    let layout = Layout::from_size_align(64, 8).unwrap();
    let ptr: *mut u8 = arena.bump.bump(layout).unwrap();
    assert!(!ptr.is_null());
}

#[test]
fn allocate_64_bytes_and_is_writable() {
    let data_to_write: u8 = 2;

    let arena = AdAllocator { bump: Bump::new() };
    let layout = Layout::from_size_align(64, 8).unwrap();
    let ptr: *mut u8 = arena.bump.bump(layout).unwrap();
    assert!(!ptr.is_null());

    unsafe {
        ptr.write(data_to_write);
        assert_eq!(ptr.read(), data_to_write);
    }
}

#[test]
fn allocate_past_end() {
    let arena = AdAllocator { bump: Bump::new() };
    for i in 10000..=200000 {
        let data = i + ('A' as usize);
        let layout = Layout::from_size_align(data, 16).unwrap();
        let ptr: Option<*mut u8> = arena.bump.bump(layout);
        if ptr.is_none() {
            return; // OOM hit as expected
        }
    }
}

#[test]
fn allocate_all_space() {
    let arena = AdAllocator { bump: Bump::new() };
    let layout = Layout::from_size_align(arena.bump.end, 16).unwrap();
    let ptr = arena.bump.bump(layout);
    assert!(ptr.is_some());
}

#[test]
fn allocate_all_space_and_1_byte_more() {
    let arena = AdAllocator { bump: Bump::new() };
    let layout = Layout::from_size_align(arena.bump.end + 1, 16).unwrap();
    let ptr = arena.bump.bump(layout);
    assert!(ptr.is_none());
}

#[test]
fn allocate_zero_bytes() {
    let arena = AdAllocator { bump: Bump::new() };
    let layout = Layout::from_size_align(0, 16).unwrap();
    let ptr = arena.bump.bump(layout);
    assert!(ptr.is_some());
}

#[test]
fn allocate_size_bigger_than_arena() {
    let arena = AdAllocator { bump: Bump::new() };
    let layout = Layout::from_size_align(4096 * 4096, 8).unwrap();
    let ptr = arena.bump.bump(layout);
    assert!(ptr.is_none());
}

#[test]
fn allocate_size_direct_from_allocator() {
    let arena: AdAllocator = AdAllocator { bump: Bump::new() };
    let layout = Layout::from_size_align(4096, 16).unwrap();
    let ptr = unsafe { arena.alloc(layout) };
    assert!(!ptr.is_null());
}

#[test]
fn deallocate_does_nothing() {
    let arena: AdAllocator = AdAllocator { bump: Bump::new() };
    let layout = Layout::from_size_align(64, 8).unwrap();
    let null_ptr: *mut u8 = std::ptr::null_mut(); 
    let ptr = unsafe { arena.dealloc(null_ptr, layout) };
    assert_eq!(ptr, ());
}
