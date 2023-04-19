

#[inline]
pub fn syscall(sys_no: usize, args: [usize; 6]) -> isize {
    let mut ret: isize;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") sys_no as isize => ret,
            in("rdi") args[0],
            in("rsi") args[1],
            in("rdx") args[2],
            in("r10") args[3],
            in("r8")  args[4],
            in("r9")  args[5],
            out("rcx") _, // rcx is used to store old rip
            out("r11") _, // r11 is used to store old rflags
            options(nostack, preserves_flags)
        );
    }
    ret
}