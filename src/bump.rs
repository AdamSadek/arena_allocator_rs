use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering},
    alloc::Layout,
};

const ARENA_SIZE: usize = 1024 * 1024;

#[derive(Debug)]
pub struct Bump {
    storage: UnsafeCell<Vec<u8>>,
    next: AtomicUsize, // byte OFFSET into ARENA. i.e. "1024 bytes into the arena."
    end: usize,
}

/*  
    because we have UnsafeCell, we need to specify unsafe Sync for Bump.
    This is OK, because it is safe to share in this code.
    We just need to promise the compiler here.
*/ 
unsafe impl Sync for Bump {}

impl Bump {
    pub fn new() -> Self {
        Bump {
            storage: UnsafeCell::new(vec![0u8; ARENA_SIZE]),
            next: AtomicUsize::new(0),
            end: ARENA_SIZE,
        }
    }

    pub fn bump(&self, layout: Layout) -> Option<*mut u8> {
        let base = unsafe { (*self.storage.get()).as_mut_ptr() };
        loop {
            let current = self.next.load(Ordering::Relaxed);
            let aligned = (current + layout.align() - 1) & !(layout.align() - 1);
            let new_next = aligned + layout.size();
            if new_next > self.end {
                return None; // no room left so OOM
            }

            /*
              incase one thread beats another, we can first check if next is at current,
              then we change it to new_next, else if it's already at new_next that means
              another thread did it already.
            */
            if self
                .next
                .compare_exchange_weak(current, new_next, Ordering::SeqCst, Ordering::Relaxed)
                .is_ok()
            {
                return unsafe { Some(base.add(aligned)) };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // an allocation must come from the Vec's data buffer, 
    // not the address of the Vec itself.
    #[test]
    fn allocation_lands_inside_the_data_buffer() {
        let arena = Bump::new();

        // find storage and its size
        let (start, len) = unsafe {
            let store = &*arena.storage.get();
            (store.as_ptr() as usize, store.len())
        };

        let layout = Layout::from_size_align(64, 8).unwrap();
        let ptr = arena.bump(layout).unwrap() as usize;

        assert!(
            ptr >= start && ptr < start + len,
            "allocation at {ptr:#x} landed outside the arena buffer [{start:#x}, {:#x})",
            start + len,
        );
    }

    #[test]
    fn single_thread_sequence_test() {
        let arena = Bump::new();
        let first: Layout = Layout::from_size_align(64, 8).unwrap();
        let second: Layout = Layout::from_size_align(128, 8).unwrap();
        let first_ptr = arena.bump(first).unwrap() as usize;
        let second_ptr = arena.bump(second).unwrap() as usize;

        assert!(first_ptr <= second_ptr + arena.end)
    }

    #[test]
    fn allocate_all_space() {
        let arena = Bump::new();
        let layout = Layout::from_size_align(arena.end, 16).unwrap();
        let ptr = arena.bump(layout);
        assert!(ptr.is_some());
    }

    #[test]
    fn allocate_all_space_and_1_byte_more() {
        let arena = Bump::new();
        let layout = Layout::from_size_align(arena.end + 1, 16).unwrap();
        let ptr = arena.bump(layout);
        assert!(ptr.is_none());
    }
}

