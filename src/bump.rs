use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering},
    alloc::Layout,
};

const ARENA_SIZE: usize = 1024 * 1024;

#[derive(Debug)]
pub struct Bump {
    storage: Box<UnsafeCell<[u8; ARENA_SIZE]>>,
    next: AtomicUsize, // byte OFFSET into ARENA. i.e. "1024 bytes into the arena."
    pub end: usize,
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
            storage: Box::new(UnsafeCell::new([0u8; ARENA_SIZE])),
            next: AtomicUsize::new(0),
            end: ARENA_SIZE,
        }
    }

    pub fn bump(&self, layout: Layout) -> Option<*mut u8> {
        let base = self.storage.get() as *mut u8;
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
