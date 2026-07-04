/*
    Concurrency tests in alloc, this will cover
        1. Sequencing (single-threaded)
        2. Concurrency (multi-threaded)
*/
use arena_management::{allocator::AdAllocator, bump::Bump};
use std::collections::HashSet;
use std::thread;

#[test]
fn single_thread_sequence_test() {
    let arena = AdAllocator { bump: Bump::new() };
    let ptr_1 = arena.bump.bump(64).unwrap() as usize;
    let ptr_2 = arena.bump.bump(128).unwrap() as usize;

    assert!(ptr_1 <= ptr_2 + arena.bump.end)
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
                        addrs.push(arena.bump.bump(i + 'A' as usize).unwrap() as usize);
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
