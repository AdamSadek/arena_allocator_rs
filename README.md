# Arena Allocator

A small bump allocator for Rust.

A bump allocator hands out memory by keeping one cursor and moving it forward on
each allocation. That's all it does. You can't free a single allocation on its
own. Everything is freed at once when the arena is dropped. The trade you make is
that you give up per-allocation freeing, and in return allocation is very fast
and works fine across threads.

This is useful when you allocate a lot of small things that all live and die
together, like the work done inside one request, one frame, or one short-lived
program.

## What's in here

- `Bump` is the arena. It owns a fixed 1 MiB buffer and gives out slices of it.
  It is lock-free and safe to share between threads.
- `AdAllocator` wraps a `Bump` so it can be used as a `#[global_allocator]`.

## Using the arena directly

```rust
use arena_management::bump::Bump;
use std::alloc::Layout;

let arena = Bump::new();
let layout = Layout::from_size_align(64, 8).unwrap();

let ptr = arena.bump(layout).expect("arena has room");
assert!(!ptr.is_null());
```

`bump` returns `None` when the arena runs out of room, so you always get to
decide what happens when it is full.

## Using it as the global allocator

`AdAllocator` serves every allocation in your program from one arena. Because
the arena never frees single allocations, this only fits programs whose total
memory use stays under the arena size for the whole run. Once it fills up,
allocation fails.

## Running the tests

```bash
cargo test
```
