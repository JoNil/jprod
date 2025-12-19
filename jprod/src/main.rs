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
unsafe extern "C" {}

extern crate jprod_core;

use jprod_core::{
    camera::Camera,
    generate,
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
    time, utils, win32,
    window::Window,
};

mod shaders;

const INSTANCE_COUNT: i32 = 200_000;

fn update_instance_data(instance_data: &mut Ssbo, pool: &mut Pool, time: f32) -> (Vec4, Vec4) {
    let mut mvps = pool.allocate_array::<Mat4>(INSTANCE_COUNT as usize, Mat4::identity());
    let mvps = pool.borrow_slice_mut(&mut mvps);

    let mut rng = Rng::new_unseeded();

    let b = 10.0;
    let a = 0.3;
    let f = 5.0 * b;
    let s = 0.002;
    let rs = 0.1;

    let len = mvps.len() / 2;
    let mut i = 0;
    let mut offset = 0.0;

    // Sequence timeline:
    // 0-10s: Double Helix
    // 10-15s: Morph to Galaxy
    // 15-30s: Galaxy Tour

    let morph_start = 5.0;
    let morph_duration = 5.0;
    let morph = math::clamp((time - morph_start) / morph_duration, 0.0, 1.0);
    // Smoothstep for nicer transition
    let morph = morph * morph * (3.0 - 2.0 * morph);

    // Camera animation
    let cam_start_pos = Vec4::xyz(0.0, 0.0, 1.0);
    let cam_start_look = Vec4::xyz(0.0, 0.0, 0.0);

    let tour_start = 12.0;
    let tour_duration = 15.0;
    let tour_t = math::clamp((time - tour_start) / tour_duration, 0.0, 1.0);
    let tour_t = tour_t * tour_t * (3.0 - 2.0 * tour_t); // Smoothstep

    let mut cam_pos = cam_start_pos;
    let mut cam_look_at = cam_start_look;

    if time > tour_start {
        // Spiral path around the galaxy
        // Ensure integer number of rotations to end at same angle
        // 2.0 * 2.0 * PI = 4.0 * PI = 2 full rotations
        let angle = tour_t * math::PI * 4.0;
        let start_radius = 1.5;
        let end_radius = 2.0;
        let radius = start_radius + tour_t * (end_radius - start_radius);

        // Start height at 0 to align with initial pos, then oscillate
        let height = 1.5 * math::sin(tour_t * math::PI);

        // We want to start at angle 0 (cos=1, sin=0) to align with (0,0,1)
        // Wait, (0,0,1) corresponds to angle PI/2 in sin/cos terms if mapped to Z/X?
        // Let's check: Vec4::xyz(cos_a * radius, height, sin_a * radius);
        // If we want result (0, 0, 1), we need cos=0, sin=1 => angle = PI/2

        let start_angle = math::PI / 2.0;
        let current_angle = start_angle + angle;

        let (sin_a, cos_a) = math::sin_cos(current_angle);

        // Note: Mapping to match Vec4::xyz(0,0,1) at start
        // X = cos(PI/2) * 1 = 0
        // Z = sin(PI/2) * 1 = 1
        cam_pos = Vec4::xyz(cos_a * radius, height, sin_a * radius);

        // Look slightly off center
        cam_look_at = Vec4::xyz(0.0, 0.0, 0.0);
    }

    // Core parameters (Strand 1 -> Core)
    let core_radius = 0.3;

    // Disk parameters (Strand 2 -> Disk)
    let disk_radius_min = 0.0;
    let disk_radius_max = 4.0;
    let disk_height = 0.1;
    let spiral_arms = 3.0;
    let spiral_tightness = 2.0;

    let golden_angle = math::PI * (3.0 - math::sqrt(5.0));

    for mvp in mvps.iter_mut() {
        if i == len {
            i = 0;
            offset = 180.0;
        }

        // Fade offset (180 degrees) towards 0 as we morph to galaxy
        let current_offset = offset * (1.0 - morph);

        let t = i as f32 / len as f32;

        // Helix
        let (sin_ft, cos_ft) = math::sin_cos(f * t);

        let h_x = a * cos_ft;
        let h_z = a * sin_ft;
        let h_y = b * t - b / 2.0;

        // Galaxy
        let idx = i as f32; // Reset index per strand
        let strand_len = len as f32;

        let g_x;
        let g_y;
        let g_z;

        if offset == 0.0 {
            // First strand -> Spherical Core (Fibonacci Sphere)
            // Reversed direction: starts at -1.0 (bottom) and goes to 1.0 (top)
            let z_s = (2.0 * idx + 1.0) / strand_len - 1.0;
            let r_s = math::sqrt(1.0 - z_s * z_s);
            let phi = idx * golden_angle;

            let (sin_phi, cos_phi) = math::sin_cos(phi);
            g_x = cos_phi * r_s * core_radius;
            g_y = z_s * core_radius;
            g_z = sin_phi * r_s * core_radius;
        } else {
            // Second strand -> Flattened Disk (Spiral Galaxy)
            let radius = disk_radius_min + (disk_radius_max - disk_radius_min) * rng.next_f32();
            let angle =
                radius * spiral_tightness + (idx / strand_len) * math::PI * 2.0 * spiral_arms;

            // Add some thickness/height variation
            let h = (rng.next_f32() - 0.5) * disk_height;
            // Density falls off with radius
            let falloff = 1.0 - (radius - disk_radius_min) / (disk_radius_max - disk_radius_min);
            let height_scale = h * falloff;

            let (sin_a, cos_a) = math::sin_cos(angle);
            g_x = cos_a * radius;
            g_y = height_scale;
            g_z = sin_a * radius;
        }

        // Center the random offset
        let offset_x = (rng.next_f32() - 0.5) * rs;
        let offset_y = (rng.next_f32() - 0.5) * rs;
        let offset_z = (rng.next_f32() - 0.5) * rs;

        let x = h_x * (1.0 - morph) + g_x * morph + offset_x;
        let y = h_y * (1.0 - morph) + g_y * morph + offset_y;
        let z = h_z * (1.0 - morph) + g_z * morph + offset_z;

        *mvp = Mat4::rotate_deg(current_offset + 4.0 * time, Vec4::xyz(0.0, 1.0, 0.0))
            .mult(Mat4::translate(Vec4::xyz(x, y, z)))
            .mult(Mat4::random_rotation(&mut rng))
            .mult(Mat4::scale(s));

        i += 1;
    }

    instance_data.upload_slice(mvps);
    (cam_pos, cam_look_at)
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
struct Light {
    pos: Vec4,
    color: Vec4,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct LightUniforms {
    eye_pos: Vec4,
    light_pos: Vec4,
    light_color: Vec4,
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
    let mut light_pso = Pso::new();
    light_pso.blending = Some(jprod_core::gfx::pso::Blending::Additive);

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
        let (tetrahedron_pos, tetrahedron_normals) = generate::tetrahedron(&mut pool);

        let tetrahedron_pos = pool.borrow_slice(&tetrahedron_pos);
        let tetrahedron_normals = pool.borrow_slice(&tetrahedron_normals);

        dna_mesh.upload(tetrahedron_pos, tetrahedron_normals, Primitive::Triangles);

        let (quad_pos, quad_normals) = generate::quad(&mut pool);

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

        let (cam_pos, cam_look_at) = update_instance_data(&mut instance_data, &mut pool, time);

        camera.set_pos(cam_pos);
        camera.look_at(cam_look_at);

        uniform_data.upload(&Uniforms {
            vp: camera.get_view_projection(),
            time,
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

        light_target.clear(Vec4::xyzw(0.0, 0.0, 0.0, 1.0));

        let lights = [
            // Original red light
            Light {
                pos: Vec4::xyz(0.0, 100.0, 100.0),
                color: Vec4::xyz(0.9, 0.1, 0.1),
            },
            // Blue accent light
            Light {
                pos: Vec4::xyz(-50.0, 50.0, 100.0),
                color: Vec4::xyz(0.1, 0.1, 0.8),
            },
        ];

        for light in lights.iter() {
            light_uniform_data.upload(&LightUniforms {
                eye_pos: camera.get_camera_pos(),
                light_pos: light.pos,
                light_color: light.color,
            });

            quad_mesh.draw(
                &light_pso,
                &light_shader,
                Some(&light_target),
                &[
                    g_buffer.get_texture(0),
                    g_buffer.get_texture(1),
                    g_buffer.get_texture(2),
                ],
                Some(&light_uniform_data),
            );
        }

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
            plane_in_focus: camera.get_camera_pos().length() / 2.0,
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
#[unsafe(no_mangle)]
pub extern "system" fn WinMainCRTStartup() {
    main();
}

#[allow(non_snake_case)]
#[cfg(not(test))]
#[unsafe(no_mangle)]
pub extern "system" fn mainCRTStartup() {
    main();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    utils::debug_trap();
}

#[allow(non_upper_case_globals)]
#[cfg(not(test))]
#[unsafe(no_mangle)]
pub static NvOptimusEnablement: i32 = 1;

#[allow(non_upper_case_globals)]
#[cfg(not(test))]
#[unsafe(no_mangle)]
pub static _fltused: i32 = 1;
