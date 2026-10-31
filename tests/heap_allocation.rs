#![feature(custom_test_frameworks)]
#![no_main]
#![no_std]
#![reexport_test_harness_main = "test_main"]
#![test_runner(operating_system::test_runner)]

extern crate alloc;

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use operating_system::{
    allocator::initialize_heap,
    initialize,
    memory::{self, BootInformationFrameAllocator},
    test_panic_handler,
};
use x86_64::VirtAddr;

entry_point!(main);
fn main(boot_information: &'static BootInfo) -> ! {
    initialize();
    let mut frame_allocator = unsafe { BootInformationFrameAllocator::initialize(&boot_information.memory_map) };
    let physical_memory_offset = VirtAddr::new(boot_information.physical_memory_offset);
    let mut memory_mapper = unsafe { memory::initialize(physical_memory_offset) };
    initialize_heap(&mut frame_allocator, &mut memory_mapper).expect("Failed to initialize heap");

    test_main();
    loop {}
}

#[panic_handler]
fn panic(information: &PanicInfo) -> ! {
    test_panic_handler(information);
}

mod tests {
    use alloc::{boxed::Box, vec::Vec};
    use operating_system::allocator::HEAP_SIZE;

    #[test_case]
    fn large_vector() -> () {
        let mut vector = Vec::new();
        let vector_size = 1_000;

        for index in 0..vector_size {
            vector.push(index);
        }

        assert_eq!(vector.iter().sum::<u64>(), vector_size / 2 * (vector_size - 1))
    }

    #[test_case]
    fn many_boxes() -> () {
        for index in 0..HEAP_SIZE {
            let value = Box::new(index);
            assert_eq!(*value, index);
        }
    }

    #[test_case]
    fn simple_allocations() -> () {
        let heap_value_0 = Box::new(0);
        let heap_value_1 = Box::new(1);

        assert_eq!(*heap_value_0, 0);
        assert_eq!(*heap_value_1, 1);
    }
}
