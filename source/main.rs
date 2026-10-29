#![feature(custom_test_frameworks)]
#![no_main]
#![no_std]
#![reexport_test_harness_main = "test_main"]
#![test_runner(operating_system::test_runner)]

use core::panic::PanicInfo;
#[cfg(test)]
use operating_system::test_panic_handler;
use operating_system::{halt_loop, initialize, print_line};

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

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    print_line!("Hello, world!");

    initialize();

    #[cfg(test)]
    test_main();

    halt_loop();
}
