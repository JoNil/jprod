#![allow(dead_code)]

use c_types::c_void;
use core::marker::PhantomData;
use core::mem;
use core::ptr;
use gl;
use shader::Shader;
use ssbo::Ssbo;
use utils;
use window::GlContext;

struct RawVao {
    handle: u32,
    marker: PhantomData<*const u32>,
}

impl RawVao {
    fn new() -> RawVao {
        let mut handle = 0;
        unsafe { gl::GenVertexArrays(1, &mut handle as *mut _) };

        utils::debug_trap_if(handle == 0);

        RawVao { handle: handle, marker: PhantomData }
    }
}

impl Drop for RawVao {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &mut self.handle as *mut _); }
    }
}

struct RawVbo {
    handle: u32,
    marker: PhantomData<*const u32>,
}

impl RawVbo {
    fn new() -> RawVbo {
        let mut handle = 0;
        unsafe { gl::GenBuffers(1, &mut handle as *mut _); }
        
        utils::debug_trap_if(handle == 0);

        RawVbo { handle: handle, marker: PhantomData }
    }
}

impl Drop for RawVbo {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &mut self.handle as *mut _); }
    }
}

pub struct Mesh {
    vao: RawVao,
    vbo: RawVbo,
    length: i32,
}

impl Mesh {
    pub fn new(_: &GlContext) -> Mesh {
        
        let vao = RawVao::new();
        let vbo = RawVbo::new();

        unsafe {

            gl::BindVertexArray(vao.handle);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo.handle);

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,              // attribute
                3,              // size
                gl::FLOAT,      // type
                0,              // normalized?
                0,              // stride
                ptr::null());   // array buffer offset

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Mesh { vao: vao, vbo: vbo, length: 0 }
    }

    pub fn upload(&mut self, data: &[[f32; 3]]) {

        self.length = data.len() as i32;

        unsafe {

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo.handle);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (3 * data.len() * mem::size_of::<f32>()) as isize,
                &*(*data.get_unchecked(0)).get_unchecked(0) as *const f32 as *const c_void,
                gl::STATIC_DRAW);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0); 
        }
    }

    pub fn draw(&self, shader: &Shader) {

        if self.length == 0 {
            return;
        }

        unsafe {
            gl::UseProgram(shader.get_program());
            gl::BindVertexArray(self.vao.handle);

            gl::DrawArrays(gl::TRIANGLES, 0, self.length);

            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    }

    pub fn draw_instanced(&self, shader: &Shader, instance_data: &Ssbo, uniform_data: &Ssbo, count: i32) {

        if self.length == 0 || count <= 0 {
            return;
        }

        unsafe {
            gl::UseProgram(shader.get_program());
            gl::BindVertexArray(self.vao.handle);

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, instance_data.get_handle());
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, instance_data.get_handle());
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, uniform_data.get_handle());
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 1, uniform_data.get_handle());
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);

            gl::DrawArraysInstanced(gl::TRIANGLES, 0, self.length, count);
            
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    }
}
