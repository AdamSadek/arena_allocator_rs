/*
    Concurrency tests in alloc, this will cover
        1. Sequencing (single-threaded)
        2. Concurrency (multi-threaded)
*/
use arena_management::{allocator::AdAllocator, bump::Bump};
use std::collections::HashSet;
use std::thread;
use std::alloc::Layout;

#[test]
fn single_thread_sequence_test() {
    let arena = AdAllocator { bump: Bump::new() };
    let first: Layout = Layout::from_size_align(64, 8).unwrap();
    let second: Layout = Layout::from_size_align(128, 8).unwrap();
    let first_ptr = arena.bump.bump(first).unwrap() as usize;
    let second_ptr = arena.bump.bump(second).unwrap() as usize;

    assert!(first_ptr <= second_ptr + arena.bump.end)
}

#[test]
fn multi_thread_sequence_test() {
    const NUM_OF_THREADS: usize = 10;
    const ALLOCS_PER_THREAD: usize = 25;

    let arena = AdAllocator { bump: Bump::new() };
    let mut ptr_set: HashSet<usize> = HashSet::new();
    thread::scope(|s| {
        // spawn N threads and keep their handles
        let handles: Vec<_> = (0..NUM_OF_THREADS)
            .map(|_| {
                s.spawn(|| {
                    // thread's own ptr list
                    let mut addrs = Vec::new();
                    for i in 0..ALLOCS_PER_THREAD {
                        let layout = Layout::from_size_align(i + 'A' as usize, 8).unwrap();
                        addrs.push(arena.bump.bump(layout).unwrap() as usize);
                    }
                    addrs
                })
            })
            .collect();
        // join each handle and merge into the set
        for handle in handles {
            ptr_set.extend(handle.join().unwrap()); // main thread
        }
    });

    assert_eq!(ptr_set.len(), (NUM_OF_THREADS * ALLOCS_PER_THREAD));

}
