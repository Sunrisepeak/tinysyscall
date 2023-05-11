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

// Memory protection flags
bitflags! {
    pub struct MemProt: i32 {
        const NONE = 0;
        const READ = 1;
        const WRITE = 2;
        const EXEC = 4;
    }
}

// Mapping flags
bitflags! {
    pub struct MemFlags: u32 {
        const SHARED = 1;
        const SHARED_VALIDATE = 3;
        const PRIVATE = 2;
        const ANONYMOUS = 0x20;
        const PRIVATE_EXCEPT_UFFD = 0x8000_0000;
    }
}

pub struct TimeSpec {
    pub tv_sec: usize,
    pub tv_nsec: isize,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Stat {
    pub dev: u64, // device ID of device containing file
    pub ino: u64, // file serial number
    pub mode: u32, // mode of file (see below)
    pub nlink: u64, // number of hard links
    pub uid: u32, // user ID of the file
    pub gid: u32, // group ID of the file
    pub rdev: u64,
    pub size: i64, // total size, in bytes
    pub blksize: i64, // blocksize for file system I/O
    pub blocks: i64, // number of 512B blocks allocated
    pub atime: i64, // time of last access
    pub mtime: i64, // time of last modification
    pub ctime: i64, // time of last status change
}

impl Stat {
    pub fn default() -> Self {
        Stat {
            dev: 0,
            ino: 0,
            mode: 0,
            nlink: 0,
            uid: 0,
            gid: 0,
            rdev: 0,
            size: 0,
            blksize: 0,
            blocks: 0,
            atime: 0,
            mtime: 0,
            ctime: 0,
        }
    }
}