use crate::allocator::Locked;
use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::{NonNull, null_mut},
};
use linked_list_allocator::Heap;

const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

fn block_size_index(layout: &Layout) -> Option<usize> {
    let size = layout.size().max(layout.align());
    BLOCK_SIZES.iter().position(|&block_size| block_size >= size)
}

pub struct FixedSizeBlockAllocator {
    fallback_allocator: Heap,
    head_nodes: [Option<&'static mut Node>; BLOCK_SIZES.len()],
}

impl FixedSizeBlockAllocator {
    fn allocate_fallback(&mut self, layout: Layout) -> *mut u8 {
        match self.fallback_allocator.allocate_first_fit(layout) {
            | Err(_) => null_mut(),
            | Ok(pointer) => pointer.as_ptr(),
        }
    }

    pub unsafe fn initialize(&mut self, heap_size: usize, heap_start_address: usize) -> () {
        unsafe { self.fallback_allocator.init(heap_start_address, heap_size) };
    }

    pub const fn new() -> Self {
        const EMPTY_NODE: Option<&'static mut Node> = None;
        Self {
            fallback_allocator: Heap::empty(),
            head_nodes: [EMPTY_NODE; BLOCK_SIZES.len()],
        }
    }
}

unsafe impl GlobalAlloc for Locked<FixedSizeBlockAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.lock();

        match block_size_index(&layout) {
            | None => allocator.allocate_fallback(layout),
            | Some(index) => match allocator.head_nodes[index].take() {
                | None => {
                    let block_size = BLOCK_SIZES[index];
                    let layout = Layout::from_size_align(block_size, block_size).unwrap();
                    allocator.allocate_fallback(layout)
                }
                | Some(node) => {
                    allocator.head_nodes[index] = node.next_node.take();
                    node as *mut Node as *mut u8
                }
            },
        }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        let mut allocator = self.lock();

        match block_size_index(&layout) {
            | None => {
                let pointer = NonNull::new(pointer).unwrap();
                unsafe { allocator.fallback_allocator.deallocate(pointer, layout) };
            }
            | Some(index) => {
                let new_node = Node {
                    next_node: allocator.head_nodes[index].take(),
                };

                assert!(align_of::<Node>() <= BLOCK_SIZES[index]);
                assert!(size_of::<Node>() <= BLOCK_SIZES[index]);

                let new_node_pointer = pointer as *mut Node;
                unsafe {
                    new_node_pointer.write(new_node);
                    allocator.head_nodes[index] = Some(&mut *new_node_pointer);
                }
            }
        }
    }
}

struct Node {
    next_node: Option<&'static mut Node>,
}
