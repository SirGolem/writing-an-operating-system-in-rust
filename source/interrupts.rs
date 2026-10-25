use crate::print_line;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref INTERRUPT_DESCRIPTOR_TABLE: InterruptDescriptorTable = {
        let mut interrupt_descriptor_table = InterruptDescriptorTable::new();
        interrupt_descriptor_table.breakpoint.set_handler_fn(breakpoint_handler);
        interrupt_descriptor_table
    };
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) -> () {
    print_line!("Exception: Breakpoint reached.");
    print_line!("{stack_frame:#?}");
}

pub fn initialize_interrupt_descriptor_table() -> () {
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
