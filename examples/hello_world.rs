use tinysyscall;

fn main() {
    let hw = "Hello, World!\n";
    tinysyscall::file::write(tinysyscall::file::STDOUT, hw.as_bytes());
}