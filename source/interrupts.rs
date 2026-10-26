use crate::{global_descriptor_table::DOUBLE_FAULT_INTERRUPT_STACK_TABLE_INDEX, print_line};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref INTERRUPT_DESCRIPTOR_TABLE: InterruptDescriptorTable = {
        let mut interrupt_descriptor_table = InterruptDescriptorTable::new();
        interrupt_descriptor_table.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe {
            interrupt_descriptor_table
                .double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_INTERRUPT_STACK_TABLE_INDEX);
        }

        interrupt_descriptor_table
    };
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) -> () {
    print_line!("Exception: Breakpoint reached.");
    print_line!("{stack_frame:#?}");
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("Exception: Double fault.\n{stack_frame:#?}");
}

pub fn initialize() -> () {
    INTERRUPT_DESCRIPTOR_TABLE.load();
}

#[cfg(test)]
mod tests {
    use x86_64::instructions::interrupts::int3;

    #[test_case]
    fn breakpoint_exception() -> () {
        int3();
    }
}
