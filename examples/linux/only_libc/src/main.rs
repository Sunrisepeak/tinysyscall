#![no_std]
#![no_main]

#![feature(lang_items)]


extern crate sal;

#[no_mangle]
fn main() {
    sal::println!("Hello, world!");
}

// rust lang items
#[panic_handler]
extern fn panic(_info: &core::panic::PanicInfo) -> ! {  loop { } }

#[lang = "eh_personality"]
fn eh_personality() {}