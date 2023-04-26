#[cfg(target_arch = "riscv64")]
use crate::arch::riscv::{syscall, SyscallTable};

#[cfg(target_arch = "x86_64")]
use crate::arch::x86_64::{syscall, SyscallTable};

use crate::interface::{ISAL, TimeSpec, OpenFlags, Stat};

use libc;

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
        let mut ret = syscall(
            SyscallTable::READ,
            [
                fd,
                buffer.as_ptr() as usize,
                len,
                0, 0, 0
            ]
        );

        if ret == -21 { // try read dir
            let read_size = syscall(
                SyscallTable::GETDENTS,
                [
                    fd,
                    buffer.as_ptr() as usize,
                    len,
                    0, 0, 0
                ]
            );

            if read_size < 0 {
                return read_size;
            }

            unsafe {
                
                let mut index: usize = 0;
                let mut offset = 0;

                while index < read_size as usize {

                    let dirent_ptr = (buffer[offset..]).as_ptr() as *mut libc::dirent;

                    let dir_name_addr = (*dirent_ptr).d_name.as_ptr() as usize - 1;

                    let name_slice: &[u8] = core::slice::from_raw_parts(
                        dir_name_addr as *const u8,
                        libc::strlen(dir_name_addr as *const i8) as usize
                    );

                    for y in 0..name_slice.len() {
                        buffer[index] = name_slice[y];
                        index += 1;
                    }
                    buffer[index] = '\n' as u8;
                    index += 1;
                    offset += (*dirent_ptr).d_reclen as usize;
                }

                ret = index as isize;

            }
        }


        ret
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

    fn sys_stat(path_ptr: usize, stat: &mut Stat) -> isize {

        let mut file_stat: libc::stat = unsafe { core::mem::zeroed() };
        
        let ret = syscall(
            SyscallTable::STAT,
            [
                path_ptr,
                (&mut file_stat as *mut libc::stat) as usize,
                0, 0, 0, 0
            ]
        );

        stat.dev = file_stat.st_dev;
        stat.ino = file_stat.st_ino;
        stat.mode = file_stat.st_mode;
        stat.nlink = file_stat.st_nlink;
        stat.uid = file_stat.st_uid;
        stat.gid = file_stat.st_gid;
        stat.rdev = file_stat.st_rdev;
        stat.size = file_stat.st_size;
        stat.blksize = file_stat.st_blksize;
        stat.blocks = file_stat.st_blocks;

        ret
    }

}