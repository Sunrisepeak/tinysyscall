/*
macro_rules! if_else {
    ($cond:expr, $if:expr, $else:expr) => {
        if $cond {
            $if
        } else {
            $else
        }
    };
}
*/




#[repr(C)]
#[cfg(any(target_os = "linux", target_os = "none"))]
pub struct dirent {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [i8; 256],
}


#[cfg(target_os = "linux")]
pub use libc::stat;

#[repr(C)]
#[cfg(target_os = "none")]
pub struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_mode: u32,
    pub st_nlink: u64,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: u32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atime: i64,
    pub st_atime_nsec: i64,
    pub st_mtime: i64,
    pub st_mtime_nsec: i64,
    pub st_ctime: i64,
    pub st_ctime_nsec: i64,
    pub __unused: [i64; 2],
}