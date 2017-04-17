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
    pos_vbo: RawVbo,
    normal_vbo: RawVbo,
    length: i32,
    primitive: Primitive,
}

impl Mesh {
    #[inline]
    pub fn new(_: &Context, primitive: Primitive) -> Mesh {
        
        let vao = RawVao::new();
        let pos_vbo = RawVbo::new();
        let normal_vbo = RawVbo::new();

        unsafe {

            gl::BindVertexArray(vao.handle);

            gl::BindBuffer(gl::ARRAY_BUFFER, pos_vbo.handle);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,              // attribute
                3,              // size
                gl::FLOAT,      // type
                0,              // normalized?
                0,              // stride
                ptr::null());   // array buffer offset

            gl::BindBuffer(gl::ARRAY_BUFFER, normal_vbo.handle);
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,              // attribute
                3,              // size
                gl::FLOAT,      // type
                0,              // normalized?
                0,              // stride
                ptr::null());   // array buffer offset

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Mesh {
            vao: vao,
            pos_vbo: pos_vbo,
            normal_vbo: normal_vbo,
            length: 0,
            primitive: primitive,
        }
    }

    #[inline]
    pub fn upload(&mut self, verts: &[[f32; 3]], normals: &[[f32; 3]]) {

        tm_zone!("Mesh::upload");

        utils::assert(verts.len() == normals.len());

        self.length = verts.len() as i32;

        unsafe {

            gl::BindBuffer(gl::ARRAY_BUFFER, self.pos_vbo.handle);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (3 * verts.len() * mem::size_of::<f32>()) as isize,
                &*(*verts.get_unchecked(0)).get_unchecked(0) as *const f32 as *const c_void,
                gl::STATIC_DRAW);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.normal_vbo.handle);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (3 * normals.len() * mem::size_of::<f32>()) as isize,
                &*(*normals.get_unchecked(0)).get_unchecked(0) as *const f32 as *const c_void,
                gl::STATIC_DRAW);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0); 
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