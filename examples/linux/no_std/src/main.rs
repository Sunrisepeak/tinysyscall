#![no_std]
#![no_main]


#![feature(lang_items)]


extern crate sal;

#[no_mangle]
fn _start() { // entry point
    main();
}

fn main() {
    let hw = "Hello No Std, Support by SAL\n";
    sal::file::write(1, hw.as_bytes());
    sal::sleep(2);
    sal::exit(0);
}

// rust lang items
#[panic_handler]
extern fn panic(_info: &core::panic::PanicInfo) -> ! {  loop { } }

#[lang = "eh_personality"]
fn eh_personality() {}
