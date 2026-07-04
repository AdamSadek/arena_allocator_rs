//! A small, lock-free bump (arena) allocator.
//!
//! Each allocation just moves a cursor forward and hands back the next aligned
//! slice of a fixed buffer. You can't free one allocation at a time. All the
//! memory is freed together when the arena is dropped. In return, allocation is
//! very cheap and works well across threads.
//!
//! - [`bump::Bump`] is the arena.
//! - [`allocator::AdAllocator`] wraps it in [`std::alloc::GlobalAlloc`] so it can
//!   be used as a `#[global_allocator]`.
//!
//! # Examples
//!
//! ```
//! use arena_management::bump::Bump;
//! use std::alloc::Layout;
//!
//! let arena = Bump::new();
//! let layout = Layout::from_size_align(64, 8).unwrap();
//! let ptr = arena.bump(layout).expect("arena has room");
//! assert!(!ptr.is_null());
//! ```

pub mod allocator;
pub mod bump;
