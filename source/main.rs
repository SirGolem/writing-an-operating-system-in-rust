#![feature(custom_test_frameworks)]
#![no_main]
#![no_std]
#![reexport_test_harness_main = "test_main"]
#![test_runner(operating_system::test_runner)]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
#[cfg(test)]
use operating_system::test_panic_handler;
use operating_system::{halt_loop, initialize, print_line};

entry_point!(main);
fn main(_boot_information: &'static BootInfo) -> ! {
    print_line!("Hello, world!");

    initialize();

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
