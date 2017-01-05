#![feature(abi_vectorcall)]
#![feature(lang_items)]
#![feature(link_args)]
#![feature(link_llvm_intrinsics)]
#![feature(platform_intrinsics)]
#![feature(repr_simd)]
#![feature(simd_ffi)]

#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(feature = "use_std"), no_std)]

// TODO:
// More intressting dna snake :)
// Camera path
// Profiling? Gpu and cpu time. Telemetry?
// Defered rendering
// Imgui
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
mod gen;
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

const INSTANCE_COUNT: i32 = 100000;

fn update_instance_data<'a>(instance_data: &mut Ssbo, pool: &mut PoolAllocator<'a>, time: f32) {

    let allocator = pool.get_sub_allocator();

    let mvps = allocator.allocate_slice::<Mat4>(INSTANCE_COUNT as usize);

    let mut rng = Rng::new_unseeded();
    
    let b = 10.0;
    let a = 0.3;
    let f = 5.0 * b;
    let s = 0.01;
    let rs = 0.1;

    let len = mvps.len() / 2;
    let mut i = 0;
    let mut offset = 0.0;

    for mvp in mvps.iter_mut() {

        if i == len {
            i = 0;
            offset = 180.0;
        }

        let t = i as f32 / len as f32;

        let x = a * f32::cos(f*t);
        let z = a * f32::sin(f*t);
        let y = b * t - b / 2.0;

        let offset_x = rng.next_f32() * rs;
        let offset_y = rng.next_f32() * rs;
        let offset_z = rng.next_f32() * rs;

        *mvp =
            Mat4::rotate_deg(offset + 4.0 * time, Vec4::y()).mul(
            Mat4::translate(Vec4::xyz(x + offset_x, y + offset_y, z + offset_z))).mul(
            Mat4::random_rotation(&mut rng)).mul(
            Mat4::scale(s));

        i += 1;
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
    let mut window = Window::new();

    let mut shader = Shader::new(&window, ShaderId::First);
    let mut mesh = Mesh::new(&window);

    {
        let sub_allocator = allocator.get_sub_allocator();
        {
            let tetrahedron = gen::tetrahedron(&sub_allocator);
            mesh.upload(tetrahedron);
        }
    }

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

        let (dt, time) = {
            let now = time::now_s();
            let dt = last - now;
            last = now;

            (dt, (now - start) as f32)
        };

        camera.update(&window, dt as f32);

        update_instance_data(&mut instance_data, &mut allocator, time);

        uniforms.vp = camera.get_view_projection();
        uniforms.time = time;
        uniform_data.upload(&uniforms);

        window.update_viewport();
        window.clear([ 0.0, 0.5, 0.0, 1.0 ]);
        mesh.draw_instanced(&shader, &instance_data, &uniform_data, INSTANCE_COUNT);
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