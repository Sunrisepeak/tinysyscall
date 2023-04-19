
use crate::arch::x86_64;
use crate::interface::{ISAL, Timespec};


/// x86_64 linux 系统调用表/号
/// refs: https://x64.syscall.sh/
const SYSCALL_WRITE: usize = 1;
const SYSCALL_NANOSLEEP: usize = 35;
const SYSCALL_EXIT: usize = 60;

/* 系统调用的函数封装 */

pub struct SAL;

impl ISAL for SAL {
    fn sys_exit(xstate: i32) -> isize {
        x86_64::syscall(SYSCALL_EXIT, [xstate as usize, 0, 0, 0, 0, 0])
    }

    fn sys_write(fd: usize, buffer: &[u8]) -> isize {
        // 输入参数数组: [文件描述符, 字符串首地址, 字符串长度]
        x86_64::syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len(), 0, 0, 0])
    }

    fn sys_nanosleep(req: Timespec, rem: Timespec) -> isize {
        let req_ptr = &req as *const Timespec;
        let rem_ptr = &rem as *const Timespec;
        x86_64::syscall(SYSCALL_NANOSLEEP, [req_ptr as usize, rem_ptr as usize, 0, 0, 0, 0])
    }
}