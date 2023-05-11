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


//#[cfg(target_os = "linux")]
//pub use libc::stat;

#[repr(C)]
#[cfg(any(target_os = "linux", target_os = "none"))]
pub struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
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
    pub __unused: [i64; 3],
}

#[repr(C)]
#[cfg(target_arch = "riscv64")]
#[cfg(any(target_os = "linux", target_os = "none"))]
pub struct mmap_args_struct {
    pub addr: usize,
    pub len: usize,
    pub prot: usize,
    pub flags: usize,
    pub fd: usize,
    pub offset: usize,
}


#[cfg(test)]
#[cfg(target_os = "linux")]
mod linux_abi {
    use crate::os::abi_types;
    use core::mem;
    extern crate libc;

    #[test]
    fn dirent_abi() {

        assert_eq!(mem::size_of::<libc::dirent>(), mem::size_of::<abi_types::dirent>());

        let dirent1: abi_types::dirent = unsafe { crate::memset::<abi_types::dirent>(0xFF) };
        let dirent2: libc::dirent = unsafe { crate::memset::<libc::dirent>(0xFF) };

        assert_eq!(mem::size_of_val(&dirent1.d_ino), mem::size_of_val(&dirent2.d_ino));
        assert_eq!(mem::size_of_val(&dirent1.d_off), mem::size_of_val(&dirent2.d_off));
        assert_eq!(mem::size_of_val(&dirent1.d_reclen), mem::size_of_val(&dirent2.d_reclen));
        assert_eq!(mem::size_of_val(&dirent1.d_type), mem::size_of_val(&dirent2.d_type));
        assert_eq!(mem::size_of_val(&dirent1.d_name), mem::size_of_val(&dirent2.d_name));

        assert_eq!(dirent1.d_ino, dirent2.d_ino);
        assert_eq!(dirent1.d_off, dirent2.d_off);
        assert_eq!(dirent1.d_reclen, dirent2.d_reclen);
        assert_eq!(dirent1.d_type, dirent2.d_type);
        assert_eq!(dirent1.d_name, dirent2.d_name);

    }

    #[test]
    fn stat_abi() {

        assert_eq!(mem::size_of::<abi_types::stat>(), mem::size_of::<libc::stat>());

        let file_stat1: abi_types::stat = unsafe { crate::memset(0x81) };
        let file_stat2: libc::stat = unsafe { crate::memset(0x81) };

        assert_eq!(mem::size_of_val(&file_stat1.st_dev), mem::size_of_val(&file_stat2.st_dev));
        assert_eq!(mem::size_of_val(&file_stat1.st_ino), mem::size_of_val(&file_stat2.st_ino));
        assert_eq!(mem::size_of_val(&file_stat1.st_mode), mem::size_of_val(&file_stat2.st_mode));
        assert_eq!(mem::size_of_val(&file_stat1.st_nlink), mem::size_of_val(&file_stat2.st_nlink));
        assert_eq!(mem::size_of_val(&file_stat1.st_uid), mem::size_of_val(&file_stat2.st_uid));
        assert_eq!(mem::size_of_val(&file_stat1.st_gid), mem::size_of_val(&file_stat2.st_gid));
        assert_eq!(mem::size_of_val(&file_stat1.st_rdev), mem::size_of_val(&file_stat2.st_rdev));
        assert_eq!(mem::size_of_val(&file_stat1.st_size), mem::size_of_val(&file_stat2.st_size));
        assert_eq!(mem::size_of_val(&file_stat1.st_blksize), mem::size_of_val(&file_stat2.st_blksize));
        assert_eq!(mem::size_of_val(&file_stat1.st_blocks), mem::size_of_val(&file_stat2.st_blocks));
        assert_eq!(mem::size_of_val(&file_stat1.st_atime), mem::size_of_val(&file_stat2.st_atime));
        assert_eq!(mem::size_of_val(&file_stat1.st_mtime), mem::size_of_val(&file_stat2.st_mtime));
        assert_eq!(mem::size_of_val(&file_stat1.st_ctime), mem::size_of_val(&file_stat2.st_ctime));

        assert_eq!(file_stat1.st_dev, file_stat2.st_dev);
        assert_eq!(file_stat1.st_ino, file_stat2.st_ino);
        assert_eq!(file_stat1.st_mode, file_stat2.st_mode);
        assert_eq!(file_stat1.st_nlink, file_stat2.st_nlink);
        assert_eq!(file_stat1.st_uid, file_stat2.st_uid);
        assert_eq!(file_stat1.st_gid, file_stat2.st_gid);
        assert_eq!(file_stat1.st_rdev, file_stat2.st_rdev);
        assert_eq!(file_stat1.st_size, file_stat2.st_size);
        assert_eq!(file_stat1.st_blksize, file_stat2.st_blksize);
        assert_eq!(file_stat1.st_blocks, file_stat2.st_blocks);
        assert_eq!(file_stat1.st_atime, file_stat2.st_atime);
        assert_eq!(file_stat1.st_mtime, file_stat2.st_mtime);
        assert_eq!(file_stat1.st_ctime, file_stat2.st_ctime);

    }
}