use crate::bump::Bump;

use std::alloc::{GlobalAlloc, Layout};

pub struct AdAllocator {
    pub bump: Bump,
}

unsafe impl GlobalAlloc for AdAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.bump.bump(layout.size()).unwrap()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump arenas dont free individual allocations.
        // The arena's memory is only reclaimed as a whole.
    }
}
