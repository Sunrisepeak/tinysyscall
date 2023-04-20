use sal;

#[test]
fn hello_sal() {
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