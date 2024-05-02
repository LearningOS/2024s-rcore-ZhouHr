use crate::sbi::console_putchar;
use core::fmt::{self, Write};


struct Stdout;

impl Write for Stdout {
    // 实现write_str接口函数
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // 直接使用asm实现
		// sys_write(1, s.as_bytes());
		// 直接使用asm实现

        // 使用sbi-rt库实现
        for c in s.chars() {
            console_putchar(c as usize);
        }
        // 使用sbi-rt库实现
        Ok(())
    }
}

// 实现打印函数
pub fn _print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}


// 定义输出宏
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        crate::console::_print(format_args!($fmt $(, $($arg)+)?));
    }
}

// 定义输出宏
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        crate::console::_print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
