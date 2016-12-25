#![cfg_attr(not(test), no_main)]
#![feature(lang_items)]
#![feature(link_args)]
#![no_std]

#[cfg_attr(not(test), link_args = "/SUBSYSTEM:WINDOWS /EXPORT:NvOptimusEnablement /FIXED")]
extern "C" {}

extern crate rlibc;

#[macro_use]
mod win32_macros;

mod c_types;
mod file;
mod gdi32;
mod gl;
mod mesh;
mod module;
mod opengl32;
mod pool;
mod shader;
mod shader_sources;
mod ssbo;
mod utils;
mod win32;
mod time;
mod win32_types;
mod window;

use mesh::Mesh;
use pool::Pool;
use shader::Shader;
use shader_sources::ShaderId;
use ssbo::Ssbo;
use window::Window;

fn main() {

    let mut pool = Pool::new(256 * 1024 * 1024);
    let allocator = pool.get_allocator();

    let window = Window::new();

    let mut shader = Shader::new(&window, ShaderId::First);

    let mut mesh = Mesh::new(&window);

    let triangle = [
        [  0.0, 1.0, 0.0   ],
        [ -1.0, -1.0, 0.0  ],
        [  1.0, -1.0, 0.0  ]
    ];

    mesh.upload(&triangle);

    let mut instance_data = Ssbo::new(&window);

    let mvps: [[[f32; 4]; 4]; 4] = [
        [
            [0.2, 0.0, 0.0, 0.0],
            [0.0, 0.2, 0.0, 0.0],
            [0.0, 0.0, 0.2, 0.0],
            [0.5, 0.5, 0.0, 1.0],
        ],
        [
            [0.2, 0.0, 0.0, 0.0],
            [0.0, 0.2, 0.0, 0.0],
            [0.0, 0.0, 0.2, 0.0],
            [0.5, -0.5, 0.0, 1.0],
        ],
        [
            [0.2, 0.0, 0.0, 0.0],
            [0.0, 0.2, 0.0, 0.0],
            [0.0, 0.0, 0.2, 0.0],
            [-0.5, 0.5, 0.0, 1.0],
        ],
        [
            [0.2, 0.0, 0.0, 0.0],
            [0.0, 0.2, 0.0, 0.0],
            [0.0, 0.0, 0.2, 0.0],
            [-0.5, -0.5, 0.0, 1.0],
        ],
    ];

    instance_data.upload(&mvps);

    let mut uniform_data = Ssbo::new(&window);

    let start = time::now_s();

    loop {
        window.process_messages();

        // win32::message_box(b"Frame\0", b"Frame\0", 0);

        shader.reload_if_changed(&allocator);

        let time: [f32; 1] = [ (time::now_s() - start) as f32 ];
        uniform_data.upload(&time);

        window.clear();

        mesh.draw_instanced(&shader, &instance_data, &uniform_data, 4);

        window.swap();
    }
}

#[cfg(not(test))]
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn WinMainCRTStartup() {

    win32::init();
    gdi32::init();
    opengl32::init();

    main();

    win32::exit_process(0);
}

#[cfg(not(test))]
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[cfg(not(test))]
#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(_msg: core::fmt::Arguments,
                                   _file: &'static str,
                                   _line: u32)
                                   -> ! {

    win32::exit_process(1);
}

// #[cfg(not(test))]
// #[allow(non_snake_case)]
// #[no_mangle]
// pub extern "system" fn __CxxFrameHandler3(_: usize, _: usize, _: usize, _: usize) {
// win32::exit_process(1);
// }

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static NvOptimusEnablement: i32 = 1;