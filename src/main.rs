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

fn print_shader_error(shader: u32) {

    const SIZE: i32 = 2048;

    let mut info: [u8; SIZE as usize] = [0; SIZE as usize];

    unsafe { gl::GetShaderInfoLog(shader, SIZE, ptr::null_mut(), &mut info[0]) };

    win32::output_debug_string(&info);
}

fn print_program_error(program: u32) {

    const SIZE: i32 = 2048;

    let mut info: [u8; SIZE as usize] = [0; SIZE as usize];

    unsafe { gl::GetProgramInfoLog(program, SIZE, ptr::null_mut(), &mut info[0]) };

    win32::output_debug_string(&info);
}

fn main() {

    let window = Window::new();

    gl::GetNamedBufferPointerv::load_with(|s| opengl32::get_proc_address(s));

    // Program functions
    gl::CreateProgram::load_with(|s| opengl32::get_proc_address(s));
    gl::DeleteProgram::load_with(|s| opengl32::get_proc_address(s));

    gl::AttachShader::load_with(|s| opengl32::get_proc_address(s));
    gl::LinkProgram::load_with(|s| opengl32::get_proc_address(s));
    gl::GetProgramInfoLog::load_with(|s| opengl32::get_proc_address(s));
    gl::ValidateProgram::load_with(|s| opengl32::get_proc_address(s));
    gl::GetProgramiv::load_with(|s| opengl32::get_proc_address(s));

    // Shader functions
    gl::CreateShader::load_with(|s| opengl32::get_proc_address(s));
    gl::DeleteShader::load_with(|s| opengl32::get_proc_address(s));

    gl::ShaderSource::load_with(|s| opengl32::get_proc_address(s));
    gl::CompileShader::load_with(|s| opengl32::get_proc_address(s));
    gl::GetShaderInfoLog::load_with(|s| opengl32::get_proc_address(s));
    gl::GetShaderiv::load_with(|s| opengl32::get_proc_address(s));


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

    {
        let frag_pointer: *const u8 = &FRAGMENT_SOURCE[0];
        let frag_size: i32 = FRAGMENT_SOURCE.len() as i32;
        unsafe { gl::ShaderSource(fragment, 1, &frag_pointer, &frag_size) };
        unsafe { gl::CompileShader(fragment) };

        let mut frag_status = 0;
        unsafe { gl::GetShaderiv(fragment, gl::COMPILE_STATUS, &mut frag_status) };

        if frag_status == 0 {
            print_shader_error(fragment);
            panic!();
        }
    }

    {
        let vert_pointer: *const u8 = &VERTEX_SOURCE[0];
        let vert_size: i32 = VERTEX_SOURCE.len() as i32;
        unsafe { gl::ShaderSource(vertex, 1, &vert_pointer, &vert_size) };
        unsafe { gl::CompileShader(vertex) };

        let mut vert_status = 0;
        unsafe { gl::GetShaderiv(vertex, gl::COMPILE_STATUS, &mut vert_status) };

        if vert_status == 0 {
            print_shader_error(vertex);
            panic!();
        }
    }

    {
        unsafe { gl::AttachShader(program, fragment) };
        unsafe { gl::AttachShader(program, vertex) };

        unsafe { gl::LinkProgram(program) };

        let mut program_status = 0;
        unsafe { gl::GetProgramiv(program, gl::LINK_STATUS, &mut program_status) };

        if program_status == 0 {
            print_program_error(program);
            panic!();
        }
    }

    {
        unsafe { gl::ValidateProgram(program) };

        let mut program_valid = 0;
        unsafe { gl::GetProgramiv(program, gl::VALIDATE_STATUS, &mut program_valid) };

        if program_valid == 0 {
            panic!();
        }
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
