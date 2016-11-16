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

use window::Window;

fn main() {

    let window = Window::new();

    gl::GetNamedBufferPointerv::load_with(|s| opengl32::get_proc_address(s));

    gl::CreateProgram::load_with(|s| opengl32::get_proc_address(s));
    gl::DeleteProgram::load_with(|s| opengl32::get_proc_address(s));

    gl::CreateShader::load_with(|s| opengl32::get_proc_address(s));
    gl::DeleteShader::load_with(|s| opengl32::get_proc_address(s));


    let program = unsafe { gl::CreateProgram() };
    if program == 0 {
        panic!();
    }

    let fragment = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    if fragment == 0 {
        panic!();
    }

    let vertex = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    if vertex == 0 {
        panic!();
    }


    loop {
        window.process_messages();

        // win32::message_box(b"Frame\0", b"Frame\0", 0);


        window.swap();
    }

    unsafe { gl::DeleteProgram(program) };
    unsafe { gl::DeleteShader(fragment) };
    unsafe { gl::DeleteShader(vertex) };
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
