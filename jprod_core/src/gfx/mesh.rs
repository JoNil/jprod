use c_types::c_void;
use core::marker::PhantomData;
use core::mem;
use core::ptr;
use gfx;
use super::Context;
use super::gl;
use super::pso::Pso;
use super::querys::QueryManager;
use super::shader::Shader;
use super::ssbo::Ssbo;
use super::target::Target;
use super::texture::Texture;
use utils;

#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq)]
enum VertexFormat {
    Float,
    Vec2,
    Vec3,
    Vec4,
}

impl VertexFormat {
    #[inline]
    fn get_components(&self) -> i32 {
        match *self {
            VertexFormat::Float => 1,
            VertexFormat::Vec2 => 2,
            VertexFormat::Vec3 => 3,
            VertexFormat::Vec4 => 4,
        }
    }
}

struct RawVao {
    handle: u32,
    marker: PhantomData<*const u32>,
}

impl RawVao {
    #[inline]
    fn new() -> RawVao {
        let mut handle = 0;
        unsafe { gl::GenVertexArrays(1, &mut handle as *mut _) };

        utils::assert(handle != 0);

        RawVao { handle: handle, marker: PhantomData }
    }
}

impl Drop for RawVao {
    #[inline]
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &mut self.handle as *mut _); }
    }
}

struct RawVbo {
    handle: u32,
    marker: PhantomData<*const u32>,
}

impl RawVbo {
    #[inline]
    fn new() -> RawVbo {
        let mut handle = 0;
        unsafe { gl::GenBuffers(1, &mut handle as *mut _); }
        
        utils::assert(handle != 0);

        RawVbo { handle: handle, marker: PhantomData }
    }
}

impl Drop for RawVbo {
    #[inline]
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &mut self.handle as *mut _); }
    }
}

#[derive(Copy, Clone)]
pub enum Primitive {
    Triangles = gl::TRIANGLES as isize,
    TriangleStrip = gl::TRIANGLE_STRIP as isize,
}

pub struct Mesh {
    vao: RawVao,
    vbos: [Option<RawVbo>; 3],
    vertex_formats: [Option<VertexFormat>; 3],
    index: Option<RawVbo>,
    length: i32,
    primitive: Primitive,
}

impl Mesh {
    #[inline]
    pub fn new(_: &Context) -> Mesh {
        Mesh {
            vao: RawVao::new(),
            vbos: [None, None, None],
            vertex_formats: [None, None, None],
            index: None,
            length: 0,
            primitive: Primitive::Triangles,
        }
    }

    fn is_same_format(&self, vertex_formats: [Option<VertexFormat>; 3], use_index_buffer: bool) -> bool {
        for (f1, f2) in vertex_formats.iter().zip(self.vertex_formats.iter()) {
            if f1 != f2 {
                return false;
            }

            if use_index_buffer && self.index.is_none() {
                return false;
            }
        }

        return true;
    }

    #[inline]
    fn setup(&mut self, vertex_formats: [Option<VertexFormat>; 3], use_index_buffer: bool) {
        let mut vbos = [None, None, None];
        let mut index = None;

        unsafe {

            gl::BindVertexArray(self.vao.handle);

            for (i, (format, vbo)) in vertex_formats.iter().zip(vbos.iter_mut()).enumerate() {

                if let &Some(ref vf) = format {

                    let new_vbo = RawVbo::new();

                    gl::BindBuffer(gl::ARRAY_BUFFER, new_vbo.handle);
                    gl::EnableVertexAttribArray(i as u32);
                    gl::VertexAttribPointer(
                        i as u32,            // attribute
                        vf.get_components(), // size
                        gl::FLOAT,           // type
                        0,                   // normalized?
                        0,                   // stride
                        ptr::null());        // array buffer offset

                    *vbo = Some(new_vbo);
                }
            }

            if use_index_buffer {
                index = Some(RawVbo::new())
            }

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        self.vbos = vbos;
        self.vertex_formats = vertex_formats;
        self.index = index;
    }

    #[inline]
    pub fn upload(&mut self, va1: &[[f32; 3]], va2: &[[f32; 3]], primitive: Primitive) {

        tm_zone!("Mesh::upload");

        utils::assert(va1.len() == va2.len());

        let vertex_format = [Some(VertexFormat::Vec3), Some(VertexFormat::Vec3), None];

        if !self.is_same_format(vertex_format, false) {
            self.setup(vertex_format, true);
        }

        self.primitive = primitive;
        self.length = va1.len() as i32;

        unsafe {
            if let (&Some(ref pos_vbo), &Some(ref normal_vbo)) = (&*self.vbos.get_unchecked(0), &*self.vbos.get_unchecked(1)) {

                gl::BindBuffer(gl::ARRAY_BUFFER, pos_vbo.handle);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (3 * va1.len() * mem::size_of::<f32>()) as isize,
                    &*(*va1.get_unchecked(0)).get_unchecked(0) as *const f32 as *const c_void,
                    gl::STATIC_DRAW);

                gl::BindBuffer(gl::ARRAY_BUFFER, normal_vbo.handle);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (3 * va2.len() * mem::size_of::<f32>()) as isize,
                    &*(*va2.get_unchecked(0)).get_unchecked(0) as *const f32 as *const c_void,
                    gl::STATIC_DRAW);

                gl::BindBuffer(gl::ARRAY_BUFFER, 0); 
            }
        }
    }

    #[inline]
    pub fn draw(
        &self,
        pso: &Pso,
        shader: &Shader,
        query_manager: &QueryManager,
        target: Option<&Target>,
        textures: &[Option<&Texture>],
        uniform_data: Option<&Ssbo>) {

        tm_zone!("Mesh::draw");

        utils::assert(self.length != 0);

        draw_internal(
            self,
            pso,
            shader,
            query_manager,
            target,
            textures,
            uniform_data,
            &self.vao,
            || {
                unsafe { gl::DrawArrays(self.primitive as u32, 0, self.length); };
            });
    }

    #[inline]
    pub fn draw_instanced(
        &self,
        pso: &Pso,
        shader: &Shader,
        query_manager: &QueryManager,
        target: Option<&Target>,
        textures: &[Option<&Texture>],
        uniform_data: Option<&Ssbo>,
        instance_data: Option<&Ssbo>,
        count: i32) 
    {
        tm_zone!("Mesh::draw_instanced");

        utils::assert(self.length != 0 && count > 0);

        draw_internal(
            self,
            pso,
            shader,
            query_manager,
            target,
            textures,
            uniform_data,
            &self.vao,
            || {
                unsafe {
                    if let Some(instance) = instance_data {
                        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, instance.get_handle());
                        gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 1, instance.get_handle());
                        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
                    }
                    
                    gl::DrawArraysInstanced(self.primitive as u32, 0, self.length, count);
                };
            });
    
    }
}

#[inline]
fn draw_internal<F: FnMut()>(
    context: &Context,
    pso: &Pso,
    shader: &Shader,
    query_manager: &QueryManager,
    target: Option<&Target>,
    textures: &[Option<&Texture>],
    uniform_data: Option<&Ssbo>,
    vao: &RawVao,
    mut f: F)
{
    let _query = query_manager.query();

    unsafe {

        if let Some(render_target) = target {

            gfx::viewport(context, render_target.get_size());

            gl::BindFramebuffer(gl::FRAMEBUFFER, render_target.get_framebuffer().get_handle());

            let (count, buffer) = render_target.get_draw_buffer_spec();
            gl::DrawBuffers(count, &buffer as *const _);
        }

        if let Some(ref scissor) = pso.scissor {
            gl::Enable(gl::SCISSOR_TEST);
            gl::Scissor(scissor.x, scissor.y, scissor.width, scissor.height);
        }

        gl::UseProgram(shader.get_program_handle());
        gl::BindVertexArray(vao.handle);

        for (i, t) in textures.iter().enumerate() {
            if let &Some(tex) = t {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                gl::BindTexture(gl::TEXTURE_2D, tex.get_handle());
                gl::Uniform1i(2 + i as i32, i as i32);
            }
        }

        if let Some(uniform) = uniform_data {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, uniform.get_handle());
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, uniform.get_handle());
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }

        f();

        for (i, t) in textures.iter().enumerate() {
            if let &Some(_) = t {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }
        }

        gl::BindVertexArray(0);
        gl::UseProgram(0);

        if let Some(_) = pso.scissor {
            gl::Disable(gl::SCISSOR_TEST);
        }

         if let Some(_) = target {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            let bufs: [u32; 1] = [ gl::BACK_LEFT ];
            gl::DrawBuffers(bufs.len() as i32, &bufs as *const _);
        }
    }
}

unsafe impl Context for Mesh {}
