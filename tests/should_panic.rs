//  定义了一个简单的集成测试，用于测试内核是否能够正确处理panic

#![no_std]
#![no_main]

use os_in_rust::{exit_qemu, serial_print, serial_println, QemuExitCode};
use core::panic::PanicInfo;


//  内核的入口点
//  调用should_fail函数，该函数应该触发panic
//  如果没有触发panic，则打印错误消息并退出QEMU
#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}


//  一个简单的测试用例，它使用断言来触发panic
fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}


//  内核的panic处理程序。它打印成功消息并退出QEMU
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}