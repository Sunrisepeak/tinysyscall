#[cfg(target_arch = "riscv64")]
use crate::arch::riscv::{syscall, SyscallTable};

#[cfg(target_arch = "x86_64")]
use crate::arch::x86_64::{syscall, SyscallTable};

use crate::interface::{isal, TimeSpec, Stat};
use crate::interface::{MemProt, MemFlags, OpenFlags};



use crate::os::abi_types;

/// [type def]: https://faculty.nps.edu/cseagle/assembly/sys_call.html
/// [error code]: https://github.com/torvalds/linux/blob/master/include/uapi/asm-generic/errno-base.h

pub struct SAL;

extern "C" fn _thread_func_wrapper(_arg: *mut libc::c_void) -> i32 {
    //let arg = arg as usize;
    crate::file::write(crate::file::STDOUT, "TODO: thread_func_wrapper no impl".as_bytes());
    //panic!("test");
    0
}

impl isal::Task for SAL {
    fn create<Func>(_func: Func, _stack_top_ptr: usize) -> usize
    where Func: Fn() + 'static {
/*
        // let flags = libc::CLONE_VM | libc::CLONE_FS | libc::CLONE_FILES | libc::CLONE_SIGHAND | libc::CLONE_THREAD;
        let flags = 0x0000_0100 | 0x0000_0200 | 0x0000_0400 | 0x0000_0800 | 0x0001_0000;

        let func_ptr = &func as *const _ as *const dyn Fn();
        
        let mut tid = unsafe {
            libc::clone(
                thread_func_wrapper, // entry point for new thread
                stack_top_ptr as *mut libc::c_void, // top of child stack
                flags as libc::c_int, // thread creation flags
                func_ptr as *mut libc::c_void, // argument to pass to thread function
            )
        };

        //let task_ptr = thread_func_wrapper as *const ();

/*      TODO: use syscall clone directly
        let mut tid = syscall(
            SyscallTable::CLONE,
            [
                (func_ptr as *mut libc::c_void) as usize, // entry point for new thread
                stack_top_ptr, // top of child stack
                flags, // thread creation flags
                (func_ptr as *mut libc::c_void) as usize, // argument to pass to thread function
                0, 0
            ]
        );
        assert_eq!(tid, 0);

*/



        if tid < 0 {
            tid = 0;
        }

        tid as usize
 */
        1
    }



    fn destory(_tid: usize) {

    }
}

impl isal::Memory for SAL {
    fn mmap(
        addr: usize,
        size: usize,
        prot: MemProt,
        flags: MemFlags,
        fd: usize,
        offset: usize
    ) -> usize {
        let ret;

        #[cfg(target_arch = "riscv64")]
        {
            let mut args = abi_types::mmap_args_struct {
                addr: addr,
                size: size,
                prot: prot.bits(),
                flags: flags.bits(),
                fd: fd,
                offset: offset
            };

            let args_ptr = args as *mut abi_types::mmap_args_struct;

            ret = syscall(
                SyscallTable::MMAP,
                [
                    args_ptr as usize,
                    0, 0, 0, 0, 0
                ]
            );

        }

        #[cfg(target_arch = "x86_64")]
        {
            ret = syscall(
                SyscallTable::MMAP,
                [
                    addr,
                    size,
                    prot.bits() as usize,
                    flags.bits() as usize,
                    fd,
                    offset
                ]
            );
        }

        ret as usize
    }
    
    fn unmmap(addr: usize, len: usize) -> isize {
        let ret = syscall(
            SyscallTable::MUNMAP,
            [
                addr,
                len,
                0, 0, 0, 0
            ]
        );
        ret
    }
}

impl isal::File for SAL {

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
                        crate::strlen(dir_name_addr as *const u8)
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

    // TODO: libc::stat -> stat, tmp ok adjust struct
    fn sys_stat(path_ptr: usize, stat: &mut Stat) -> isize {

        let mut file_stat: abi_types::stat = unsafe { core::mem::zeroed() };
        let file_stat_ptr = (&mut file_stat as *const abi_types::stat) as usize;
        //assert_eq!(core::mem::size_of::<abi_types::stat>(), core::mem::size_of::<libc::stat>());

        let ret = syscall(
            SyscallTable::STAT,
            [
                path_ptr,
                file_stat_ptr,
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

impl isal::Process for SAL {
    fn sys_exit(xstate: i32) -> isize {
        syscall(
            SyscallTable::EXIT,
            [
                xstate as usize,
                0, 0, 0, 0, 0
            ]
        )
    }
}

impl isal::Time for SAL {
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
