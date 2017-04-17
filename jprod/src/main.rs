#![feature(abi_vectorcall)]
#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(lang_items)]
#![feature(link_args)]
#![feature(link_llvm_intrinsics)]
#![feature(platform_intrinsics)]
#![feature(repr_simd)]
#![feature(simd_ffi)]

#![cfg_attr(all(not(test), not(feature = "use_std")), no_main)]
#![cfg_attr(not(feature = "use_std"), no_std)]

// TODO:
// Fix normal not being rotated
// Defered rendering
//  - Other light model
//    * http://blog.selfshadow.com/sandbox/specaa.html
//    * https://github.com/wdas/brdf
//  - Area lights
//    * http://advances.realtimerendering.com/s2016/s2016_ltc_rnd.pdf
//    * https://eheitzresearch.wordpress.com/415-2/
//  - Light shader: Cook-Torrence and GGX
//    * https://learnopengl.com/#!PBR/Theory
//    * http://ruh.li/GraphicsCookTorrance.html
//    * http://www.codinglabs.net/article_physically_based_rendering_cook_torrance.aspx
//    * https://renderman.pixar.com/view/cook-torrance-shader
// Rocket interop
// Dof
//   - http://http.developer.nvidia.com/GPUGems3/gpugems3_ch28.html
//   - http://http.developer.nvidia.com/GPUGems/gpugems_ch23.html
//   - http://www.crytek.com/download/Sousa_Graphics_Gems_CryENGINE3.pdf
//   - http://ivizlab.sfu.ca/papers/cgf2012.pdf
//   - 
// AA
//   - https://github.com/playdeadgames/temporal
//   - https://timothylottes.github.io/20110403.html
// Audio output
// Square wave
// More intressting dna snake :)
// Camera path

// Cleanup
// Remove query_manager from all draw calls

// Inspiration
// Doom: http://www.adriancourreges.com/blog/2016/09/09/doom-2016-graphics-study/

// Optimizations
// Downsample every bloom blur pass


#[cfg_attr(all(not(test), not(feature = "use_std"), target_pointer_width = "64"), link_args = "/SUBSYSTEM:WINDOWS /EXPORT:NvOptimusEnablement /FIXED ../lib/msvcrt-light-x64.lib libcmt.lib vcruntime.lib")]
#[cfg_attr(all(not(test), not(feature = "use_std"), target_pointer_width = "32"), link_args = "/SUBSYSTEM:WINDOWS /EXPORT:NvOptimusEnablement /FIXED ../lib/msvcrt-light.lib libcmt.lib vcruntime.lib")]
extern "C" {}

#[cfg(all(not(test), not(feature = "use_std")))]
extern crate compiler_builtins;

#[cfg(feature = "use_telemetry")]
extern crate telemetry;

#[macro_use]
extern crate telemetry_macro;

extern crate jprod_core;

#[cfg(feature = "use_std")]
mod core {
    pub use std::*;
}

use jprod_core::camera::Camera;
use jprod_core::gen;
use jprod_core::gfx::mesh::Mesh;
use jprod_core::gfx::mesh::Primitive;
use jprod_core::gfx::querys::QueryManager;
use jprod_core::gfx::shader::Shader;
use jprod_core::gfx::ssbo::Ssbo;
use jprod_core::gfx::target::Target;
use jprod_core::gfx::texture::Format;
use jprod_core::gfx;
use jprod_core::math::Mat4;
use jprod_core::math::Vec4;
use jprod_core::math;
use jprod_core::pool::Pool;
use jprod_core::pool::PoolAllocator;
use jprod_core::random::Rng;
use jprod_core::shaders::ShaderId;
use jprod_core::time;
use jprod_core::utils;
use jprod_core::win32;
use jprod_core::window::Window;

const INSTANCE_COUNT: i32 = 40_000;

fn update_instance_data(instance_data: &mut Ssbo, pool: &mut PoolAllocator, time: f32) {

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

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct Uniforms {
    vp: Mat4,
    time: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct LightUniforms {
    eye_pos: Vec4,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct DofUniforms {
    z_near: f32,
    z_far: f32,
    plane_in_focus: f32,
}

fn main() {

    win32::init();

    let mut pool = Pool::new(256 * 1024 * 1024);
    let mut allocator = pool.get_allocator();

    tm_init!(
        b"JProd\0",
        win32::load_library,
        win32::get_proc_address,
        allocator.allocate_slice(32 * 1024 * 1024));

    let mut window = Window::new();
    let mut camera = Camera::new(&window);
    let mut query_manager = QueryManager::new(&window);

    let dna_shader = Shader::new(&window, ShaderId::Dna);
    let light_shader = Shader::new(&window, ShaderId::Light);
    let bloom_extraction_shader = Shader::new(&window, ShaderId::BloomExtraction);
    let bloom_resolv_shader = Shader::new(&window, ShaderId::BloomResolv);
    let horizontal_blur = Shader::new(&window, ShaderId::HorizontalGaussianBlur);
    let vertical_blur = Shader::new(&window, ShaderId::VerticalGaussianBlur);
    let dof_extraction_shader = Shader::new(&window, ShaderId::DofExtraction);
    let dof_far_blur_shader = Shader::new(&window, ShaderId::DofFarBlur);
    let dof_far_blur_max_shader = Shader::new(&window, ShaderId::DofFarBlurMax);
    let dof_merge_shader = Shader::new(&window, ShaderId::DofMerge);

    let window_size = window.get_size();
    let mut g_buffer = Target::new(&window, window_size, &[Some(Format::RgbR11G11B10), Some(Format::RgbF16), Some(Format::RgbF16)], true);
    let mut light_target = Target::new(&window, window_size, &[Some(Format::RgbR11G11B10), None, None], false);
    let mut bloom_blur1 = Target::new(&window, (window_size.0/2, window_size.1/2), &[Some(Format::RgbR11G11B10), None, None], false);
    let mut bloom_blur2 = Target::new(&window, (window_size.0/2, window_size.1/2), &[Some(Format::RgbR11G11B10), None, None], false);
    let mut bloom_merge_target = Target::new(&window, window_size, &[Some(Format::RgbR11G11B10), None, None], false);
    let mut dof_extracted_target = Target::new(&window, (window_size.0/2, window_size.1/2), &[Some(Format::RgbaF16), None, None], false);
    let mut dof_far_blur_target = Target::new(&window, (window_size.0/2, window_size.1/2), &[Some(Format::RgbaF16), None, None], false);

    let mut instance_data = Ssbo::new(&window);
    let mut uniform_data = Ssbo::new(&window);
    let mut light_uniform_data = Ssbo::new(&window);
    let mut dof_uniform_data = Ssbo::new(&window);

    let mut dna_mesh = Mesh::new(&window, Primitive::Triangles);
    let mut quad_mesh = Mesh::new(&window, Primitive::TriangleStrip);

    {
        let sub_allocator = allocator.get_sub_allocator();
        
        let (tetrahedron_pos, tetrahedron_normals) = gen::tetrahedron(&sub_allocator);
        dna_mesh.upload(tetrahedron_pos, tetrahedron_normals);

        let (quad_pos, quad_normals) = gen::quad(&sub_allocator);
        quad_mesh.upload(quad_pos, quad_normals);
    }

    let start = time::now_s();
    let mut last = start;

    loop {
        window.update();

        let (dt, time) = {
            let now = time::now_s();
            let dt = last - now;
            last = now;

            (dt, (now - start) as f32)
        };

        camera.update(&window, dt as f32);

        update_instance_data(&mut instance_data, &mut allocator, time);

        uniform_data.upload(&Uniforms {
            vp: camera.get_view_projection(),
            time: time,
        });

        g_buffer.clear(Vec4::xyzw(0.0, 0.0, 0.0, 1.0));
        dna_mesh.draw_instanced(
            &dna_shader,
            &query_manager,
            Some(&g_buffer),
            &uniform_data,
            &instance_data,
            INSTANCE_COUNT);

        light_uniform_data.upload(&LightUniforms {
            eye_pos: camera.get_camera_pos(),
        });

        light_target.clear(Vec4::xyzw(0.0, 0.0, 0.0, 1.0));
        quad_mesh.draw(
            &light_shader,
            &query_manager,
            Some(&light_target),
            Some(&light_uniform_data),
            &[g_buffer.get_texture(0), g_buffer.get_texture(1), g_buffer.get_texture(2)]);

        bloom_blur1.clear(Vec4::xyzw(0.0, 0.0, 0.0, 1.0));
        bloom_blur2.clear(Vec4::xyzw(0.0, 0.0, 0.0, 1.0));
        quad_mesh.draw(
            &bloom_extraction_shader,
            &query_manager,
            Some(&bloom_blur2),
            None,
            &[light_target.get_texture(0)]);

        for _ in 0..5 {
            quad_mesh.draw(
                &horizontal_blur,
                &query_manager,
                Some(&bloom_blur1),
                None,
                &[bloom_blur2.get_texture(0)]);
            quad_mesh.draw(
                &vertical_blur,
                &query_manager,
                Some(&bloom_blur2),
                None,
                &[bloom_blur1.get_texture(0)]);
        }

        bloom_merge_target.clear(Vec4::xyzw(0.0, 0.0, 0.0, 1.0));
        quad_mesh.draw(
            &bloom_resolv_shader,
            &query_manager,
            Some(&bloom_merge_target),
            None,
            &[light_target.get_texture(0), bloom_blur2.get_texture(0)]);

        dof_uniform_data.upload(&DofUniforms {
            z_near: camera.get_near(),
            z_far: camera.get_far(),
            plane_in_focus: 0.5,
        });

        dof_extracted_target.clear(Vec4::xyzw(0.0, 0.0, 0.0, 1.0));
        quad_mesh.draw(
            &dof_extraction_shader,
            &query_manager,
            Some(&dof_extracted_target),
            Some(&dof_uniform_data),
            &[bloom_merge_target.get_texture(0), g_buffer.get_depth_texture()]);


        dof_far_blur_target.clear(Vec4::xyzw(0.0, 0.0, 0.0, 1.0));
        quad_mesh.draw(
            &dof_far_blur_shader,
            &query_manager,
            Some(&dof_far_blur_target),
            None,
            &[dof_extracted_target.get_texture(0)]);

        dof_extracted_target.clear(Vec4::xyzw(0.0, 0.0, 0.0, 1.0));
        quad_mesh.draw(
            &dof_far_blur_max_shader,
            &query_manager,
            Some(&dof_extracted_target),
            None,
            &[dof_far_blur_target.get_texture(0)]);

        window.update_viewport();
        window.clear(&[ 0.0, 0.0, 0.0, 1.0 ]);
        quad_mesh.draw(
            &dof_merge_shader,
            &query_manager,
            None,
            Some(&dof_uniform_data),
            &[bloom_merge_target.get_texture(0), dof_extracted_target.get_texture(0), g_buffer.get_depth_texture()]);
        window.swap();

        query_manager.submit_zones();

        utils::assert(!gfx::is_error(&window));

        tm_tick!();
    }
}

#[cfg(all(not(test), not(feature = "use_std"), target_pointer_width = "64"))]
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn WinMainCRTStartup() {
    main();
}

#[cfg(all(not(test), not(feature = "use_std"), target_pointer_width = "32"))]
#[allow(non_snake_case)]
#[no_mangle]
pub extern "cdecl" fn WinMainCRTStartup() {
    main();
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
