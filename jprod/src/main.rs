#![windows_subsystem = "windows"]
#![no_main]
#![no_std]

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

// Inspiration
// Doom: http://www.adriancourreges.com/blog/2016/09/09/doom-2016-graphics-study/

// Optimizations
// Downsample every bloom blur pass

#[link(name = "lib/msvcrt-light-x64", kind = "static")]
extern "C" {}

extern crate jprod_core;

use core::panic::PanicInfo;
use jprod_core::{
    camera::Camera,
    gen,
    gfx::{
        self,
        mesh::{Mesh, Primitive},
        pso::Pso,
        shader::Shader,
        ssbo::Ssbo,
        target::Target,
        texture::Format,
    },
    math::{self, Mat4, Vec4},
    pool::Pool,
    random::Rng,
    shaders, time, utils, win32,
    window::Window,
};

const INSTANCE_COUNT: i32 = 10_000;

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct Uniforms {
    vp: Mat4,
    time: Vec4,
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

    let mut pool = Pool::new(256 * 1024 * 1024, 0);

    let mut window = Window::new();
    let mut camera = Camera::new(&window);

    let pso = Pso::new();

    let dna_shader = Shader::from_source(&window, shaders::DNA_VERT, shaders::DNA_FRAG);
    let light_shader = Shader::from_source(&window, shaders::LIGHT_VERT, shaders::LIGHT_FRAG);
    let bloom_extraction_shader = Shader::from_source(
        &window,
        shaders::BLOOM_EXTRACTION_VERT,
        shaders::BLOOM_EXTRACTION_FRAG,
    );
    let bloom_resolv_shader = Shader::from_source(
        &window,
        shaders::BLOOM_RESOLV_VERT,
        shaders::BLOOM_RESOLV_FRAG,
    );
    let horizontal_blur = Shader::from_source(
        &window,
        shaders::HORIZONTAL_GAUSSIAN_BLUR_VERT,
        shaders::HORIZONTAL_GAUSSIAN_BLUR_FRAG,
    );
    let vertical_blur = Shader::from_source(
        &window,
        shaders::VERTICAL_GAUSSIAN_BLUR_VERT,
        shaders::VERTICAL_GAUSSIAN_BLUR_FRAG,
    );
    let dof_extraction_shader = Shader::from_source(
        &window,
        shaders::DOF_EXTRACTION_VERT,
        shaders::DOF_EXTRACTION_FRAG,
    );
    let dof_far_blur_shader = Shader::from_source(
        &window,
        shaders::DOF_FAR_BLUR_VERT,
        shaders::DOF_FAR_BLUR_FRAG,
    );
    let dof_far_blur_max_shader = Shader::from_source(
        &window,
        shaders::DOF_FAR_BLUR_MAX_VERT,
        shaders::DOF_FAR_BLUR_MAX_FRAG,
    );
    let dof_merge_shader =
        Shader::from_source(&window, shaders::DOF_MERGE_VERT, shaders::DOF_MERGE_FRAG);

    let window_size = window.get_size();
    let mut g_buffer = Target::new(
        &window,
        window_size,
        &[
            Some(Format::RgbR11G11B10),
            Some(Format::RgbF16),
            Some(Format::RgbF16),
        ],
        true,
    );
    let mut light_target = Target::new(
        &window,
        window_size,
        &[Some(Format::RgbR11G11B10), None, None],
        false,
    );
    let mut bloom_blur1 = Target::new(
        &window,
        (window_size.0 / 2, window_size.1 / 2),
        &[Some(Format::RgbR11G11B10), None, None],
        false,
    );
    let mut bloom_blur2 = Target::new(
        &window,
        (window_size.0 / 2, window_size.1 / 2),
        &[Some(Format::RgbR11G11B10), None, None],
        false,
    );
    let mut bloom_merge_target = Target::new(
        &window,
        window_size,
        &[Some(Format::RgbR11G11B10), None, None],
        false,
    );
    let mut dof_extracted_target = Target::new(
        &window,
        (window_size.0 / 2, window_size.1 / 2),
        &[Some(Format::RgbaF16), None, None],
        false,
    );
    let mut dof_far_blur_target = Target::new(
        &window,
        (window_size.0 / 2, window_size.1 / 2),
        &[Some(Format::RgbaF16), None, None],
        false,
    );

    let mut instance_data = Ssbo::new(&window);
    let mut uniform_data = Ssbo::new(&window);
    let mut light_uniform_data = Ssbo::new(&window);
    let mut dof_uniform_data = Ssbo::new(&window);

    let mut dna_mesh = Mesh::new(&window);
    let mut quad_mesh = Mesh::new(&window);

    {
        let (tetrahedron_pos, tetrahedron_normals) = gen::tetrahedron(&mut pool);

        let tetrahedron_pos = pool.borrow_slice(&tetrahedron_pos);
        let tetrahedron_normals = pool.borrow_slice(&tetrahedron_normals);

        dna_mesh.upload(tetrahedron_pos, tetrahedron_normals, Primitive::Triangles);

        let (quad_pos, quad_normals) = gen::quad(&mut pool);

        let quad_pos = pool.borrow_slice(&quad_pos);
        let quad_normals = pool.borrow_slice(&quad_normals);

        quad_mesh.upload(quad_pos, quad_normals, Primitive::TriangleStrip);
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

        uniform_data.upload(&Uniforms {
            vp: camera.get_view_projection(),
            time: Vec4::xyz(time, INSTANCE_COUNT as f32, 0.0),
        });

        g_buffer.clear(Vec4::xyzw(0.0, 0.0, 0.0, 1.0));
        dna_mesh.draw_instanced(
            &pso,
            &dna_shader,
            Some(&g_buffer),
            &[],
            Some(&uniform_data),
            Some(&instance_data),
            INSTANCE_COUNT,
        );

        light_uniform_data.upload(&LightUniforms {
            eye_pos: camera.get_camera_pos(),
        });

        light_target.clear(Vec4::xyzw(0.0, 0.0, 0.0, 1.0));
        quad_mesh.draw(
            &pso,
            &light_shader,
            Some(&light_target),
            &[
                g_buffer.get_texture(0),
                g_buffer.get_texture(1),
                g_buffer.get_texture(2),
            ],
            Some(&light_uniform_data),
        );

        bloom_blur1.clear(Vec4::xyzw(0.0, 0.0, 0.0, 1.0));
        bloom_blur2.clear(Vec4::xyzw(0.0, 0.0, 0.0, 1.0));
        quad_mesh.draw(
            &pso,
            &bloom_extraction_shader,
            Some(&bloom_blur2),
            &[light_target.get_texture(0)],
            None,
        );

        for _ in 0..5 {
            quad_mesh.draw(
                &pso,
                &horizontal_blur,
                Some(&bloom_blur1),
                &[bloom_blur2.get_texture(0)],
                None,
            );
            quad_mesh.draw(
                &pso,
                &vertical_blur,
                Some(&bloom_blur2),
                &[bloom_blur1.get_texture(0)],
                None,
            );
        }

        bloom_merge_target.clear(Vec4::xyzw(0.0, 0.0, 0.0, 1.0));
        quad_mesh.draw(
            &pso,
            &bloom_resolv_shader,
            Some(&bloom_merge_target),
            &[light_target.get_texture(0), bloom_blur2.get_texture(0)],
            None,
        );

        dof_uniform_data.upload(&DofUniforms {
            z_near: camera.get_near(),
            z_far: camera.get_far(),
            plane_in_focus: 0.5,
        });

        dof_extracted_target.clear(Vec4::xyzw(0.0, 0.0, 0.0, 1.0));
        quad_mesh.draw(
            &pso,
            &dof_extraction_shader,
            Some(&dof_extracted_target),
            &[
                bloom_merge_target.get_texture(0),
                g_buffer.get_depth_texture(),
            ],
            Some(&dof_uniform_data),
        );

        dof_far_blur_target.clear(Vec4::xyzw(0.0, 0.0, 0.0, 1.0));
        quad_mesh.draw(
            &pso,
            &dof_far_blur_shader,
            Some(&dof_far_blur_target),
            &[dof_extracted_target.get_texture(0)],
            None,
        );

        dof_extracted_target.clear(Vec4::xyzw(0.0, 0.0, 0.0, 1.0));
        quad_mesh.draw(
            &pso,
            &dof_far_blur_max_shader,
            Some(&dof_extracted_target),
            &[dof_far_blur_target.get_texture(0)],
            None,
        );

        window.update_viewport();
        window.clear(&[0.0, 0.0, 0.0, 1.0]);
        quad_mesh.draw(
            &pso,
            &dof_merge_shader,
            None,
            &[
                bloom_merge_target.get_texture(0),
                dof_extracted_target.get_texture(0),
                g_buffer.get_depth_texture(),
            ],
            Some(&dof_uniform_data),
        );
        window.swap();

        utils::assert(!gfx::is_error(&window));

        pool.clean();
    }
}

#[allow(non_snake_case)]
#[cfg(not(test))]
#[no_mangle]
pub extern "system" fn WinMainCRTStartup() {
    main();
}

#[allow(non_snake_case)]
#[cfg(not(test))]
#[no_mangle]
pub extern "system" fn mainCRTStartup() {
    main();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    utils::debug_trap();
}

#[allow(non_upper_case_globals)]
#[cfg(not(test))]
#[no_mangle]
pub static NvOptimusEnablement: i32 = 1;

#[allow(non_upper_case_globals)]
#[cfg(not(test))]
#[no_mangle]
pub static _fltused: i32 = 1;
