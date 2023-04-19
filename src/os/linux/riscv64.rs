
use crate::arch::riscv;
use crate::interface::{ISAL, Timespec};


/// riscv linux 系统调用表/号
/// refs: https://jborza.com/post/2021-05-11-riscv-linux-syscalls
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_NANOSLEEP: usize = 101;


/* 系统调用的函数封装 */

pub struct SAL;

impl ISAL for SAL {
    fn sys_exit(xstate: i32) -> isize {
        riscv::syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
    }

    fn sys_write(fd: usize, buffer: &[u8]) -> isize {
        // 输入参数数组: [文件描述符, 字符串首地址, 字符串长度]
        riscv::syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
    }

    fn sys_nanosleep(req: Timespec, rem: Timespec) -> isize {
        let req_ptr = &req as *const Timespec;
        let rem_ptr = &rem as *const Timespec;
        riscv::syscall(SYSCALL_NANOSLEEP, [req_ptr as usize, rem_ptr as usize, 0])
    }
}