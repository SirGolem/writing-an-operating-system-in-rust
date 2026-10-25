#![feature(custom_test_frameworks)]
#![no_main]
#![no_std]
#![reexport_test_harness_main = "test_main"]
#![test_runner(operating_system::test_runner)]

use core::panic::PanicInfo;
use operating_system::print_line;
#[cfg(test)]
use operating_system::test_panic_handler;

#[cfg(not(test))]
#[panic_handler]
fn panic(information: &PanicInfo) -> ! {
    print_line!("{}", information);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(information: &PanicInfo) -> ! {
    test_panic_handler(information);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    print_line!("Hello, world!");

    #[cfg(test)]
    test_main();

    loop {}
}
