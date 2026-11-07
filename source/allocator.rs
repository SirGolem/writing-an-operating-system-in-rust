pub mod bump;
pub mod fixed_size_block;
pub mod linked_list;

use crate::allocator::fixed_size_block::FixedSizeBlockAllocator;
use spin::{Mutex, MutexGuard};
use x86_64::{
    VirtAddr,
    structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB, mapper::MapToError},
};

pub const HEAP_SIZE: usize = 100 * 1024;
pub const HEAP_START_ADDRESS: usize = 0x4444_4444_0000;

#[global_allocator]
static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());

fn align_up(address: usize, alignment: usize) -> usize {
    (address + alignment - 1) & !(alignment - 1)
}

pub fn initialize_heap(
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    mapper: &mut impl Mapper<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start_address = VirtAddr::new(HEAP_START_ADDRESS as u64);
        let heap_end_address = heap_start_address + HEAP_SIZE as u64 - 1;
        let heap_end_page = Page::containing_address(heap_end_address);
        let heap_start_page = Page::containing_address(heap_start_address);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        let frame = frame_allocator.allocate_frame().ok_or(MapToError::FrameAllocationFailed)?;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    unsafe { ALLOCATOR.lock().initialize(HEAP_SIZE, HEAP_START_ADDRESS) };

    Ok(())
}

struct Locked<Value> {
    value: Mutex<Value>,
}

impl<Value> Locked<Value> {
    pub fn lock(&self) -> MutexGuard<'_, Value> {
        self.value.lock()
    }

    pub const fn new(value: Value) -> Self {
        Self {
            value: Mutex::new(value),
        }
    }
}
