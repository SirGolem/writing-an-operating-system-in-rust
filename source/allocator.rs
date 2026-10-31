use linked_list_allocator::LockedHeap;
use x86_64::{
    VirtAddr,
    structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB, mapper::MapToError},
};

pub const HEAP_SIZE: usize = 100 * 1024;
pub const HEAP_START_ADDRESS: usize = 0x4444_4444_0000;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

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

    unsafe { ALLOCATOR.lock().init(HEAP_START_ADDRESS, HEAP_SIZE) };

    Ok(())
}
