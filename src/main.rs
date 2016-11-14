#![feature(const_fn)]
#![feature(lang_items)]
#![feature(link_args)]
#![no_main]
#![no_std]

#[link_args = "/SUBSYSTEM:WINDOWS"]
extern "C" {}

extern crate rlibc;

#[macro_use]
mod win32_macros;

mod c_types;
mod gdi32;
mod gl;
mod module;
mod opengl32;
mod utils;
mod win32;
mod win32_types;
mod window;

use core::ptr;
use core::mem;
use win32_types::*;

static WINDOW_CLASS: &'static [u8] = b"C\0";
static WINDOW_NAME: &'static [u8] = b"JProd\n\0";

static WGL_ATTRIBS: &'static [i32] =
&[
    WGL_CONTEXT_MAJOR_VERSION_ARB, 4,
    WGL_CONTEXT_MINOR_VERSION_ARB, 5,
    WGL_CONTEXT_FLAGS_ARB, WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB | WGL_CONTEXT_DEBUG_BIT_ARB,
    WGL_CONTEXT_PROFILE_MASK_ARB, WGL_CONTEXT_CORE_PROFILE_BIT_ARB,
    0,
];

static WINDOW_ATTRIBS: &'static [i32] =
&[
    WGL_DRAW_TO_WINDOW_ARB, 1,
    WGL_ACCELERATION_ARB, WGL_FULL_ACCELERATION_ARB,
    WGL_SUPPORT_OPENGL_ARB, 1,
    WGL_DOUBLE_BUFFER_ARB, 1,
    WGL_PIXEL_TYPE_ARB, WGL_TYPE_RGBA_ARB,
    0,
];

extern "system" fn window_proc(window: WindowHandle,
                               msg: u32,
                               wparam: usize,
                               lparam: usize)
                               -> usize {

    match msg {

        WM_SIZE => {
            win32::output_debug_string(b"WM_SIZE\n\0");
        }

        WM_CLOSE => {
            win32::output_debug_string(b"WM_CLOSE\n\0");
            win32::exit_process(0);
        }

        WM_ACTIVATEAPP => {
            win32::output_debug_string(b"WM_ACTIVATEAPP\n\0");
        }

        WM_DESTROY => {
            win32::output_debug_string(b"WM_DESTROY\n\0");
        }

        _ => {
            return win32::def_window_proc(window, msg, wparam, lparam);
        }
    }

    return 0;
}

fn set_pixel_format(dc: DcHandle, initial: bool) {

    let mut suggested_pixel_format_index = 0;
    let mut extended_pick = 0;

    if !initial {
        if opengl32::choose_pixel_format(dc, Some(WINDOW_ATTRIBS), None, &mut suggested_pixel_format_index, &mut extended_pick) == 0 {
            panic!();
        }
    }

    if extended_pick == 0 {

        let desired_pixel_format = PixelFormatDescriptor {
            size: mem::size_of::<PixelFormatDescriptor>() as u16,
            version: 1,
            flags: PFD_SUPPORT_OPENGL | PFD_DRAW_TO_WINDOW | PFD_DOUBLEBUFFER,
            pixel_type: PFD_TYPE_RGBA,
            color_bits: 32,
            red_bits: 0,
            red_shift: 0,
            green_bits: 0,
            green_shift: 0,
            blue_bits: 0,
            blue_shift: 0,
            alpha_bits: 8,
            alpha_shift: 0,
            accum_bits: 0,
            accum_red_bits: 0,
            accum_green_bits: 0,
            accum_blue_bits: 0,
            accum_alpha_bits: 0,
            depth_bits: 0,
            stencil_bits: 0,
            aux_buffers: 0,
            layer_type: PFD_MAIN_PLANE,
            reserved: 0,
            layer_mask: 0,
            visible_mask: 0,
            damage_mask: 0,
        };

        suggested_pixel_format_index = gdi32::choose_pixel_format(dc, &desired_pixel_format);
        if suggested_pixel_format_index == 0 {
            panic!();
        }
    }

    let mut suggested_pixel_format = unsafe { mem::uninitialized() };
    if gdi32::describe_pixel_format(dc,
                                    suggested_pixel_format_index,
                                    mem::size_of::<PixelFormatDescriptor>() as u32,
                                    &mut suggested_pixel_format) == 0 {
        panic!();
    }

    if gdi32::set_pixel_format(dc, suggested_pixel_format_index, &suggested_pixel_format) == 0 {
        panic!();
    }
}

fn main() {
    

    if !win32::register_class(WINDOW_CLASS, window_proc) {
        panic!();
    }

    {
        let window = win32::create_window(WINDOW_CLASS, WINDOW_NAME, false);

        let gl_dc = win32::get_dc(window);
        if gl_dc == ptr::null_mut() {
            panic!();
        }

        set_pixel_format(gl_dc, true);

        let gl_context = opengl32::create_context(gl_dc);
        if gl_context == ptr::null_mut() {
            panic!();
        }

        if opengl32::make_current(gl_dc, gl_context) == 0 {
            panic!();
        }

        opengl32::load_extensions();

        if opengl32::make_current(ptr::null_mut(), ptr::null_mut()) == 0 {
            panic!();
        }

        if opengl32::delete_context(gl_context) == 0 {
            panic!();
        }

        if win32::release_dc(window, gl_dc) == 0 {
            panic!();
        }

        if win32::destroy_window(window) == 0 {
            panic!();
        }
    }

    let window = win32::create_window(WINDOW_CLASS, WINDOW_NAME, true);

    let gl_dc = win32::get_dc(window);
    if gl_dc == ptr::null_mut() {
        panic!();
    }

    set_pixel_format(gl_dc, false);

    let gl_context = opengl32::create_context_attribs(gl_dc, ptr::null_mut(), WGL_ATTRIBS);
    if gl_context == ptr::null_mut() {
        panic!();
    }

    if opengl32::make_current(gl_dc, gl_context) == 0 {
        panic!();
    }

    gl::GetNamedBufferPointerv::load_with(|s| opengl32::get_proc_address(s));

    if window != ptr::null_mut() {
        loop {
            if let Some(msg) = win32::get_message() {
                win32::translate_and_dispatch_message(&msg);
            }
        }
    }

    win32::message_box(b"Hi\0", b"there\0", 0);
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn WinMainCRTStartup() {

    win32::init();
    gdi32::init();
    opengl32::init();

    main();

    win32::exit_process(0);
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(_msg: core::fmt::Arguments,
                                   _file: &'static str,
                                   _line: u32)
                                   -> ! {

    win32::exit_process(1);
}

// #[allow(non_snake_case)]
// #[no_mangle]
// pub extern "system" fn __CxxFrameHandler3(_: usize, _: usize, _: usize, _: usize) {
// win32::exit_process(1);
// }
