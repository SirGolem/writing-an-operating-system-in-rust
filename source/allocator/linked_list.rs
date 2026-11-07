use crate::allocator::{Locked, align_up};
use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::null_mut,
};

pub struct LinkedListAllocator {
    head_node: Node,
}

impl LinkedListAllocator {
    unsafe fn add_free_region(&mut self, size: usize, start_address: usize) -> () {
        assert!(size >= size_of::<Node>());
        assert_eq!(align_up(start_address, align_of::<Node>()), start_address);

        let mut node = Node::new(size);
        node.next_node = self.head_node.next_node.take();
        let node_pointer = start_address as *mut Node;
        unsafe {
            node_pointer.write(node);
            self.head_node.next_node = Some(&mut *node_pointer);
        }
    }

    fn adjust_layout(layout: Layout) -> (usize, usize) {
        let layout = layout.align_to(align_of::<Node>()).expect("Failed to adjust alignment").pad_to_align();
        let size = layout.size().max(size_of::<Node>());
        (layout.align(), size)
    }

    fn allocate_region(alignment: usize, region: &Node, size: usize) -> Result<usize, ()> {
        let start_address = align_up(region.start_address(), alignment);
        let end_address = start_address.checked_add(size).ok_or(())?;
        if end_address > region.end_address() {
            return Err(());
        }

        let excess_size = region.end_address() - end_address;
        if excess_size > 0 && excess_size < size_of::<Node>() {
            return Err(());
        }

        Ok(start_address)
    }

    fn find_region(&mut self, alignment: usize, size: usize) -> Option<(&'static mut Node, usize)> {
        let mut current_node = &mut self.head_node;
        while let Some(ref mut region) = current_node.next_node {
            if let Ok(start_address) = Self::allocate_region(alignment, &region, size) {
                let next_node = region.next_node.take();
                let return_value = Some((current_node.next_node.take().unwrap(), start_address));
                current_node.next_node = next_node;
                return return_value;
            } else {
                current_node = current_node.next_node.as_mut().unwrap();
            }
        }

        None
    }

    pub unsafe fn initialize(&mut self, heap_size: usize, heap_start_address: usize) -> () {
        unsafe { self.add_free_region(heap_size, heap_start_address) };
    }

    pub const fn new() -> Self {
        Self {
            head_node: Node::new(0),
        }
    }
}

unsafe impl GlobalAlloc for Locked<LinkedListAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let (alignment, size) = LinkedListAllocator::adjust_layout(layout);
        let mut allocator = self.lock();

        if let Some((region, start_address)) = allocator.find_region(alignment, size) {
            let end_address = start_address.checked_add(size).expect("Allocation overflows assigned region");
            let excess_size = region.end_address() - end_address;
            if excess_size > 0 {
                unsafe { allocator.add_free_region(excess_size, end_address) };
            }

            start_address as *mut u8
        } else {
            null_mut()
        }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        let (_, size) = LinkedListAllocator::adjust_layout(layout);
        unsafe { self.lock().add_free_region(size, pointer as usize) };
    }
}

struct Node {
    next_node: Option<&'static mut Node>,
    size: usize,
}

impl Node {
    fn end_address(&self) -> usize {
        self.start_address() + self.size
    }

    const fn new(size: usize) -> Self {
        Self {
            next_node: None,
            size,
        }
    }

    fn start_address(&self) -> usize {
        self as *const Self as usize
    }
}
