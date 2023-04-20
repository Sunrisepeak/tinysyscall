use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OpenFlags: usize {
        const Read = 0b00000000;
        const Write = 0b00000001;
        const RW = 0b00000010;
        const Create = 0b00000100;
    }
}

pub struct TimeSpec {
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
    // file
    fn sys_open(path_ptr: usize, flags: OpenFlags) -> isize;
    fn sys_read(fd: usize, buffer: &mut [u8], len: usize) -> isize;
    fn sys_write(fd: usize, buffer: &[u8]) -> isize;
    fn sys_ioctl(fd: usize, cmd: usize, arg: usize) -> isize;
    fn sys_close(fd: usize) -> isize;

    // control
    fn sys_exit(xstate: i32) -> isize;
    fn sys_nanosleep(req: TimeSpec, rem: TimeSpec) -> isize;
}

// pub struct HAL DefualtImpl