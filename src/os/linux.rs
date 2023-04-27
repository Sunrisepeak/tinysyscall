#[cfg(target_arch = "riscv64")]
use crate::arch::riscv::{syscall, SyscallTable};

#[cfg(target_arch = "x86_64")]
use crate::arch::x86_64::{syscall, SyscallTable};

use crate::interface::{ISAL, TimeSpec, OpenFlags, Stat};

use crate::os::abi_types;

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

            let mut index: usize = 0;
            let mut offset = 0;

            while index < read_size as usize {

                let dirent_ptr = (buffer[offset..]).as_ptr() as *mut abi_types::dirent;

                let name_slice: &[u8];
                unsafe { // convert file name to name slice
                    // TODO: why - 1 ?
                    let dir_name_addr = (*dirent_ptr).d_name.as_ptr() as usize - 1;

                    name_slice = core::slice::from_raw_parts(
                        dir_name_addr as *const u8,
                        strlen(dir_name_addr as *const u8)
                    );

                    offset += (*dirent_ptr).d_reclen as usize;
                }

                for y in 0..name_slice.len() {
                    buffer[index] = name_slice[y];
                    index += 1;
                }

                buffer[index] = '\n' as u8;
                index += 1;
                
            }

            ret = index as isize;

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

    // TODO: libc::stat -> stat
    fn sys_stat(path_ptr: usize, stat: &mut Stat) -> isize {

        let mut file_stat: abi_types::stat = unsafe { core::mem::zeroed() };
        let file_stat_ptr = (&mut file_stat as *mut abi_types::stat) as usize;
        //assert_eq!(core::mem::size_of::<abi_types::stat>(), core::mem::size_of::<libc::stat>());

        let ret = syscall(
            SyscallTable::STAT,
            [
                path_ptr,
                file_stat_ptr,
                0, 0, 0, 0
            ]
        );

/*      // pointer align?
        let file_stat2 = (file_stat_ptr - 1) as *mut abi_types::stat;

        unsafe {
            assert_eq!(file_stat.st_dev, 66308);
            
            assert_eq!(file_stat.st_ino, (*file_stat2).st_ino);
            assert_eq!(file_stat.st_mode, (*file_stat2).st_mode);
            assert_eq!(file_stat.st_nlink, (*file_stat2).st_nlink);
            assert_eq!(file_stat.st_uid, (*file_stat2).st_uid);
            assert_eq!(file_stat.st_gid, (*file_stat2).st_gid);
            assert_eq!(file_stat.st_rdev, (*file_stat2).st_rdev);
            assert_eq!(file_stat.st_size, (*file_stat2).st_size);
            assert_eq!(file_stat.st_blksize, (*file_stat2).st_blksize);
            assert_eq!(file_stat.st_blocks, (*file_stat2).st_blocks);
            assert_eq!(file_stat.st_atime, (*file_stat2).st_atime);
            assert_eq!(file_stat.st_mtime, (*file_stat2).st_mtime);
            assert_eq!(file_stat.st_ctime, (*file_stat2).st_ctime);
            
        }
*/

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


unsafe fn strlen(s: *const u8) -> usize {
    let mut len = 0;
    unsafe {
        while *s.offset(len as isize) != 0 {
            len += 1;
        }
    }
    len
}