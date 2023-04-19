#![no_std]

use interface::ISAL;
use os::syscall;

mod lang_items;
mod arch;
mod interface;
mod os;


pub fn hello_sal() {
    syscall::sys_write(1, "Hello, SAL!\n".as_bytes());
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall::sys_exit(xstate)
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall::sys_write(fd, buffer)
}

pub fn sys_sleep(sec: usize) -> isize {
    let ts = interface::Timespec { tv_sec: sec, tv_nsec: 0  };
    let tsn = interface::Timespec { tv_sec: 0, tv_nsec: 0 };
    syscall::sys_nanosleep(ts, tsn)
}


/*

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_sal() {
        super::hello_sal();
    }
}

*/