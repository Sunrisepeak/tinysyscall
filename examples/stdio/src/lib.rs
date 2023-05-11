#![no_std]

extern crate sal;

use core::fmt;

struct Stdout;

/// 为Stdout 实现core::fmt::Write trait 的 write_str 函数
impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // 使用系统调用, fd 1 为标准输出
        sal::file::write(sal::file::STDOUT, s.as_bytes());
        Ok(())
    }
}

// 封装一个接受 fmt::Arguments 类型的 函数, 用于println！ 宏
pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    Stdout.write_fmt(args).unwrap();
}

// println! 和 print! 
// 宏标准库实现: https://doc.rust-lang.org/nightly/src/std/macros.rs.html#132-139
//#[macro_export]
#[macro_export(local_inner_macros)]
macro_rules! print {
    ($($arg:tt)*) => {{
        // 调用上面的print函数
        $crate::print(core::format_args!($($arg)*));
    }};
}

//#[macro_export]
#[macro_export(local_inner_macros)]
macro_rules! println {
    // 分支1: 当无参数时
    () => {
        $crate::print!("\n")
    };
    // 分支2
    ($($arg:tt)*) => {{
        $crate::print!("{}\n", core::format_args!($($arg)*));
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn hello_world() {
        super::println!("Hello World!");
    }
}