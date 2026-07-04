/*  
    Testing different alloc cases. 
    Contains coverage on happy paths and boundries (edges).
*/
use arena_management::{allocator::AdAllocator, bump::Bump};

#[test]
fn allocate_64_bytes() {
    let arena = AdAllocator { bump: Bump::new() };
    let ptr: *mut u8 = arena.bump.bump(64).unwrap();
    assert!(!ptr.is_null());
}

#[test]
fn allocate_64_bytes_and_is_writable() {
    let data_to_write: u8 = 2;

    let arena = AdAllocator { bump: Bump::new() };
    let ptr: *mut u8 = arena.bump.bump(64).unwrap();
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
        let ptr: Option<*mut u8> = arena.bump.bump(data);
        if ptr.is_none() {
            return; // OOM hit as expected
        }
    }
}

#[test]
fn allocate_all_space() {
    let arena = AdAllocator { bump: Bump::new() };
    let ptr = arena.bump.bump(arena.bump.end);
    assert!(ptr.is_some());
}

#[test]
fn allocate_all_space_and_1_byte_more() {
    let arena = AdAllocator { bump: Bump::new() };
    let ptr = arena.bump.bump(arena.bump.end + 1);
    assert!(ptr.is_none());
}

#[test]
fn allocate_zero_bytes() {
    let arena = AdAllocator { bump: Bump::new() };
    let ptr = arena.bump.bump(0);
    assert!(ptr.is_some());
}

#[test]
fn allocate_size_bigger_than_arena() {
    let arena = AdAllocator { bump: Bump::new() };
    let ptr = arena.bump.bump(4096 * 4096);
    assert!(ptr.is_none());
}
