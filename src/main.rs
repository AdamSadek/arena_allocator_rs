mod allocator;
mod bump;

use allocator::AdAllocator;
use bump::Bump;

/*
* Memory arena alllocator & reusable harness
*/
#[global_allocator]
static GLOBAL: AdAllocator = AdAllocator { bump: Bump::new() };

fn main() {
    let arena = Bump::new();
    let mut addresses_used: Vec<*mut u8> = Vec::new();
    for i in 0..=63 {
        let data = i + ('A' as usize);
        let ptr = arena.bump(data);
        addresses_used.push(ptr);
        dbg!(ptr);
    }
    dbg!(&addresses_used);
}
