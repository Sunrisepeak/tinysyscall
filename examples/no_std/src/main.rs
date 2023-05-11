#![no_std]  // 禁用标准库
#![no_main] // 关闭main为程序入口函数

extern crate tinysyscall;

/// 修改程序入口函数名main -> _start, 其中_* 字符告诉编译器没有使用该函数时
/// 不用发出警告. 并且使用no_mangle属性, 让编译器保留原始函数名`_start`
#[no_mangle]
extern fn _start() {
    tinysyscall::hello();
    tinysyscall::time::sleep(3);
    tinysyscall::process::exit(0);
}