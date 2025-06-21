use libc;
use std::alloc::{alloc, dealloc, GlobalAlloc, Layout, LayoutErr};
use std::ptr;
use std::sync::Mutex;

use std::cell::UnsafeCell;



struct BumpAlloc {
    cap: usize,
    pointer: *mut u8,
    offset: Mutex<usize>
}

impl BumpAlloc {
    pub fn new(cap:usize) -> Self {
        let layout = Layout::from_size_align(cap, 8).unwrap();
        let ptr = unsafe {
            alloc(layout)
        };
        Self {
            cap: cap,
            pointer: ptr,
            offset: Mutex::new(0)
        }

    }
    fn allocate(&self, layout: Layout) -> *mut u8 {
        let mut reg = self.offset.lock().unwrap();
        let old_offset = *reg;
        let align = layout.align();
        let aligned_offset = if old_offset % align == 0 {
            old_offset
        } else {
            old_offset + (align - (old_offset % align))
        };
        let size = layout.size();
        if aligned_offset > self.cap {
            panic!("OVERFLOW")
        }
        let res = aligned_offset + size;
        *reg = res;
        
        unsafe {
            self.pointer.add(aligned_offset)
        }

    } 
    fn deallocate(&self, ptr: *mut u8, layout: Layout) {
        
    }

    fn reset(&self)  {
        let mut current_offset = self.offset.lock().unwrap();
        *current_offset = 0;
    }
    
}
unsafe impl Sync for BumpAlloc {
    
}
unsafe impl GlobalAlloc for BumpAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocate(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.deallocate(ptr, layout);
    }   
}

static mut HEAP_MEM : [u8; 10000000] = [0; 10000000];

#[global_allocator]
static global_alloc: BumpAlloc = BumpAlloc {
    pointer: unsafe {HEAP_MEM.as_mut_ptr()},
    offset: Mutex::new(0),
    cap: unsafe{HEAP_MEM.len()},
};
fn main() {
    let v1 = vec![10, 10, 10, 10];

    println!("{:?}", v1);
}
