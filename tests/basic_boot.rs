#![feature(custom_test_frameworks)]
#![no_main]
#![no_std]
#![reexport_test_harness_main = "test_main"]
#![test_runner(operating_system::test_runner)]

use core::panic::PanicInfo;
use operating_system::test_panic_handler;

#[panic_handler]
fn panic(information: &PanicInfo) -> ! {
    test_panic_handler(information);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

mod tests {
    use operating_system::print_line;

    #[test_case]
    fn print_line() -> () {
        print_line!("tests::print_line");
    }
}
