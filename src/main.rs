#![feature(lang_items,const_fn)]

#![no_std]
#![no_main]

#[macro_use]
mod vga_buffer;

extern crate rlibc;
extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate spin;

#[no_mangle]
pub fn _start() -> ! {

    vga_buffer::clear();

    print!("reduk v0.1.0");

    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg : core::fmt::Arguments,
    _file: &'static str, _line: u32, _column: u32) -> !
{
    loop {}
}