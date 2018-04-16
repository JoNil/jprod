use core::marker::PhantomData;
use core::ptr;
use super::Context;
use super::gl;
use utils;
use win32;

struct RawProgram {
    handle: u32,
    marker: PhantomData<*const u32>,
}

impl RawProgram {
    #[inline]
    fn new() -> RawProgram {
        let handle = unsafe { gl::CreateProgram() };
        
        utils::assert(handle != 0);

        RawProgram {
            handle: handle,
            marker: PhantomData,
        }
    }
}

impl Drop for RawProgram {
    #[inline]
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.handle) };
    }
}

struct RawShader {
    handle: u32,
    marker: PhantomData<*const u32>,
}

impl RawShader {
    #[inline]
    fn new(shader_type: u32) -> RawShader {
        let handle = unsafe { gl::CreateShader(shader_type) };
        
        utils::assert(handle != 0);

        RawShader {
            handle: handle,
            marker: PhantomData,
        }
    }
}

impl Drop for RawShader {
    #[inline]
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.handle) };
    }
}

fn load_shader(vertex_source: &[u8], fragment_source: &[u8]) -> Option<(RawProgram, RawShader, RawShader)> {

    let program = RawProgram::new();
    let vertex = RawShader::new(gl::VERTEX_SHADER);
    let fragment = RawShader::new(gl::FRAGMENT_SHADER);

    unsafe {
        let vert_pointer: *const u8 = &*vertex_source.get_unchecked(0);
        let vert_size: i32 = vertex_source.len() as i32;
        gl::ShaderSource(vertex.handle, 1, &vert_pointer, &vert_size);
        gl::CompileShader(vertex.handle);

        let mut vert_status = 0;
        gl::GetShaderiv(vertex.handle, gl::COMPILE_STATUS, &mut vert_status);

        if vert_status == 0 {
            print_shader_error(vertex.handle);
            return None;
        }
    }

    unsafe {
        let frag_pointer: *const u8 = &*fragment_source.get_unchecked(0);
        let frag_size: i32 = fragment_source.len() as i32;
        gl::ShaderSource(fragment.handle, 1, &frag_pointer, &frag_size);
        gl::CompileShader(fragment.handle);

        let mut frag_status = 0;
        gl::GetShaderiv(fragment.handle, gl::COMPILE_STATUS, &mut frag_status);

        if frag_status == 0 {
            print_shader_error(fragment.handle);
            return None;
        }
    }

    unsafe {
        gl::AttachShader(program.handle, fragment.handle);
        gl::AttachShader(program.handle, vertex.handle);

        gl::LinkProgram(program.handle);

        let mut program_status = 0;
        gl::GetProgramiv(program.handle, gl::LINK_STATUS, &mut program_status);

        if program_status == 0 {
            print_program_error(program.handle);
            return None;
        }
    }

    unsafe {
        gl::ValidateProgram(program.handle);

        let mut program_valid = 0;
        gl::GetProgramiv(program.handle, gl::VALIDATE_STATUS, &mut program_valid);

        if program_valid == 0 {
            return None;
        }
    }

    Some((program, vertex, fragment))
}

pub struct Shader {
    program: RawProgram,

    _vertex: RawShader,
    _fragment: RawShader,
}

impl Shader {
    #[inline]
    pub fn from_source(_: &Context, vertex_source: &str, fragment_source: &str) -> Shader {

        if let Some((program, vertex, fragment)) = load_shader(vertex_source.as_bytes(), fragment_source.as_bytes()) {
            return Shader {
                program: program,
                _vertex: vertex,
                _fragment: fragment,
            }
        } else {
            utils::debug_trap();
        }
    }

    #[inline]
    pub(super) fn get_program_handle(&self) -> u32 {
        self.program.handle
    }
}

#[inline]
fn print_shader_error(shader: u32) {

    const SIZE: i32 = 2048;

    let mut info: [u8; SIZE as usize] = [0; SIZE as usize];

    unsafe { gl::GetShaderInfoLog(shader, SIZE, ptr::null_mut(), &mut *info.get_unchecked_mut(0)) };

    win32::message_box(&info, b"\0");
}

#[inline]
fn print_program_error(program: u32) {

    const SIZE: i32 = 2048;

    let mut info: [u8; SIZE as usize] = [0; SIZE as usize];

    unsafe { gl::GetProgramInfoLog(program, SIZE, ptr::null_mut(), &mut *info.get_unchecked_mut(0)) };

    win32::message_box(&info, b"\0");
}
