#![feature(lang_items, const_fn, asm)]
#![no_std]
#![no_main]

extern crate rlibc;
extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate os_bootinfo;
extern crate spin;
extern crate x86_64;
#[macro_use]
extern crate bitflags;

#[macro_use]
mod vga_buffer;
mod boot_info;
mod fixed_queue;
mod keyboard;
mod memory;
mod random;

use vga_buffer::Color;

use memory::paging::{Mapper, Page, PageTable, RecursivePageTable, Size4KB};
use memory::VirtAddr;

#[no_mangle]
pub fn _start() -> ! {
    vga_buffer::clear_bg(Color::White);
    vga_buffer::set_foreground(Color::DarkGray);

    kprintln!("reduk v0.0.1");

    boot_info::show();
    keyboard::wait_any();

    let p4_addr = boot_info::BOOT_INFO.lock().p4_table_addr;

    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(
    _msg: core::fmt::Arguments,
    _file: &'static str,
    _line: u32,
    _column: u32,
) -> ! {
    vga_buffer::set_foreground(Color::Red);
    kprintln!();
    kprintln!("kernel panic");
    kprintln!("{}", _msg);
    kprintln!("{} line {}", _file, _line);

    loop {}
}
