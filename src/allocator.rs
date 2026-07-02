use crate::bump::Bump;

use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::{LazyLock, Mutex},
};
pub struct AdAllocator {
    pub bump: LazyLock<Mutex<Bump>>,
}

unsafe impl GlobalAlloc for AdAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            System.dealloc(ptr, layout);
        }
    }
}
