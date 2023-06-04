//  定义了内核的入口点和panic处理程序


#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_in_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

use os_in_rust::println;
use core::panic::PanicInfo;



//  内核的入口点它打印“Hello World！”并在测试配置下调用test_main函数最后，进入无限循环
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");

    os_in_rust::init();

    // 调用一个中断点异常
    // x86_64::instructions::interrupts::int3();


    // 触发一个双重异常
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // };


    // 触发栈溢出
    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();


    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    os_in_rust::hlt_loop();
}




// 内核的panic处理程序
// 在非测试配置下，打印panic信息并进入无限循环
// 在测试配置下，调用os_in_rust::test_panic_handler函数
/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    os_in_rust::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_in_rust::test_panic_handler(info)
}



//  一个简单的测试用例
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}