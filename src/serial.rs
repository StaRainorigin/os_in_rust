//  定义了用于与串行端口进行交互的函数和宏


use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;



//  创建一个静态的SerialPort实例
//  使用lazy_static crate来实现延迟初始化
//  一个指向SerialPort实例的全局静态引用。它被包装在一个自旋锁中。

//  由于Rust的语法特性，语法块中最后一行如果不加 ';' 符号，会被当作返回值处理
//  这里是在对SERIAL1进行初始化的时候，语法块中的 init() 被执行一次， 然后再把 Mutex::new(serial_port) 的返回值当作值赋给SERIAL1
//  个人非常喜欢这种语法😊
lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}




//  格式化print宏，实现方式跟vga相似

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    });
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}