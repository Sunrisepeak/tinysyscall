#![no_std]

//use core::mem;
use os::syscall;
use interface::isal;

mod lang_items;
mod arch;
mod interface;
mod os;
mod console;
mod utils;

// public api
pub use utils::*;
pub use crate::interface::*;
pub use console::print;
pub use os::abi_types;

pub use self::file::*;
pub use self::process::*;
pub use self::time::*;


// println! 和 print! 
// 宏标准库实现: https://doc.rust-lang.org/nightly/src/std/macros.rs.html#132-139
//#[macro_export]
#[macro_export(local_inner_macros)]
macro_rules! print {
    ($($arg:tt)*) => {{
        // 调用上面的print函数
        $crate::print(core::format_args!($($arg)*));
    }};
}

//#[macro_export]
#[macro_export(local_inner_macros)]
macro_rules! println {
    // 分支1: 当无参数时
    () => {
        $crate::print!("\n")
    };
    // 分支2
    ($($arg:tt)*) => {{
        $crate::print!("{}\n", core::format_args!($($arg)*));
    }};
}

pub fn hello() {
    use interface::isal::File;
    println!("hello");
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



#[cfg(test)]
mod tests {
    use super::*;
    
    const TEST_FILE: &str = "tests/sal-test.data";

    #[test]
    fn hello_sal() {
        assert_eq!((), hello());
    }

    #[test]
    fn sal_exit() {
        //assert_eq!(3, exit(3));
    }

    #[test]
    fn sal_write() {
        let hw = "Hello, World!\n";
        assert_eq!(hw.len(), write(1, hw.as_bytes()) as usize);
    }

    #[test]
    fn sal_sleep() {
        assert_eq!(0, sleep(1) as usize);
    }

    #[test]
    fn sal_open_read_close() {
        const READ_LEN: usize = 2;
        let fd = open(TEST_FILE, OpenFlags::Create | OpenFlags::RW);
        debug_assert!(fd > 0);
        let buffer: &mut [u8; READ_LEN] = &mut [0; READ_LEN];
        assert_eq!(read(fd as usize, buffer, READ_LEN), READ_LEN as isize);
        assert_eq!(close(fd as usize), 0);
    }
}