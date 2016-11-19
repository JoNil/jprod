use core::marker::PhantomData;
use core::ptr;
use gl;
use win32;

struct RawProgram {
    program: u32,
    marker: PhantomData<*const u32>,
}

impl RawProgram {
    fn new() -> RawProgram {
        let program = unsafe { gl::CreateProgram() };
        if program == 0 {
            panic!();
        }

        RawProgram {
            program: program,
            marker: PhantomData,
        }
    }
}

impl Drop for RawProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.program) };
    }
}

struct RawShader {
    shader: u32,
    marker: PhantomData<*const u32>,
}

impl RawShader {
    fn new(shader_type: u32) -> RawShader {
        let shader = unsafe { gl::CreateShader(shader_type) };
        if shader == 0 {
            panic!();
        }

        RawShader {
            shader: shader,
            marker: PhantomData,
        }
    }
}

impl Drop for RawShader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.shader) };
    }
}

pub struct Shader {
    program: RawProgram,

    fragment: RawShader,
    vertex: RawShader,
}

impl Shader {
    pub fn new(fragment_source: &[u8], vertex_source: &[u8]) -> Shader {

        let program = RawProgram::new();
        let fragment = RawShader::new(gl::FRAGMENT_SHADER);
        let vertex = RawShader::new(gl::VERTEX_SHADER);

        {
            let frag_pointer: *const u8 = &fragment_source[0];
            let frag_size: i32 = fragment_source.len() as i32;
            unsafe { gl::ShaderSource(fragment.shader, 1, &frag_pointer, &frag_size) };
            unsafe { gl::CompileShader(fragment.shader) };

            let mut frag_status = 0;
            unsafe { gl::GetShaderiv(fragment.shader, gl::COMPILE_STATUS, &mut frag_status) };

            if frag_status == 0 {
                print_shader_error(fragment.shader);
                panic!();
            }
        }

        {
            let vert_pointer: *const u8 = &vertex_source[0];
            let vert_size: i32 = vertex_source.len() as i32;
            unsafe { gl::ShaderSource(vertex.shader, 1, &vert_pointer, &vert_size) };
            unsafe { gl::CompileShader(vertex.shader) };

            let mut vert_status = 0;
            unsafe { gl::GetShaderiv(vertex.shader, gl::COMPILE_STATUS, &mut vert_status) };

            if vert_status == 0 {
                print_shader_error(vertex.shader);
                panic!();
            }
        }

        {
            unsafe { gl::AttachShader(program.program, fragment.shader) };
            unsafe { gl::AttachShader(program.program, vertex.shader) };

            unsafe { gl::LinkProgram(program.program) };

            let mut program_status = 0;
            unsafe { gl::GetProgramiv(program.program, gl::LINK_STATUS, &mut program_status) };

            if program_status == 0 {
                print_program_error(program.program);
                panic!();
            }
        }

        {
            unsafe { gl::ValidateProgram(program.program) };

            let mut program_valid = 0;
            unsafe { gl::GetProgramiv(program.program, gl::VALIDATE_STATUS, &mut program_valid) };

            if program_valid == 0 {
                panic!();
            }
        }

        Shader {
            program: program,
            fragment: fragment,
            vertex: vertex,
        }
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
