mod types;

pub use types::*;

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

pub mod isal {
    use super::*;

    pub trait File {
        fn sys_open(path_ptr: usize, flags: OpenFlags) -> isize;
        fn sys_read(fd: usize, buffer: &mut [u8], len: usize) -> isize;
        fn sys_write(fd: usize, buffer: &[u8]) -> isize;
        fn sys_ioctl(fd: usize, cmd: usize, arg: usize) -> isize;
        fn sys_stat(path_ptr: usize, stat: &mut Stat) -> isize;
        fn sys_close(fd: usize) -> isize;
    }

    pub trait Process {
        fn sys_exit(xstate: i32) -> isize;
    }

    pub trait Task {

    }

    pub trait Time {
        fn sys_nanosleep(req: TimeSpec, rem: TimeSpec) -> isize;
    }
}

/*
// syscall abstract layer interface
pub trait ISAL {
    // file
    fn sys_open(path_ptr: usize, flags: OpenFlags) -> isize;
    fn sys_read(fd: usize, buffer: &mut [u8], len: usize) -> isize;
    fn sys_write(fd: usize, buffer: &[u8]) -> isize;
    fn sys_ioctl(fd: usize, cmd: usize, arg: usize) -> isize;
    fn sys_stat(path_ptr: usize, stat: &mut Stat) -> isize;
    fn sys_close(fd: usize) -> isize;

    // control
    fn sys_exit(xstate: i32) -> isize;
    fn sys_nanosleep(req: TimeSpec, rem: TimeSpec) -> isize;
}
*/