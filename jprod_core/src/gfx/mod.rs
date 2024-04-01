mod framebuffer;
mod gl;
pub mod mesh;
pub mod pso;
pub mod shader;
pub mod ssbo;
pub mod target;
pub mod texture;

use c_types;
use core::ptr;
use win32;

// TODO: Make into struct
pub unsafe trait Context {}

#[inline]
pub fn init(_: &dyn Context) {
    gl::init();

    if cfg!(debug_assertions) {
        unsafe { gl::DebugMessageCallback(debug_callback, ptr::null_mut()) };
    }

    unsafe {
        gl::Enable(gl::CULL_FACE);
    }
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }
}

#[inline]
pub fn clear(_: &dyn Context, color: &[f32; 4]) {
    unsafe {
        gl::ClearBufferfv(gl::COLOR, 0, color as *const _);
        gl::ClearBufferfv(gl::DEPTH, 0, &[1.0f32] as *const _);
    }
}

#[inline]
pub fn viewport(_: &dyn Context, size: (i32, i32)) {
    unsafe { gl::ViewportIndexedf(0, 0.0, 0.0, size.0 as f32, size.1 as f32) };
}

#[inline]
pub fn is_error(_: &dyn Context) -> bool {
    unsafe { gl::GetError() != 0 }
}

extern "system" fn debug_callback(
    _source: gl::GLenum,
    _type: gl::GLenum,
    _id: gl::GLuint,
    _severity: gl::GLenum,
    _length: gl::GLsizei,
    message: *const gl::GLchar,
    _param: *mut c_types::c_void,
) {
    win32::output_debug_string_raw(message);
    win32::output_debug_string(b"\n\0");
}
