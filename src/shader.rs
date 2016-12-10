use core::marker::PhantomData;
use core::ptr;
use gl;
use shader_sources::get_shader_source;
use shader_sources::ShaderId;
use shader_sources::ShaderSource;
use win32;
use win32_types::*;
use window::GlContext;

struct RawProgram {
    handle: u32,
    marker: PhantomData<*const u32>,
}

impl RawProgram {
    fn new() -> RawProgram {
        let handle = unsafe { gl::CreateProgram() };
        if handle == 0 {
            win32::debug_break();
        }

        RawProgram {
            handle: handle,
            marker: PhantomData,
        }
    }
}

impl Drop for RawProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.handle) };
    }
}

struct RawShader {
    handle: u32,
    marker: PhantomData<*const u32>,
}

impl RawShader {
    fn new(shader_type: u32) -> RawShader {
        let handle = unsafe { gl::CreateShader(shader_type) };
        if handle == 0 {
            win32::debug_break();
        }

        RawShader {
            handle: handle,
            marker: PhantomData,
        }
    }
}

impl Drop for RawShader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.handle) };
    }
}

pub struct Shader {
    source: ShaderSource,

    program: RawProgram,

    #[allow(dead_code)]
    fragment: RawShader,
    #[allow(dead_code)]
    vertex: RawShader,
}

impl Shader {
    pub fn new(_: &GlContext, id: ShaderId) -> Shader {

        let source = get_shader_source(id);

        let program = RawProgram::new();
        let fragment = RawShader::new(gl::FRAGMENT_SHADER);
        let vertex = RawShader::new(gl::VERTEX_SHADER);

        {
            let frag_pointer: *const u8 = &source.fragment_source[0];
            let frag_size: i32 = source.fragment_source.len() as i32;
            unsafe { gl::ShaderSource(fragment.handle, 1, &frag_pointer, &frag_size) };
            unsafe { gl::CompileShader(fragment.handle) };

            let mut frag_status = 0;
            unsafe { gl::GetShaderiv(fragment.handle, gl::COMPILE_STATUS, &mut frag_status) };

            if frag_status == 0 {
                print_shader_error(fragment.handle);
                win32::debug_break();
            }
        }

        {
            let vert_pointer: *const u8 = &source.vertex_source[0];
            let vert_size: i32 = source.vertex_source.len() as i32;
            unsafe { gl::ShaderSource(vertex.handle, 1, &vert_pointer, &vert_size) };
            unsafe { gl::CompileShader(vertex.handle) };

            let mut vert_status = 0;
            unsafe { gl::GetShaderiv(vertex.handle, gl::COMPILE_STATUS, &mut vert_status) };

            if vert_status == 0 {
                print_shader_error(vertex.handle);
                win32::debug_break();
            }
        }

        {
            unsafe { gl::AttachShader(program.handle, fragment.handle) };
            unsafe { gl::AttachShader(program.handle, vertex.handle) };

            unsafe { gl::LinkProgram(program.handle) };

            let mut program_status = 0;
            unsafe { gl::GetProgramiv(program.handle, gl::LINK_STATUS, &mut program_status) };

            if program_status == 0 {
                print_program_error(program.handle);
                win32::debug_break();
            }
        }

        {
            unsafe { gl::ValidateProgram(program.handle) };

            let mut program_valid = 0;
            unsafe { gl::GetProgramiv(program.handle, gl::VALIDATE_STATUS, &mut program_valid) };

            if program_valid == 0 {
                win32::debug_break();
            }
        }

        Shader {
            source: source,
            program: program,
            fragment: fragment,
            vertex: vertex,
        }
    }

    // TODO(jonil): Should not be public! Make module for raw gl abstractions
    pub fn get_program(&self) -> u32 {

        let mut vertex_file_attributes: FileAttributrData = FileAttributrData::new();
        win32::get_file_attributes(self.source.vertex_path, GET_FILE_EX_INFO_STANDARD, &mut vertex_file_attributes);

        let mut fragment_file_attributes: FileAttributrData = FileAttributrData::new();
        win32::get_file_attributes(self.source.fragment_path, GET_FILE_EX_INFO_STANDARD, &mut fragment_file_attributes);



        self.program.handle
    }
}

fn print_shader_error(shader: u32) {

    const SIZE: i32 = 2048;

    let mut info: [u8; SIZE as usize] = [0; SIZE as usize];

    unsafe { gl::GetShaderInfoLog(shader, SIZE, ptr::null_mut(), &mut info[0]) };

    win32::output_debug_string(&info);
}

fn print_program_error(program: u32) {

    const SIZE: i32 = 2048;

    let mut info: [u8; SIZE as usize] = [0; SIZE as usize];

    unsafe { gl::GetProgramInfoLog(program, SIZE, ptr::null_mut(), &mut info[0]) };

    win32::output_debug_string(&info);
}
