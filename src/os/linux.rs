#[cfg(target_arch = "riscv64")]
use crate::arch::riscv::{syscall, SyscallTable};

#[cfg(target_arch = "x86_64")]
use crate::arch::x86_64::{syscall, SyscallTable};

use crate::interface::{ISAL, Timespec};

pub struct SAL;

impl ISAL for SAL {
    fn sys_exit(xstate: i32) -> isize {
        syscall(SyscallTable::EXIT, [xstate as usize, 0, 0, 0, 0, 0])
    }

    fn sys_write(fd: usize, buffer: &[u8]) -> isize {
        // 输入参数数组: [文件描述符, 字符串首地址, 字符串长度]
        syscall(SyscallTable::WRITE, [fd, buffer.as_ptr() as usize, buffer.len(), 0, 0, 0])
    }

    fn sys_nanosleep(req: Timespec, rem: Timespec) -> isize {
        let req_ptr = &req as *const Timespec;
        let rem_ptr = &rem as *const Timespec;
        syscall(SyscallTable::NANOSLEEP, [req_ptr as usize, rem_ptr as usize, 0, 0, 0, 0])
    }
}