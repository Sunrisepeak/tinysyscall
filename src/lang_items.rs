// 标志当程序发生异常时的处理函数
// Rust 强制需要实现的 "语义", 默认由std实现, 当禁止标准库std后需要用户自己实现
#[panic_handler]
#[cfg(target_os = "none")]
extern fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}