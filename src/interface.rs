
pub struct Timespec {
    pub tv_sec: usize,
    pub tv_nsec: isize,
}

// syscall abstract layer interface
pub trait ISAL {
    fn sys_exit(xstate: i32) -> isize;
    fn sys_write(fd: usize, buffer: &[u8]) -> isize;
    fn sys_nanosleep(req: Timespec, rem: Timespec) -> isize;
}

// pub struct HAL DefualtImpl