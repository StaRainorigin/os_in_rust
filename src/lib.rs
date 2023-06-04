// 定义了一些用于测试的公共函数和类型


#![no_std]      //指示编译器不链接Rust标准库，因为标准库依赖于操作系统，但是本项目要编写一个操作系统内核
#![cfg_attr(test, no_main)]     //禁用main函数的检查，因为我们将使用自定义的测试入口点
#![feature(custom_test_frameworks)]     //启用了自定义测试框架的使用
#![test_runner(crate::test_runner)]     //指定了自定义测试运行器函数
#![reexport_test_harness_main = "test_main"]    //将生成的测试入口点重命名为test_main
#![feature(abi_x86_interrupt)]



use core::panic::PanicInfo;

pub mod serial;
pub mod vga_buffer;
//  包含用于与串行端口和VGA缓冲区进行交互的代码的模块
pub mod interrupts;
pub mod gdt;
//  中断相关模块




//  interrupts相关的
pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();     //启用中断
}






//  这个trait定义了一个run方法，用于运行测试它为所有实现了Fn() trait的类型提供了一个默认实现，该实现打印测试名称并运行测试函数
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}




//  接受一个测试切片，并运行其中的每个测试
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}



//  这个函数在测试失败时被调用它打印错误信息并退出QEMU
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}



//  定义了两种退出代码，分别表示成功和失败
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}



//  使用x86指令向I/O端口0xf4写入退出代码，以便退出QEMU
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}


/// 自定义的节能的无限循环
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}





/// 自定义测试的入口点它调用test_main函数并进入无限循环
/// cargo test --lib 的入口
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}