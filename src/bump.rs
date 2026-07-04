use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering},
    alloc::Layout,
};

/// Total capacity of a single arena, in bytes (1 MiB).
const ARENA_SIZE: usize = 1024 * 1024;

/// A fixed-size, lock-free bump allocator.
///
/// The arena holds one buffer and gives out slices of it by moving a single
/// atomic cursor forward (see [`Bump::bump`]). Allocations are never freed one
/// at a time. All the memory is freed when the `Bump` is dropped.
///
/// `Bump` is [`Sync`], so you can share it between threads. Two threads that
/// allocate at the same time settle it with a compare-and-swap loop.
#[derive(Debug)]
pub struct Bump {
    /// The buffer we allocate from. It sits in an [`UnsafeCell`] because
    /// [`Bump::bump`] gives out `*mut u8` pointers into it through `&self`.
    storage: UnsafeCell<Vec<u8>>,
    /// The cursor. This is a byte offset into the buffer (so `1024` means "1024
    /// bytes in"), not a real address.
    next: AtomicUsize,
    /// The offset just past the end of the buffer. Allocations can't go beyond
    /// it. Equal to `ARENA_SIZE`.
    end: usize,
}

// SAFETY: an `UnsafeCell` is not `Sync` on its own, so we opt in by hand.
// Sharing a `Bump` between threads is fine because the buffer only changes
// through `bump`, which claims each region with an atomic compare-and-swap on
// `next`. Two threads can never get the same region, so the bytes never race.
unsafe impl Sync for Bump {}

impl Bump {
    /// Makes a new, empty arena that can hold `ARENA_SIZE` bytes.
    ///
    /// The buffer is zeroed when it is created, so this cost grows with the
    /// arena size.
    pub fn new() -> Self {
        Bump {
            storage: UnsafeCell::new(vec![0u8; ARENA_SIZE]),
            next: AtomicUsize::new(0),
            end: ARENA_SIZE,
        }
    }

    /// Allocates `layout.size()` bytes with the alignment from `layout`.
    ///
    /// Returns a pointer to the start of the region, or [`None`] if the arena is
    /// out of room. The pointer stays valid as long as the `Bump` is alive, and
    /// no other caller ever gets the same region.
    ///
    /// It is lock-free, so many threads can call it at once.
    ///
    /// # Examples
    ///
    /// ```
    /// use arena_management::bump::Bump;
    /// use std::alloc::Layout;
    ///
    /// let arena = Bump::new();
    /// let layout = Layout::from_size_align(64, 8).unwrap();
    /// let ptr = arena.bump(layout).unwrap();
    /// assert_eq!(ptr as usize % 8, 0); // honours the requested alignment
    /// ```
    pub fn bump(&self, layout: Layout) -> Option<*mut u8> {
        // SAFETY: this only reads the buffer's data pointer, it does not change
        // the buffer. The pointer stays good because the buffer is never resized
        // after it is created.
        let base = unsafe { (*self.storage.get()).as_mut_ptr() };
        loop {
            let current = self.next.load(Ordering::Relaxed);
            // Round the cursor up to the alignment we need. This works because
            // `Layout` promises the alignment is a power of two.
            let aligned = (current + layout.align() - 1) & !(layout.align() - 1);
            let new_next = aligned + layout.size();
            if new_next > self.end {
                return None;
            }

            // Only claim the region if no other thread moved the cursor while we
            // were reading it. If one did, `current` is stale, so we loop and try
            // again. The `_weak` version can also fail for no reason, and the
            // loop handles that too.
            if self
                .next
                .compare_exchange_weak(current, new_next, Ordering::SeqCst, Ordering::Relaxed)
                .is_ok()
            {
                // SAFETY: `aligned` is inside the buffer, and the swap above just
                // claimed this region for us alone, so no one else can use it.
                return unsafe { Some(base.add(aligned)) };
            }
        }
    }
}

impl Default for Bump {
    fn default() -> Self {
        Self::new()
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

