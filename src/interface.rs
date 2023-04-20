
pub struct Timespec {
    pub tv_sec: usize,
    pub tv_nsec: isize,
}

#[allow(dead_code)]
pub enum SyscallTable {
    // file
    OPEN = 0,
    READ = 1,
    WRITE = 2,
    IOCTL = 3,
    FSTAT = 4,
    CLOSE = 5,

    // control
    NANOSLEEP = 6,
    EXIT = 7,
}

// syscall abstract layer interface
pub trait ISAL {
    fn sys_exit(xstate: i32) -> isize;
    fn sys_write(fd: usize, buffer: &[u8]) -> isize;
    fn sys_nanosleep(req: Timespec, rem: Timespec) -> isize;
}

// pub struct HAL DefualtImpl