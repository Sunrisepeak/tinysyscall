#[inline]
pub fn syscall(id: SyscallTable, args: [usize; 6]) -> isize {
    let mut ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") id as isize => ret,
            in("rdi") args[0],
            in("rsi") args[1],
            in("rdx") args[2],
            in("r10") args[3],
            in("r8")  args[4],
            in("r9")  args[5],
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
    }
    ret
}

/// x86_64 linux 系统调用表/号
/// refs: https://x64.syscall.sh/
#[cfg(any(target_os = "linux", target_os = "none"))]
pub enum SyscallTable {
    // file
    OPEN = 2,
    READ = 0,
    WRITE = 1,
    IOCTL = 16,
    CLOSE = 3,

    EXIT = 60,
    NANOSLEEP = 35,

    STAT = 4,
    GETDENTS = 78,
}