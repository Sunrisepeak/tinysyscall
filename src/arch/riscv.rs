
// Rust 内联汇编： https://doc.rust-lang.org/stable/rust-by-example/unsafe/asm.html?highlight=assem#inline-assembly

/// 系统调用入口函数
///     id: 系统调用号
///     args: 系统调用参数, [usize; 3]的数组
/// 
/// RISCV 函数调用 寄存器 规约：
///     x10, x11  (a0-1) 函数参数/返回值
///     x12 ~ x17 (a2-7) 函数参数
/// inlateout： rust 内敛汇编 为了节省寄存器的关键字, 让其先作为输入寄存器 然后再作为输入寄存器
#[inline]
pub fn syscall(id: SyscallTable, args: [usize; 6]) -> isize {
    let mut ret;
    unsafe {
        core::arch::asm!(
            "ecall", // 系统调用 指令
            inlateout("x10") args[0] => ret, // a0/x10 先作为输入参数0 再作为返回值 -> ret
            in("x11") args[1], // a1 寄存器 输入参数1
            in("x12") args[2], // a2/x11 寄存器 输入参数2
            in("x17") id as isize,      // a7/x17 寄存器 输入 系统调用号
        );
    }
    ret
}

/// riscv linux 系统调用表/号
/// refs: https://jborza.com/post/2021-05-11-riscv-linux-syscalls
#[cfg(any(target_os = "linux", target_os = "none"))]
pub enum SyscallTable {
    // file
    OPEN = 56,
    READ = 63,
    WRITE = 64,
    IOCTL = 29,
    CLOSE = 57,
    
    EXIT = 93,
    NANOSLEEP = 101,

    STAT = 80, // newfstat
    GETDENTS = 61, // getdents64

    MMAP = 222,
    MUNMAP = 215,

}