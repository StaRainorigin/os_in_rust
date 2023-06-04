//定义了用于与VGA文本缓冲区进行交互的函数和类型


use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
    /// 一个全局的`Writer`实例，可用于打印到VGA文本缓冲区。
    ///
    /// 被`print!`和`println!`宏所使用。
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}



/// VGA文本模式下的标准调色模式
#[allow(dead_code)] ///这个禁用了编译器的警告
#[derive(Debug, Clone, Copy, PartialEq, Eq)]    
/// 为下面的结构体实现了Debug（带有复杂数据类型的格式化输出操作
/// 克隆（复杂数据类型的复制，也有其他语言中的深复制的作用）
/// 复制（简单数据类型的复制，可以按位复制）
/// 判断（PartialEq，和Eq都是比较相等性的 trait）
/// 这意味着它可以使用 {:?} 格式说明符进行打印，可以按位复制，可以使用 == 和 != 运算符进行比较。
#[repr(u8)] ///#[repr(u8)] 表示枚举 `Color` 的每个变量都用一个 `u8` 类型的值来表示。
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}
/// 因为VGA Text中用一个字节（8位）表示颜色，其中半个八位（4位）分别表示前景色（字符颜色）和背景色，
/// 那么可以分给颜色的位就是4位，意味着可以容纳16个数，给这16个数用枚举分别分配好颜色

/// 定义一个结构体表示 一个前景色 和 一个背景色 的组合
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    /// 类似于构造函数（方法），
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// VGA文本缓冲区中的一个屏幕字符，由一个ASCII字符和一个`ColorCode'组成。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

/// 文本缓冲区的高度（通常为25行）。
const BUFFER_HEIGHT: usize = 25;
/// 文本缓冲区的宽度（通常为80列）。
const BUFFER_WIDTH: usize = 80;

/// 一个代表VGA文本缓冲区的结构。
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// 一个写入器类型，允许将ASCII字节和字符串写入底层的`缓冲区'。
///
/// 以`BUFFER_WIDTH`包裹行。支持换行符，并实现了`core::fmt::Write` 特征。
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    /// 写一个ASCII字节到缓冲区。
    ///
    /// 以`BUFFER_WIDTH'包裹行。支持`\n'换行符。
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    /// 将给定的ASCII字符串写到缓冲区。
    ///
    /// 以`BUFFER_WIDTH`的方式包行。支持`n'换行符。不***
    /// 支持非ASCII字符的字符串，因为它们不能在VGA文本中打印。
    /// 模式下打印。
    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// 将所有行向上移动一行，并清除最后一行。
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// 通过用空白字符覆盖来清除某一行。
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}




/// 就像标准库中的`print!`宏一样，但打印到VGA文本缓冲区。
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
/// 就像标准库中的`println!`宏一样，但打印到VGA文本缓冲区。
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// 通过全局的`WRITER`实例将给定的格式化字符串打印到VGA文本缓冲区。
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}





#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}