#![no_std]
pub use interface::*;
use crate::os::syscall;

mod lang_items;
mod arch;
mod interface;
mod os;


pub fn hello() {
    syscall::sys_write(1, "Hello, SAL!\n".as_bytes());
}

pub fn open(path: &str, mode: interface::OpenFlags) -> isize {
    assert!(path.len() < 256);
    let mut arr:[u8; 256] = [0; 256];
    for (i, &byte) in path.as_bytes().iter().enumerate() {
        arr[i] = byte;
    }
    syscall::sys_open(arr.as_ptr() as usize, mode)
}

pub fn read(fd: usize, buffer: &mut [u8], len: usize) -> isize {
    syscall::sys_read(fd, buffer, len)
}

pub fn write(fd: usize, buffer: &[u8]) -> isize {
    syscall::sys_write(fd, buffer)
}

pub fn ioctl(fd: usize, cmd: usize, arg: usize) -> isize {
    syscall::sys_ioctl(fd, cmd, arg)
}

pub fn close(fd: usize) -> isize {
    syscall::sys_close(fd)
}

pub fn exit(xstate: i32) -> isize {
    syscall::sys_exit(xstate)
}

pub fn sleep(sec: usize) -> isize {
    let ts = interface::TimeSpec { tv_sec: sec, tv_nsec: 0  };
    let tsn = interface::TimeSpec { tv_sec: 0, tv_nsec: 0 };
    syscall::sys_nanosleep(ts, tsn)
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