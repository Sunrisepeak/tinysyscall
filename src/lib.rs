#![no_std]

use interface::ISAL;
use os::syscall;

mod lang_items;
mod arch;
mod interface;
mod os;


pub fn hello() {
    syscall::sys_write(1, "Hello, SAL!\n".as_bytes());
}

pub fn exit(xstate: i32) -> isize {
    syscall::sys_exit(xstate)
}

pub fn write(fd: usize, buffer: &[u8]) -> isize {
    syscall::sys_write(fd, buffer)
}

pub fn sleep(sec: usize) -> isize {
    let ts = interface::Timespec { tv_sec: sec, tv_nsec: 0  };
    let tsn = interface::Timespec { tv_sec: 0, tv_nsec: 0 };
    syscall::sys_nanosleep(ts, tsn)
}



#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(0, sleep(2) as usize);
    }
}