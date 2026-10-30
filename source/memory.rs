use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    PhysAddr, VirtAddr,
    registers::control::Cr3,
    structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB},
};

unsafe fn active_level_four_page_table(physical_offset: VirtAddr) -> &'static mut PageTable {
    let (level_four_page_table_frame, _) = Cr3::read();
    let physical_address = level_four_page_table_frame.start_address();
    let virtual_address = physical_offset + physical_address.as_u64();
    let page_table_pointer: *mut PageTable = virtual_address.as_mut_ptr();
    unsafe { &mut *page_table_pointer }
}

pub struct BootInformationFrameAllocator {
    memory_map: &'static MemoryMap,
    next_frame: usize,
}

impl BootInformationFrameAllocator {
    pub unsafe fn initialize(memory_map: &'static MemoryMap) -> Self {
        Self {
            memory_map,
            next_frame: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|region| region.region_type == MemoryRegionType::Usable);
        let usable_address_ranges = usable_regions.map(|region| region.range.start_addr()..region.range.end_addr());
        let usable_frame_addresses = usable_address_ranges.flat_map(|range| range.step_by(4096));
        usable_frame_addresses.map(|address| PhysFrame::containing_address(PhysAddr::new(address)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInformationFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frames().nth(self.next_frame);
        self.next_frame += 1;
        frame
    }
}

pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        None
    }
}

pub unsafe fn initialize(physical_offset: VirtAddr) -> OffsetPageTable<'static> {
    unsafe {
        let level_four_page_table = active_level_four_page_table(physical_offset);
        OffsetPageTable::new(level_four_page_table, physical_offset)
    }
}
