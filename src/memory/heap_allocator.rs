extern crate alloc;
use alloc::alloc::{ GlobalAlloc, Layout };

#[global_allocator]
static HEAP_MEM;
