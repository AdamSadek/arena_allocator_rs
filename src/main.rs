mod allocator;
mod bump;

use allocator::AdAllocator;
use bump::Bump;
use std::sync::{LazyLock, Mutex};

/*
 * Memory arena alllocator & reusable harness
 *
 */
#[global_allocator]

static GLOBAL: AdAllocator = AdAllocator {
    bump: LazyLock::new(|| Mutex::new(Bump::new())),
};

fn main() {
    let mut arena = Bump::new();
    for i in 0..=100 {
        print!("i: {} ", i);
        arena.bump(i);
    }
}
