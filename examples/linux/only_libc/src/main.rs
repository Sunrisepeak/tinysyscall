#![no_std]
#![no_main]

#![feature(lang_items)]


extern crate tinysyscall;

// main called in libc
#[no_mangle]
fn main() {
    tinysyscall::file::write(tinysyscall::file::STDOUT, "Hello, world!\n".as_bytes());
}

// rust lang items
#[panic_handler]
extern fn panic(_info: &core::panic::PanicInfo) -> ! {  loop { } }

#[lang = "eh_personality"]
fn eh_personality() {}