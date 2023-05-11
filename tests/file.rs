use std;
use core::ffi::CStr;
use libc::c_char;

use tinysyscall as sal;

#[test]
fn file_ops() {
    
    let fd = sal::file::open(
        "tests/sal-test.data",
        sal::file::OpenFlags::Create | sal::file::OpenFlags::RW
    );

    assert!(fd > 0);

    const READ_LEN: usize = 10;
    let buffer: &mut [u8; READ_LEN] = &mut [0; READ_LEN];
    
    assert_eq!(sal::file::read(fd, buffer, READ_LEN), READ_LEN as isize);
    assert_eq!(sal::file::write(fd, buffer), READ_LEN as isize);
    assert_eq!(sal::file::close(fd), 0);
}

#[test]
fn file_info() {
    let fd = sal::file::open(
        "tests/sal-test.data",
        sal::file::OpenFlags::Create | sal::file::OpenFlags::RW
    );
    assert!(fd > 0);

    let mut stat = sal::file::Stat::default();

    assert_eq!(sal::file::stat("tests/sal-test.data", &mut stat), 0);

    let metadata = std::fs::metadata("tests/sal-test.data").expect("Failed to open file");

    let file_size = metadata.len();

    assert_eq!(stat.size, file_size as i64);

    assert_eq!(sal::file::close(fd), 0);

}

#[test]
fn current_dir_list() {

    let fd = sal::file::open(
        ".",
        sal::file::OpenFlags::Create | sal::file::OpenFlags::Read
    );

    assert!(fd > 0);

    const READ_LEN: usize = 1024;
    let buffer: &mut [u8; READ_LEN] = &mut [0; READ_LEN];
    
    let mut offset = 0;
    loop {
        let read_len = sal::file::read(fd, &mut buffer[offset..], READ_LEN - offset);
        if read_len <= 0 {
            break;
        }
        offset += read_len as usize;
    }

    let files;
    unsafe {
        files = CStr::from_ptr(buffer[..offset].as_ptr() as *const c_char).to_str().unwrap();
    }
    // overwrite
    let files = files.split('\n');
    let mut file_nums = 0;
    std::println!();
    for file in files {
        if !file.is_empty() {
            //std::println!("current_dir_list: {}", file);
        }
        file_nums += 1;
    }
    assert!(file_nums >= 2);
    assert_eq!(sal::file::close(fd), 0);

}