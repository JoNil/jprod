#![cfg_attr(not(test), no_main)]
#![feature(lang_items)]
#![feature(link_args)]

#![no_std]

// TODO:
// Load kernal32 stuff by ordinal
// Figure out a way to test -C relocation-model=static

#[cfg_attr(not(test), link_args = "/SUBSYSTEM:WINDOWS /EXPORT:NvOptimusEnablement /FIXED /FORCE")]
extern "C" {}

extern crate rlibc;

mod c_types;
mod file;
mod gl;
mod mesh;
mod module;
mod pool;
mod random;
mod shader;
mod shader_sources;
mod ssbo;
mod time;
mod utils;
mod win32;
mod win32_types;
mod window;

use core::mem;
use core::slice;
use mesh::Mesh;
use pool::Pool;
use pool::PoolAllocator;
use random::Generator;
use shader::Shader;
use shader_sources::ShaderId;
use ssbo::Ssbo;
use window::Window;

fn update_instance_data<'a>(instance_data: &mut Ssbo, pool: &mut PoolAllocator<'a>, rng: &mut Generator) {

    let allocator = pool.get_sub_allocator();

    let buffer = allocator.allocate(100 * mem::size_of::<[[f32; 4]; 4]>());

    let mvps: &mut [[[f32; 4]; 4]] = unsafe { slice::from_raw_parts_mut(&mut *buffer.get_unchecked_mut(0) as *mut u8 as *mut _, buffer.len() / mem::size_of::<[[f32; 4]; 4]>()) };

    for mvp in mvps.iter_mut() {

        unsafe {
            (*mvp.get_unchecked_mut(0)) = [0.2, 0.0, 0.0, 0.0];
            (*mvp.get_unchecked_mut(1)) = [0.0, 0.2, 0.0, 0.0];
            (*mvp.get_unchecked_mut(2)) = [0.0, 0.0, 0.2, 0.0];
            (*mvp.get_unchecked_mut(3)) = [rng.next_f32() - 0.5, rng.next_f32() - 0.5, 0.0, 1.0];
        }
     }

    instance_data.upload_slice(mvps);
}

fn main() {

    let mut pool = Pool::new(256 * 1024 * 1024);
    let mut allocator = pool.get_allocator();

    let mut rng = Generator::new_unseeded();

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

    let mut uniform_data = Ssbo::new(&window);

    let start = time::now_s();

    loop {
        window.process_messages();

        // win32::message_box(b"Frame\0", b"Frame\0", 0);

        shader.reload_if_changed(&allocator);

        let time = (time::now_s() - start) as f32;
        uniform_data.upload(&time);

        update_instance_data(&mut instance_data, &mut allocator, &mut rng);

        window.clear();

        mesh.draw_instanced(&shader, &instance_data, &uniform_data, 100);

        window.swap();
    }
}

#[cfg(not(test))]
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn WinMainCRTStartup() {

    win32::init();

    main();

    win32::exit_process(0);
}

#[cfg(not(test))]
#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(_msg: core::fmt::Arguments,
                                   _file: &'static str,
                                   _line: u32)
                                   -> ! {

    win32::debug_break();
}

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static NvOptimusEnablement: i32 = 1;