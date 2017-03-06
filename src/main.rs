#![feature(abi_vectorcall)]
#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(lang_items)]
#![feature(link_args)]
#![feature(link_llvm_intrinsics)]
#![feature(platform_intrinsics)]
#![feature(pub_restricted)]
#![feature(repr_simd)]
#![feature(simd_ffi)]

#![cfg_attr(all(not(test), not(feature = "use_std")), no_main)]
#![cfg_attr(not(feature = "use_std"), no_std)]

// TODO:
// Defered rendering
//  - Light shader: http://www.codinglabs.net/article_physically_based_rendering_cook_torrance.aspx
//  - Light list
// More intressting dna snake :)
// Camera path
// Audio output
// Square wave

// Optimizations
// Load kernal32 stuff by ordinal

#[cfg_attr(all(not(test), not(feature = "use_std")), link_args = "/SUBSYSTEM:WINDOWS /EXPORT:NvOptimusEnablement /FIXED vcruntime.lib libcmt.lib")]
extern "C" {}

extern crate compiler_builtins;

#[cfg(feature = "use_telemetry")]
extern crate telemetry;

#[macro_use]
extern crate telemetry_macro;

#[cfg(feature = "use_std")]
mod core {
    pub use std::*;
}

mod c_types;
mod camera;
mod file;
mod g_buffer;
mod gen;
mod gfx;
mod intrinsics;
mod math;
mod pool;
mod random;
mod shader_sources;
mod simdty;
mod time;
mod utils;
mod win32;
mod window;

use camera::Camera;
use g_buffer::GBuffer;
use gfx::mesh::Mesh;
use gfx::mesh::Primitive;
use gfx::querys::QueryManager;
use gfx::shader::Shader;
use gfx::ssbo::Ssbo;
use math::Mat4;
use math::Vec4;
use pool::Pool;
use pool::PoolAllocator;
use random::Rng;
use shader_sources::ShaderId;
use window::Window;

const INSTANCE_COUNT: i32 = 40_000;

fn update_instance_data<'a>(instance_data: &mut Ssbo, pool: &mut PoolAllocator<'a>, time: f32) {

    tm_zone!("update_instance_data");

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

        let (sin_ft, cos_ft) = math::sin_cos(f*t);

        let x = a * cos_ft;
        let z = a * sin_ft;
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

    tm_init!(
        b"JProd\0",
        win32::load_library,
        win32::get_proc_address,
        allocator.allocate_slice(32 * 1024 * 1024));

    let mut window = Window::new();

    let mut query_manager = QueryManager::new(&window);

    let mut shader = Shader::new(&window, ShaderId::First);
    let mut mesh = Mesh::new(&window, Primitive::Triangles);

    let mut quad_shader = Shader::new(&window, ShaderId::Passthrough);
    let mut quad_mesh = Mesh::new(&window, Primitive::TriangleStrip);

    let mut g_buffer = GBuffer::new(&window);

    {
        let sub_allocator = allocator.get_sub_allocator();
        
        let (tetrahedron_pos, tetrahedron_normals) = gen::tetrahedron(&sub_allocator);
        mesh.upload(tetrahedron_pos, tetrahedron_normals);

        let (quad_pos, quad_normals) = gen::quad(&sub_allocator);
        quad_mesh.upload(quad_pos, quad_normals);
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
        quad_shader.reload_if_changed(&allocator);

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

        g_buffer.clear();
        mesh.draw_instanced(
            &shader,
            &query_manager,
            Some(g_buffer.get_framebuffer()),
            &uniform_data,
            &instance_data,
            INSTANCE_COUNT);

        window.update_viewport();
        window.clear(&[ 0.0, 0.0, 0.0, 1.0 ]);
        quad_mesh.draw(
            &quad_shader,
            &query_manager,
            &uniform_data,
            &[g_buffer.get_color_texture(), g_buffer.get_pos_texture(),  g_buffer.get_normal_texture()]);
        window.swap();

        query_manager.submit_zones();

        utils::assert(!gfx::is_error(&window));

        tm_tick!();
    }
}

#[cfg(all(not(test), not(feature = "use_std")))]
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