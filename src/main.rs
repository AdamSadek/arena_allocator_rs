mod allocator;
mod bump;

use allocator::AdAllocator;
use bump::Bump;

/*
* Memory arena alllocator & reusable harness
*/
#[global_allocator]
static GLOBAL: AdAllocator = AdAllocator { bump: Bump::new() };

fn main() {}
