#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![no_std]
#![reexport_test_harness_main = "test_main"]
#![test_runner(crate::test_runner)]

pub mod global_descriptor_table;
pub mod interrupts;
pub mod serial;
pub mod video_graphics_array_buffer;

use crate::interrupts::PROGRAMMABLE_INTERRUPT_CONTROLLERS;
use core::{any::type_name, panic::PanicInfo};
use x86_64::instructions::{self, hlt, port::Port};

#[repr(u32)]
pub enum EmulatorExitCode {
    Failure = 0x11,
    Success = 0x10,
}

pub fn exit_emulator(exit_code: EmulatorExitCode) -> () {
    unsafe {
        let mut port = Port::new(0xF4);
        port.write(exit_code as u32);
    };
}

pub fn halt_loop() -> ! {
    loop {
        hlt();
    }
}

pub fn initialize() -> () {
    global_descriptor_table::initialize();
    interrupts::initialize();
    unsafe { PROGRAMMABLE_INTERRUPT_CONTROLLERS.lock().initialize() };
    instructions::interrupts::enable();
}

#[cfg(test)]
#[panic_handler]
fn panic(information: &PanicInfo) -> ! {
    test_panic_handler(information);
}

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    initialize();
    test_main();
    halt_loop();
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<Type> Testable for Type
where Type: Fn()
{
    fn run(&self) -> () {
        serial_print!("{}", type_name::<Type>());
        self();
        serial_print_line!(": Pass");
    }
}

pub fn test_panic_handler(information: &PanicInfo) -> ! {
    serial_print_line!(": Fail");
    serial_print_line!("Error: {}", information);
    exit_emulator(EmulatorExitCode::Failure);
    halt_loop();
}

pub fn test_runner(tests: &[&dyn Testable]) -> () {
    serial_print_line!("Running {} tests...", tests.len());

    for test in tests {
        test.run();
    }

    exit_emulator(EmulatorExitCode::Success);
}
