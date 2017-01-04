#![feature(lang_items)]
#![feature(link_args)]
#![feature(link_llvm_intrinsics)]
#![feature(platform_intrinsics)]
#![feature(repr_simd)]
#![feature(simd_ffi)]

#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(feature = "use_std"), no_std)]

// TODO:
// Dna shaped rombs
// Profiling? Gpu and cpu time. Telemetry?
// Defered rendering
// Imgui
// Camera path
// Audio output
// Square wave
// Emulate gameboy sound chip?

// Optimizations
// Load kernal32 stuff by ordinal

#[cfg_attr(not(test), link_args = "/SUBSYSTEM:WINDOWS /EXPORT:NvOptimusEnablement /FIXED /FORCE")]
extern "C" {}

extern crate rt;

#[cfg(feature = "use_std")]
mod core {
    pub use std::*;
}

mod c_types;
mod camera;
mod f32;
mod file;
mod gl;
mod intrinsics;
mod mat4;
mod mesh;
mod pool;
mod random;
mod shader;
mod shader_sources;
mod simdty;
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
use vec4::Vec4;
use window::Window;

fn update_instance_data<'a>(instance_data: &mut Ssbo, pool: &mut PoolAllocator<'a>, rng: &mut Rng) {

    let allocator = pool.get_sub_allocator();

    let mvps = allocator.allocate_slice::<Mat4>(5000);

    let a = 0.3;
    let b = 1.0;
    let f = 10.0;
    let len = mvps.len();

    for (i, mvp) in mvps.iter_mut().enumerate() {

        let t = i as f32 / len as f32;

        let x = a * f32::cos(f*t);
        let z = a * f32::sin(f*t);
        let y = b * t;

        *mvp = Mat4::translate(Vec4::xyz(x, y, z)) * Mat4::scale(0.05, 0.05, 0.05);
     }

    instance_data.upload_slice(mvps);
}

#[derive(Copy, Clone)]
struct Uniforms {
    vp: Mat4,
    time: f32,
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
        [  1.0, -1.0, 0.0  ],
    ];

    mesh.upload(&triangle);

    let mut instance_data = Ssbo::new(&window);
    let mut uniform_data = Ssbo::new(&window);

    let start = time::now_s();
    let mut last = start;

    let mut uniforms = Uniforms {
        vp: Mat4::identity(),
        time: 0.0,
    };

    let mut camera = Camera::new(&window);

    loop {
        window.update();

        shader.reload_if_changed(&allocator);

        let dt = {
            let now = time::now_s();
            let dt = last - now;
            last = now;
            dt
        };

        camera.update(&window, dt as f32);

        update_instance_data(&mut instance_data, &mut allocator, &mut rng);

        uniforms.vp = camera.get_view_projection();
        uniforms.time = (time::now_s() - start) as f32;
        uniform_data.upload(&uniforms);

        window.update_viewport();
        window.clear([ 0.0, 0.5, 0.0, 1.0 ]);
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

    utils::debug_trap();
}

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static NvOptimusEnablement: i32 = 1;