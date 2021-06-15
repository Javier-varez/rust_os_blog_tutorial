#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::{exit_qemu, serial_println, QemuExitCode};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() {
    blog_os::init();
    test_double_fault_irq_is_handled();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{:?}", info);
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn test_double_fault_irq_is_handled() {
    // This should cause a page fault, which would be escalated to a double fault
    unsafe {
        let ptr = 0xdeadbeef as *mut u64;
        *ptr = 0;
    }
}
