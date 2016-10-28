#![feature(const_fn)]
#![feature(lang_items)]
#![feature(link_args)]
#![no_main]
#![no_std]


#[link_args = "/SUBSYSTEM:WINDOWS"]
extern {}

extern crate rlibc;

#[macro_use]
mod win32_macros;

mod opengl;
mod win32;
mod win32_types;

use core::ptr;
use win32_types::*;

static window_class: &'static [u8] = b"C\0";

static mut W32: *const win32::Api = ptr::null();

extern "system" fn window_proc(window: WindowHandle, msg: u32, wparam: usize, lparam: usize) -> usize {
    
    match msg {

        WM_SIZE => {
            win32::output_debug_string_a(b"WM_SIZE\n\0");
        }

        WM_CLOSE => {
            win32::output_debug_string_a(b"WM_CLOSE\n\0");
            win32::exit_process(0);
        }

        WM_ACTIVATEAPP => {
            win32::output_debug_string_a(b"WM_ACTIVATEAPP\n\0");
        }

        WM_DESTROY =>  {
            win32::output_debug_string_a(b"WM_DESTROY\n\0");
        }

        _ => {
            return unsafe { (*W32).def_window_proc(window, msg, wparam, lparam) };
        }
    }

    return 0;
}

fn main() {
    let w32 = win32::Api::new();
    unsafe { W32 = &w32 };

    let gl = opengl::Api::new();

    if !w32.register_class(window_class, window_proc) {
        panic!();
    }

    let window = w32.create_window(window_class, b"JProd\n\0");

    gl.create_context(&w32);

    if window != ptr::null_mut() {
        loop {
            if let Some(msg) = w32.get_message() {
                w32.translate_and_dispatch_message(&msg);
            }
        }
    }

    w32.message_box(b"Hi\0", b"there\0", 0);
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn WinMainCRTStartup() {

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