#![no_std]

//use core::mem;
use os::syscall;
use interface::isal;

mod lang_items;
mod arch;
mod interface;
mod os;
mod utils;

// public api
pub use utils::*;
pub use crate::interface::*;
pub use os::abi_types;

pub use self::file::*;
pub use self::process::*;
pub use self::time::*;

pub fn hello() {
    use interface::isal::File;
    syscall::sys_write(1, "Hello, SAL!\n".as_bytes());
}

pub mod task {
    use super::*;
    use isal::Task;

    pub fn create<Func>(func: Func, stack_ptr: & mut [u8]) -> usize
    where Func: Fn() + 'static {
        unsafe {
            syscall::create(func, stack_ptr.as_mut_ptr().add(stack_ptr.len() - 1) as usize)
        }
    }
}

pub mod mem {
    use super::*;
    use isal::Memory;

    pub fn mmap(
        addr: usize,
        size: usize,
        prot: MemProt,
        flags: MemFlags,
        fd: isize,
        offset: usize
    ) -> usize {
        syscall::mmap(addr, size, prot, flags, fd as usize, offset)
    }

    pub fn unmmap(addr: usize, len: usize) -> isize {
        syscall::unmmap(addr, len)
    }
}

pub mod file {
    use super::*;
    use isal::File;

    pub fn open(path: &str, mode: interface::OpenFlags) -> isize {
        let arr:[u8; 256] = path_check_and_convert(path);
        syscall::sys_open(arr.as_ptr() as usize, mode)
    }
    
    // TODO: del len
    pub fn read(fd: usize, buffer: &mut [u8], len: usize) -> isize {
        syscall::sys_read(fd, buffer, len)
    }
    
    pub fn write(fd: usize, buffer: &[u8]) -> isize {
        syscall::sys_write(fd, buffer)
    }
    
    pub fn ioctl(fd: usize, cmd: usize, arg: usize) -> isize {
        syscall::sys_ioctl(fd, cmd, arg)
    }
    
    pub fn stat(path: &str, stat: &mut Stat) -> isize {
        // Convert path to array of bytes
        let path_arr = path_check_and_convert(path);
        /*
        println!("---------> {}", mem::size_of::<Stat>());
        let stat_bytes: &mut [u8] = unsafe {
            // Safety: `Stat` and `[u8; size_of::<Stat>()]` have the same size and alignment
            mem::transmute::<&mut Stat, &mut [u8; mem::size_of::<Stat>()]>(stat)
        };
        */
        // Call syscall to get stat information
        syscall::sys_stat(path_arr.as_ptr() as usize, stat)
    }
    
    pub fn close(fd: usize) -> isize {
        syscall::sys_close(fd)
    }
}

pub mod process {
    use super::*;
    use isal::Process;
    pub fn exit(xstate: i32) -> isize {
        syscall::sys_exit(xstate)
    }
}

pub mod time {
    use super::*;
    use isal::Time;
    pub fn sleep(sec: usize) -> isize {
        let ts = interface::TimeSpec { tv_sec: sec, tv_nsec: 0  };
        let tsn = interface::TimeSpec { tv_sec: 0, tv_nsec: 0 };
        syscall::sys_nanosleep(ts, tsn)
    }
}


fn path_check_and_convert(path: &str) -> [u8; 256] {
    assert!(path.len() < 256);
    let mut arr:[u8; 256] = [0; 256];
    for (i, &byte) in path.as_bytes().iter().enumerate() {
        arr[i] = byte;
    }
    arr
}