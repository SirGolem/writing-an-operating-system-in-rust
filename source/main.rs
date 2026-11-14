#![feature(custom_test_frameworks)]
#![no_main]
#![no_std]
#![reexport_test_harness_main = "test_main"]
#![test_runner(operating_system::test_runner)]

extern crate alloc;

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
#[cfg(test)]
use operating_system::test_panic_handler;
use operating_system::{
    allocator::initialize_heap,
    halt_loop, initialize,
    memory::{self, BootInformationFrameAllocator},
    print_line,
    task::{Task, executor::Executor, keyboard::print_key_presses},
};
use x86_64::VirtAddr;

async fn asynchronous_number() -> u32 {
    64
}

async fn asynchronous_number_task() -> () {
    let number = asynchronous_number().await;
    print_line!("Asynchronous Number: {number}");
}

entry_point!(main);
fn main(boot_information: &'static BootInfo) -> ! {
    print_line!("Hello, world!");

    initialize();

    let mut frame_allocator = unsafe { BootInformationFrameAllocator::initialize(&boot_information.memory_map) };
    let physical_memory_offset = VirtAddr::new(boot_information.physical_memory_offset);
    let mut memory_mapper = unsafe { memory::initialize(physical_memory_offset) };
    initialize_heap(&mut frame_allocator, &mut memory_mapper).expect("Failed to initialize heap");

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(asynchronous_number_task()));
    executor.spawn(Task::new(print_key_presses()));
    executor.run();
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
