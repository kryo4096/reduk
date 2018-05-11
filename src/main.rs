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
use memory::{AreaFrameAllocator, FrameAllocator};

#[no_mangle]
pub fn _start() -> ! {

    let _boot_info;

    unsafe {
        _boot_info = boot_info::get_boot_info();
    }

    let mut frame_allocator = AreaFrameAllocator::new(&mut _boot_info.memory_map);

    vga_buffer::clear_bg(Color::White);
    vga_buffer::set_foreground(Color::DarkGray);

    kprintln!("reduk v0.0.1");

    frame_allocator.print_memory_map();

    for i in 0..100 {
        frame_allocator.allocate_frame();
    }

    frame_allocator.print_memory_map();

    keyboard::wait_any();


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
