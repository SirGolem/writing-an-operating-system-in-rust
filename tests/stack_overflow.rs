#![feature(abi_x86_interrupt)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use operating_system::{
    EmulatorExitCode, exit_emulator,
    global_descriptor_table::{self, DOUBLE_FAULT_INTERRUPT_STACK_TABLE_INDEX},
    serial_print_line, test_panic_handler,
};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref TEST_INTERRUPT_DESCRIPTOR_TABLE: InterruptDescriptorTable = {
        let mut interrupt_descriptor_table = InterruptDescriptorTable::new();

        unsafe {
            interrupt_descriptor_table
                .double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_INTERRUPT_STACK_TABLE_INDEX);
        }

        interrupt_descriptor_table
    };
}

fn initialize_test_interrupt_descriptor_table() -> () {
    TEST_INTERRUPT_DESCRIPTOR_TABLE.load();
}

#[panic_handler]
fn panic(information: &PanicInfo) -> ! {
    test_panic_handler(information);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    global_descriptor_table::initialize();
    initialize_test_interrupt_descriptor_table();

    tests::stack_overflow();
    panic!("Execution continued after stack overflow.");
}

extern "x86-interrupt" fn test_double_fault_handler(_stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    serial_print_line!(": Pass");
    exit_emulator(EmulatorExitCode::Success);
    loop {}
}

mod tests {
    use operating_system::serial_print;
    use volatile::Volatile;

    pub fn stack_overflow() -> () {
        serial_print!("tests::stack_overflow");

        #[allow(unconditional_recursion)]
        fn cause_stack_overflow() -> () {
            cause_stack_overflow();
            Volatile::new(0).read();
        }

        cause_stack_overflow();
    }
}
