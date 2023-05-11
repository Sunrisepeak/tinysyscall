#![no_std]
#![no_main]


#![feature(lang_items)]


extern crate tinysyscall;

#[no_mangle]
fn _start() { // entry point
    main();
}

fn main() {
    let hw = "Hello No Std, Support by SAL\n";
    tinysyscall::file::write(tinysyscall::file::STDOUT, hw.as_bytes());
    tinysyscall::time::sleep(2);
    tinysyscall::process::exit(0);
}

// rust lang items
#[panic_handler]
extern fn panic(_info: &core::panic::PanicInfo) -> ! {  loop { } }

#[lang = "eh_personality"]
fn eh_personality() {}
