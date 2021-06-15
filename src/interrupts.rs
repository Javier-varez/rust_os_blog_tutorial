use crate::serial_println;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::gdt;
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_isr);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_isr)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_isr(stack_frame: InterruptStackFrame) {
    serial_println!("Breakpoint ISR: {:?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_isr(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    panic!("Double fault ISR {}: {:?}", error_code, stack_frame);
}

#[test_case]
pub fn test_irq_is_handled() {
    x86_64::instructions::interrupts::int3();
}
