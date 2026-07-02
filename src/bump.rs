use std::alloc::{GlobalAlloc, Layout, System};
use std::ptr;

#[derive(Debug)]
pub struct Bump {
    pub next: *mut u8,
    pub end: *mut u8,
}

// promising the compiler it's safe! (thanks to mutex)
unsafe impl Send for Bump {}

impl Bump {
    pub fn new() -> Self {
        let layout = Layout::from_size_align(4096, 8).unwrap();
        let ptr = unsafe { System.alloc(layout) };
        let bump = Bump {
            next: ptr,
            end: unsafe { ptr.add(layout.size()) },
        };
        dbg!(&bump);
        bump
    }
    pub fn bump(&mut self, size: usize) -> *mut u8 {
        println!(
            "ptr is: {:?}, end is: {:?}",
            self.next as usize + size,
            (self.end as usize)
        );
        assert!(self.next as usize + size <= self.end as usize);
        // move ptr to next free space (bump forward)
        let start = self.next;
        unsafe {
            // TODO: what if size ws near usize:MAX? we should handle this
            self.next = self.next.add(size);
        }
        start
    }
}
