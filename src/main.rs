
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::cell::UnsafeCell;

struct BumpAlloc {
    cap: usize,
    pointer: *mut u8,
    offset: UnsafeCell<usize>
}

fn main() {
    println!("Hello World");
}
