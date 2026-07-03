use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering},
};

const ARENA_SIZE: usize = 1024*1024;

#[derive(Debug)]
pub struct Bump {
    storage: Box<UnsafeCell<[u8; ARENA_SIZE]>>,
    next: AtomicUsize, // byte OFFSET into ARENA. i.e. "1024 bytes into the arena."
    pub end: usize,
    align: usize,
}

unsafe impl Sync for Bump {}

impl Default for Bump {
    fn default() -> Self {
        Self::new()
    }
}

impl Bump {
    pub fn new() -> Self {
        Bump {
            storage: Box::new(UnsafeCell::new([0u8; ARENA_SIZE])),
            next: AtomicUsize::new(0),
            end: ARENA_SIZE,
            align: 8,
        }
    }

    pub fn bump(&self, size: usize) -> Option<*mut u8> {
        let base = self.storage.get() as *mut u8;
        loop {
            let current = self.next.load(Ordering::Relaxed);
            let aligned = (current + self.align - 1) & !(self.align - 1); // revisit
            let new_next = aligned + size;
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
