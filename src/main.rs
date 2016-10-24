#![feature(lang_items)]
#![feature(link_args)]
#![feature(start)]
#![no_std]
#![no_main]


#[link_args = "/SUBSYSTEM:WINDOWS"]
extern {}

extern crate rlibc;

mod win32;

fn main()
{
    let w32 = win32::Api::new();

    win32::output_debug_string_a(b"hej\n\0");
    win32::output_debug_string_a(b"hej\n\0");
    win32::output_debug_string_a(b"hej\n\0");

    w32.user32.message_box(b"Hi\0", b"there\0", 0);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn WinMainCRTStartup()
{
    main();
    
    win32::exit_process(0);
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {

    win32::exit_process(1);
}