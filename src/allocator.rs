use crate::bump::Bump;

use std::alloc::{GlobalAlloc, Layout};
use std::ptr;

/// A [`GlobalAlloc`] that serves every allocation from one [`Bump`] arena.
///
/// The arena never frees single allocations, so this only fits cases where the
/// total memory used stays under the arena size for the whole run (like a small
/// or short-lived program). Once the arena is full,
/// [`alloc`](AdAllocator::alloc) returns null and the allocation fails.
#[derive(Debug)]
pub struct AdAllocator {
    /// The arena behind every allocation.
    pub bump: Bump,
}

// SAFETY: `GlobalAlloc` needs allocations to be valid, aligned, and not
// overlapping. `Bump::bump` gives us all three: it uses the size and alignment
// from `layout` and claims each region on its own with an atomic swap.
unsafe impl GlobalAlloc for AdAllocator {
    /// Allocates from the arena, or returns a null pointer when it is full, which
    /// is how [`GlobalAlloc`] expects a failed allocation to look.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.bump.bump(layout).unwrap_or(ptr::null_mut())
    }

    /// Does nothing. A bump arena can't free single allocations. The memory is
    /// only freed when the whole arena is dropped.
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
