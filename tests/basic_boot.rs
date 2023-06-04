//  定义了一个简单的集成测试，用于测试内核是否能够成功启动并打印文本
//  结构有点像main.rs


#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_in_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

use os_in_rust::println;
use core::panic::PanicInfo;

#[no_mangle] // 同main.rs中一样，这里也是让它不修改名字，因为要作为测试的入口点
pub extern "C" fn _start() -> ! {

    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_in_rust::test_panic_handler(info)
}


//  一个简单的测试用例，它打印一行文本
#[test_case]
fn test_println() {
    println!("test_println output");
}