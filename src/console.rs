use core::fmt;
use crate::os::syscall;

struct Stdout;

/// 为Stdout 实现core::fmt::Write trait 的 write_str 函数
impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // 使用系统调用, fd 1 为标准输出
        use crate::interface::isal::File;
        syscall::sys_write(1, s.as_bytes());
        Ok(())
    }
}

// 封装一个接受 fmt::Arguments 类型的 函数, 用于println！ 宏
pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    Stdout.write_fmt(args).unwrap();
}