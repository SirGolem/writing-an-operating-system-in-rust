use crate::allocator::{Locked, align_up};
use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::null_mut,
};

pub struct BumpAllocator {
    allocation_count: usize,
    heap_end_address: usize,
    heap_start_address: usize,
    next_address: usize,
}

impl BumpAllocator {
    pub unsafe fn initialize(&mut self, heap_size: usize, heap_start_address: usize) -> () {
        self.heap_end_address = heap_size + heap_start_address;
        self.heap_start_address = heap_start_address;
        self.next_address = heap_start_address;
    }

    pub const fn new() -> Self {
        Self {
            allocation_count: 0,
            heap_end_address: 0,
            heap_start_address: 0,
            next_address: 0,
        }
    }
}

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.lock();

        let start_address = align_up(allocator.next_address, layout.align());
        let end_address = match start_address.checked_add(layout.size()) {
            | None => return null_mut(),
            | Some(end_address) => end_address,
        };

        if end_address > allocator.heap_end_address {
            null_mut()
        } else {
            allocator.allocation_count += 1;
            allocator.next_address += layout.size();
            start_address as *mut u8
        }
    }

    unsafe fn dealloc(&self, _pointer: *mut u8, _layout: Layout) -> () {
        let mut allocator = self.lock();

        allocator.allocation_count -= 1;
        if allocator.allocation_count == 0 {
            allocator.next_address = allocator.heap_start_address;
        }
    }
}
