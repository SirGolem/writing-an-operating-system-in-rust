#![feature(custom_test_frameworks)]
#![no_main]
#![no_std]
#![reexport_test_harness_main = "test_main"]
#![test_runner(operating_system::test_runner)]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
#[cfg(test)]
use operating_system::test_panic_handler;
use operating_system::{
    allocator::initialize_heap,
    halt_loop, initialize,
    memory::{self, BootInformationFrameAllocator},
    print_line,
};
use x86_64::VirtAddr;

entry_point!(main);
fn main(boot_information: &'static BootInfo) -> ! {
    print_line!("Hello, world!");

    initialize();

    let mut frame_allocator = unsafe { BootInformationFrameAllocator::initialize(&boot_information.memory_map) };
    let physical_memory_offset = VirtAddr::new(boot_information.physical_memory_offset);
    let mut memory_mapper = unsafe { memory::initialize(physical_memory_offset) };
    initialize_heap(&mut frame_allocator, &mut memory_mapper).expect("Failed to initialize heap");

    let heap_value = Box::new(0);
    print_line!("heap_value: {heap_value:p}");

    let mut vector = Vec::new();
    for index in 0..500 {
        vector.push(index);
    }
    print_line!("vector: {:p}", vector.as_slice());

    let reference_counted_vector = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted_vector.clone();
    print_line!("Vector Reference Count: {}", Rc::strong_count(&cloned_reference));
    drop(reference_counted_vector);
    print_line!("Vector Reference Count: {}", Rc::strong_count(&cloned_reference));

    #[cfg(test)]
    test_main();

    halt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(information: &PanicInfo) -> ! {
    print_line!("{}", information);
    halt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(information: &PanicInfo) -> ! {
    test_panic_handler(information);
}
