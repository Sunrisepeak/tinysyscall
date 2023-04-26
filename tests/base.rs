use core::ffi::CStr;
use libc::c_char;
use sal;

#[test]
fn hello_sal() {
    println!();
    assert_eq!((), sal::hello());
}

#[test]
fn file_ops() {
    
    let fd = sal::open(
        "tests/sal-test.data",
        sal::OpenFlags::Create | sal::OpenFlags::RW
    );

    assert!(fd > 0);

    const READ_LEN: usize = 10;
    let buffer: &mut [u8; READ_LEN] = &mut [0; READ_LEN];
    
    assert_eq!(sal::read(fd as usize, buffer, READ_LEN), READ_LEN as isize);
    assert_eq!(sal::write(fd as usize, buffer), READ_LEN as isize);
    assert_eq!(sal::close(fd as usize), 0);
}

#[test]
fn file_info() {
    let fd = sal::open(
        "tests/sal-test.data",
        sal::OpenFlags::Create | sal::OpenFlags::RW
    );
    assert!(fd > 0);

    let mut stat = sal::Stat::default();

    assert_eq!(sal::stat("tests/sal-test.data", &mut stat), 0);

    sal::println!("\nfile_info: {:?}", stat);
    sal::println!("File size: {} bytes", stat.size as isize);
    sal::println!("Number of links: {}", stat.nlink);
    sal::println!("File inode: {}", stat.ino);
    sal::println!("File permissions: {:o}", stat.mode);
    sal::println!("File owner: {}", stat.uid);
    sal::println!("File group: {}", stat.gid);
    sal::println!("Last access time: {}", stat.atime);
    sal::println!("Last modification time: {}", stat.mtime);
    sal::println!("Last status change time: {}", stat.ctime);

    assert_eq!(sal::close(fd as usize), 0);

}

#[test]
fn current_dir_list() {
    println!("");

    let fd = sal::open(
        ".",
        sal::OpenFlags::Create | sal::OpenFlags::Read
    );

    assert!(fd > 0);

    const READ_LEN: usize = 1024;
    let buffer: &mut [u8; READ_LEN] = &mut [0; READ_LEN];
    
    let mut offset = 0;
    loop {
        let read_len = sal::read(fd as usize, &mut buffer[offset..], READ_LEN - offset);
        println!("current_dir_list: read size, {}", read_len);
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
    sal::println!();
    for file in files {
        if !file.is_empty() {
            sal::println!("current_dir_list: {}", file);
        }
        file_nums += 1;
    }
    assert!(file_nums >= 2);
    assert_eq!(sal::close(fd as usize), 0);

}


