#![allow(dead_code)]

use c_types::c_void;
use core::marker::PhantomData;
use core::mem;
use core::ptr;
use gl;
use shader::Shader;
use ssbo::Ssbo;
use win32;
use window::GlContext;

struct RawVao {
    handle: u32,
    marker: PhantomData<*const u32>,
}

impl RawVao {
    fn new() -> RawVao {
        let mut handle = 0;
        unsafe { gl::GenVertexArrays(1, &mut handle as *mut _) };
        if handle == 0 {
            win32::debug_break();
        }

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
        if handle == 0 {
            win32::debug_break();
        }

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

        unsafe { gl::BindVertexArray(vao.handle); }
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, vbo.handle); }

        unsafe { gl::EnableVertexAttribArray(0); }
        unsafe { gl::VertexAttribPointer(
                0,              // attribute
                3,              // size
                gl::FLOAT,      // type
                0,              // normalized?
                0,              // stride
                ptr::null()); } // array buffer offset

        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0); }
        unsafe { gl::BindVertexArray(0); }

        Mesh { vao: vao, vbo: vbo, length: 0 }
    }

    pub fn upload(&mut self, data: &[[f32; 3]]) {

        self.length = data.len() as i32;

        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo.handle); }

        unsafe { gl::BufferData(gl::ARRAY_BUFFER,
                (3 * data.len() * mem::size_of::<f32>()) as isize,
                &*(*data.get_unchecked(0)).get_unchecked(0) as *const f32 as *const c_void,
                gl::STATIC_DRAW); }

        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0); }
    }

    pub fn draw(&self, shader: &Shader) {

        if self.length == 0 {
            return;
        }

        unsafe { gl::UseProgram(shader.get_program()); }
        unsafe { gl::BindVertexArray(self.vao.handle); }

        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, self.length); }

        unsafe { gl::BindVertexArray(0); }
        unsafe { gl::UseProgram(0); }
    }

    pub fn draw_instanced(&self, shader: &Shader, instance_data: &Ssbo, uniform_data: &Ssbo, count: i32) {

        if self.length == 0 || count <= 0 {
            return;
        }

        unsafe { gl::UseProgram(shader.get_program()); }
        unsafe { gl::BindVertexArray(self.vao.handle); }

        unsafe { gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, instance_data.get_handle()); }
        unsafe { gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 0, instance_data.get_handle()); }
        unsafe { gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, uniform_data.get_handle()); }
        unsafe { gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, 1, uniform_data.get_handle()); }
        unsafe { gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0); }

        unsafe { gl::DrawArraysInstanced(gl::TRIANGLES, 0, self.length, count); }
        
        unsafe { gl::BindVertexArray(0); }
        unsafe { gl::UseProgram(0); }
    }
}

