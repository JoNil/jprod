#![feature(lang_items)]
#![feature(link_args)]
#![feature(link_llvm_intrinsics)]

#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(feature = "use_std"), no_std)]

// TODO:
// Debug camera
// Profiling? Gpu and cpu time. Telemetry?
// Defered rendering
// Dna shaped rombs
// Imgui
// Camera path
// Audio output
// Square wave

// Optimizations
// Load kernal32 stuff by ordinal

#[cfg_attr(not(test), link_args = "/SUBSYSTEM:WINDOWS /EXPORT:NvOptimusEnablement /FIXED /FORCE")]
extern "C" {}

extern crate rlibc;

#[cfg(feature = "use_std")]
mod core {
    pub use std::*;
}

mod c_types;
mod camera;
mod file;
mod gl;
mod mat4;
mod math;
mod mesh;
mod module;
mod pool;
mod random;
mod shader;
mod shader_sources;
mod ssbo;
mod time;
mod utils;
mod vec4;
mod win32;
mod win32_types;
mod window;

use camera::Camera;
use mat4::Mat4;
use mesh::Mesh;
use pool::Pool;
use pool::PoolAllocator;
use random::Rng;
use shader::Shader;
use shader_sources::ShaderId;
use ssbo::Ssbo;
use window::Window;

fn update_instance_data<'a>(instance_data: &mut Ssbo, pool: &mut PoolAllocator<'a>, rng: &mut Rng, x: f32, y: f32) {

    let allocator = pool.get_sub_allocator();

    let mvps = allocator.allocate_slice::<[[f32; 4]; 4]>(5000);

    for mvp in mvps.iter_mut() {

        unsafe {
            (*mvp.get_unchecked_mut(0)) = [0.01, 0.0, 0.0, 0.0];
            (*mvp.get_unchecked_mut(1)) = [0.0, 0.01, 0.0, 0.0];
            (*mvp.get_unchecked_mut(2)) = [0.0, 0.0, 0.01, 0.0];
            (*mvp.get_unchecked_mut(3)) = [rng.next_f32() - 0.5 + x, rng.next_f32() - 0.5 + y, 0.0, 1.0];
        }
     }

    instance_data.upload_slice(mvps);
}

#[derive(Copy, Clone)]
struct Uniforms {
    time: f32,
    vp: Mat4,
}

fn main() {

    let mut pool = Pool::new(256 * 1024 * 1024);
    let mut allocator = pool.get_allocator();

    let mut rng = Rng::new_unseeded();

    let mut window = Window::new();

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

    let mut x = 0.0;
    let mut y = 0.0;

    let mut uniforms = Uniforms { time: 0.0, vp: Mat4::identity() };

    let camera = Camera::new();

    loop {
        window.update();

        let actions = window.get_actions();

        if actions.forward.active {
            y += 0.02;
        }
        if actions.backward.active {
            y -= 0.02;
        }
        if actions.right.active {
            x += 0.02;
        }
        if actions.left.active {
            x -= 0.02;
        }

        let mouse = window.get_mouse_pos();
        let size = window.get_size();

        shader.reload_if_changed(&allocator);

        uniforms.time = (time::now_s() - start) as f32;
        
        uniform_data.upload(&uniforms);

        let mouse_offset = {

            let mouse_x = mouse.0 as f32;
            let mouse_y = mouse.1 as f32;
            let width = size.0 as f32;
            let height = size.1 as f32;

            (
                2.0 * mouse_x / width - 1.0,
                -2.0 * mouse_y / height + 1.0,
            )
        };

        update_instance_data(&mut instance_data, &mut allocator, &mut rng,
            mouse_offset.0 + x, mouse_offset.1 + y);

        unsafe { gl::ViewportIndexedf(0, 0.0, 0.0, size.0 as f32, size.1 as f32) };

        window.clear(if actions.left_mouse.active { [0.5, 0.0, 0.0, 1.0 ] } else { [0.0, 0.5, 0.0, 1.0 ] });

        mesh.draw_instanced(&shader, &instance_data, &uniform_data, 5000);

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

#[cfg(all(not(test), not(feature = "use_std")))]
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