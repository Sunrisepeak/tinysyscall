#[cfg(target_arch = "riscv64")]
use crate::arch::riscv::{syscall, SyscallTable};

#[cfg(target_arch = "x86_64")]
use crate::arch::x86_64::{syscall, SyscallTable};

use crate::interface::{ISAL, TimeSpec, OpenFlags};

/// [type def]: https://faculty.nps.edu/cseagle/assembly/sys_call.html
/// [error code]: https://github.com/torvalds/linux/blob/master/include/uapi/asm-generic/errno-base.h

pub struct SAL;

impl ISAL for SAL {

    fn sys_open(path_ptr: usize, flags: OpenFlags) -> isize {
        syscall(
            SyscallTable::OPEN, 
            [
                path_ptr,
                flags.bits(), 
                0, 0, 0, 0
            ]
        )
    }

    fn sys_read(fd: usize, buffer: &mut [u8], len: usize) -> isize {
        syscall(
            SyscallTable::READ,
            [
                fd,
                buffer.as_ptr() as usize,
                len,
                0, 0, 0
            ]
        )
    }

    fn sys_write(fd: usize, buffer: &[u8]) -> isize {
        // 输入参数数组: [文件描述符, 字符串首地址, 字符串长度]
        syscall(
            SyscallTable::WRITE, 
            [
                fd,
                buffer.as_ptr() as usize,
                buffer.len(),
                 0, 0, 0
            ]
        )
    }

    fn sys_ioctl(fd: usize, cmd: usize, arg: usize) -> isize {
        syscall(
            SyscallTable::IOCTL, 
            [
                fd,
                cmd,
                arg,
                0, 0, 0
            ]
        )
    }

    fn sys_close(fd: usize) -> isize {
        syscall(
            SyscallTable::CLOSE, 
            [
                fd,
                0, 0, 0, 0, 0
            ]
        )
    }

    fn sys_exit(xstate: i32) -> isize {
        syscall(
            SyscallTable::EXIT,
            [
                xstate as usize,
                0, 0, 0, 0, 0
            ]
        )
    }

    fn sys_nanosleep(req: TimeSpec, _rem: TimeSpec) -> isize {
        let req_ptr = &req as *const TimeSpec;
        syscall(
            SyscallTable::NANOSLEEP,
            [
                req_ptr as usize,
                0, 0, 0, 0, 0
            ]
        )
    }
}