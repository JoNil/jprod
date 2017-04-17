use imgui;
use jprod_core::gfx::mesh::Mesh;
use jprod_core::gfx::mesh::Primitive;
use jprod_core::gfx::shader::Shader;
use jprod_core::gfx::texture::Format;
use jprod_core::gfx::texture::Texture;
use jprod_core::window::Window;
use std::error::Error;
use std::f32;

static VERTEX_SOURCE: &'static str = r##"
#version 450 core

layout(location = 0) in vec2 vertex_pos;
layout(location = 1) in vec2 vertex_uv;
layout(location = 2) in vec4 vertex_color;

out vec2 frag_uv;
out vec4 frag_color;

layout(std430, binding = 0) buffer uniforms
{
    mat4 mvp;
};

void main() {
    frag_uv = vertex_uv;
    frag_color = vertex_color / 255.0;
    gl_Position = mvp * vec4(vertex_pos.xy, 0.0, 1.0);
}
"##;

static FRAG_SOURCE: &'static str = r##"
#version 450 core

in vec2 frag_uv;
in vec4 frag_color;

layout(location = 0) out vec4 color;

layout(location = 2) uniform sampler2D tex;

void main() {
    color = frag_color * texture(tex, frag_uv);
}
"##;

pub struct ImGuiImpl {
    imgui: imgui::ImGui,
    shader: Shader,
    mesh: Mesh,
    texture: Texture,
}

impl ImGuiImpl {

    #[inline]
    pub fn new(window: &Window) -> ImGuiImpl {

        let mut imgui = imgui::ImGui::init();

        let shader = Shader::from_source(window, VERTEX_SOURCE, FRAG_SOURCE);

        let mesh = Mesh::new(window, Primitive::Triangles);

        let texture = imgui.prepare_texture(|handle| {

            let mut tex = Texture::new(window);

            tex.upload(
                (handle.width as i32, handle.height as i32),
                Format::RgbaU8,
                handle.pixels);

            tex
        });

        imgui.set_texture_id(texture.get_handle() as usize);

        ImGuiImpl {
            imgui: imgui,
            shader: shader,
            mesh: mesh,
            texture: texture,
        }
    }

    #[inline]
    pub fn render<F: FnMut(&imgui::Ui)>(&mut self, mut gui_func: F, window: &Window, dt: f32) {

        let size = window.get_size();
        let size_u32 = (size.0 as u32, size.1 as u32);

        let safe_dt = f32::max(1.0/1000.0, dt);

        let ui = self.imgui.frame(size_u32, size_u32, safe_dt);

        gui_func(&ui);

        let mesh = &self.mesh;

        ui.render(|ui, draw_list| render_draw_list(mesh, ui, draw_list)).unwrap();
    }
}

#[inline]
fn render_draw_list(mesh: &Mesh, ui: &imgui::Ui, draw_list: imgui::DrawList) -> Result<(), Box<Error>> {
    


    /*self.device_objects.upload_vertex_buffer(&self.ctx, draw_list.vtx_buffer);
    self.device_objects.upload_index_buffer(&self.ctx, draw_list.idx_buffer);

    let (width, height) = ui.imgui().display_size();
    let (scale_width, scale_height) = ui.imgui().display_framebuffer_scale();

    if width == 0.0 || height == 0.0 {
        return Ok(());
    }

    let matrix = [[2.0 / width as f32, 0.0, 0.0, 0.0],
                  [0.0, 2.0 / -(height as f32), 0.0, 0.0],
                  [0.0, 0.0, -1.0, 0.0],
                  [-1.0, 1.0, 0.0, 1.0]];
    let font_texture_id = self.device_objects.texture.get_id() as usize;

    let mut idx_start = 0;
    for cmd in draw_list.cmd_buffer {
        // We don't support custom textures...yet!
        assert!(cmd.texture_id as usize == font_texture_id);

        let idx_end = idx_start + cmd.elem_count as usize;

        try!(surface.draw(&self.device_objects.vertex_buffer,
                          &self.device_objects
                              .index_buffer
                              .slice(idx_start..idx_end)
                              .expect("Invalid index buffer range"),
                          &self.device_objects.program,
                          &uniform! {
                      matrix: matrix,
                      tex: self.device_objects.texture.sampled()
                          .magnify_filter(MagnifySamplerFilter::Nearest),
                  },
                          &DrawParameters {
                              blend: Blend::alpha_blending(),
                              scissor: Some(Rect {
                                  left: (cmd.clip_rect.x * scale_width) as u32,
                                  bottom: ((height - cmd.clip_rect.w) * scale_height) as u32,
                                  width: ((cmd.clip_rect.z - cmd.clip_rect.x) * scale_width) as
                                         u32,
                                  height: ((cmd.clip_rect.w - cmd.clip_rect.y) *
                                           scale_height) as
                                          u32,
                              }),
                              ..DrawParameters::default()
                          }));

        idx_start = idx_end;
    }*/

    Ok(())
}