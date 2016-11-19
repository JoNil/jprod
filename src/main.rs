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
mod shader;
mod window;

use shader::Shader;
use window::Window;

static FRAGMENT_SOURCE: &'static [u8] = br##"

#version 450 core

in vec2 frag_uv;

layout(location = 0) out vec4 color;

void main()
{
    color = vec4(frag_uv, 0.0, 1.0);
}
"##;

static VERTEX_SOURCE: &'static [u8] = br##"

#version 450 core

layout(location = 0) in vec3 vertex_pos;

out vec2 frag_uv;

void main()
{
    frag_uv = vertex_pos.xy/2.0 + 0.5;

    gl_Position = vec4(vertex_pos, 1.0);
}
"##;

fn main() {

    let window = Window::new();

    let shader = Shader::new(FRAGMENT_SOURCE, VERTEX_SOURCE);

    loop {
        window.process_messages();

        // win32::message_box(b"Frame\0", b"Frame\0", 0);


        window.swap();
    }
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
