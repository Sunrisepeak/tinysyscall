use tinysyscall as sal;

#[test]
fn hello_sal() {
    assert_eq!((), sal::hello());
}

#[test]
fn stdout() {
    let tiny_syscall = "hello everyone, say hi with tiny syscall...\n";
    sal::file::write(sal::file::STDOUT, tiny_syscall.as_bytes());
}