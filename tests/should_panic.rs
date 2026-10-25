#![no_main]
#![no_std]

use core::panic::PanicInfo;
use operating_system::{EmulatorExitCode, exit_emulator, serial_print_line};

#[panic_handler]
fn panic(_information: &PanicInfo) -> ! {
    serial_print_line!(": Pass");
    exit_emulator(EmulatorExitCode::Success);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    tests::should_panic();
    serial_print_line!(": Fail");
    exit_emulator(EmulatorExitCode::Failure);
    loop {}
}

mod tests {
    use operating_system::serial_print;

    pub fn should_panic() -> () {
        serial_print!("tests::should_panic");
        assert_eq!(0, 1);
    }
}
