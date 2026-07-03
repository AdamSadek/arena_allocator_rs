/*
    Concurrency tests in alloc, this will cover
        1. Sequencing (single-threaded)
        2. Concurrency (multi-threaded)
*/
use arena_management::{allocator::AdAllocator, bump::Bump};
use std::collections::HashSet;
use std::thread;

// using barrier to make multiple threads wait at the same point,
// until all of them arrive.
use std::sync::Barrier;

#[test]
fn single_thread_sequence_test() {
    let arena = AdAllocator { bump: Bump::new() };
    let ptr_1 = arena.bump.bump(64).unwrap() as usize;
    let ptr_2 = arena.bump.bump(128).unwrap() as usize;;

    assert!(ptr_1 <= ptr_2 + arena.bump.end)
}

#[test]
fn multi_thread_sequence_test() {
    let arena = AdAllocator { bump: Bump::new() };
    // let mut ptr_set: HashSet<*mut u8> = HashSet::new();
    thread::scope(|s| {
        s.spawn(|| {
            arena.bump.bump(64).unwrap();
        });
    });
}
