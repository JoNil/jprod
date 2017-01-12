use core::default::Default;
use core::marker::PhantomData;
use core::ptr;
use file::File;
use pool::PoolAllocator;
use shader_sources::get_shader_source;
use shader_sources::ShaderId;
use shader_sources::ShaderSource;
use super::Context;
use super::gl;
use utils;
use win32::types::*;
use win32;

struct RawProgram {
    handle: u32,
    marker: PhantomData<*const u32>,
}

impl RawProgram {
    fn new() -> RawProgram {
        let handle = unsafe { gl::CreateProgram() };
        
        utils::assert(handle == 0);

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
        
        utils::assert(handle == 0);

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

fn load_shader(fragment_source: &[u8], vertex_source: &[u8]) -> Option<(RawProgram, RawShader, RawShader)> {

    let program = RawProgram::new();
    let fragment = RawShader::new(gl::FRAGMENT_SHADER);
    let vertex = RawShader::new(gl::VERTEX_SHADER);

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

    Some((program, fragment, vertex))
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
    pub fn new(_: &Context, id: ShaderId) -> Shader {

        let source = get_shader_source(id);

        if let Some((program, fragment, vertex)) = load_shader(source.fragment_source, source.vertex_source) {
            return Shader {
                source: source,
                program: program,
                fragment: fragment,
                vertex: vertex,
            }
        } else {
            utils::debug_trap();
        }
    }

    pub fn reload_if_changed<'a>(&mut self, allocator: &PoolAllocator<'a>) {
        if cfg!(debug_assertions) {
            let mut needs_update = false;

            let mut vertex_file_attributes: FileAttributeData = Default::default();
            win32::get_file_attributes(self.source.vertex_path, GET_FILE_EX_INFO_STANDARD, &mut vertex_file_attributes);

            if win32::compare_file_time(&vertex_file_attributes.last_write_time, &self.source.vertex_filetime) == 1 {
                needs_update = true;
            }

            let mut fragment_file_attributes: FileAttributeData = Default::default();
            win32::get_file_attributes(self.source.fragment_path, GET_FILE_EX_INFO_STANDARD, &mut fragment_file_attributes);

            if win32::compare_file_time(&fragment_file_attributes.last_write_time, &self.source.fragment_filetime) == 1 {
                needs_update = true;
            }

            if needs_update {
                let local_allocator = allocator.get_sub_allocator();

                if let (Some(fragment_file), Some(vertex_file)) =
                    (File::open(self.source.fragment_path), File::open(self.source.vertex_path)) {

                    let vertex_source = vertex_file.read_entire_file(&local_allocator);
                    let fragment_source = fragment_file.read_entire_file(&local_allocator);

                    self.source.vertex_filetime = vertex_file_attributes.last_write_time;
                    self.source.fragment_filetime = fragment_file_attributes.last_write_time;

                    if let Some((program, fragment, vertex)) = load_shader(fragment_source, vertex_source) {
                        self.program = program;
                        self.fragment = fragment;
                        self.vertex = vertex;
                    }
                }
            }
        }
    }

    pub(super) fn get_program_handle(&self) -> u32 {
        self.program.handle
    }
}

fn print_shader_error(shader: u32) {

    const SIZE: i32 = 2048;

    let mut info: [u8; SIZE as usize] = [0; SIZE as usize];

    unsafe { gl::GetShaderInfoLog(shader, SIZE, ptr::null_mut(), &mut *info.get_unchecked_mut(0)) };

    win32::message_box(&info, b"\0");
}

fn print_program_error(program: u32) {

    const SIZE: i32 = 2048;

    let mut info: [u8; SIZE as usize] = [0; SIZE as usize];

    unsafe { gl::GetProgramInfoLog(program, SIZE, ptr::null_mut(), &mut *info.get_unchecked_mut(0)) };

    win32::message_box(&info, b"\0");
}
