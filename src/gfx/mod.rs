mod gl;
pub mod framebuffer;
pub mod mesh;
pub mod shader;
pub mod ssbo;
pub mod texture;

#[cfg(feature = "use_telemetry")]
pub mod querys;

#[cfg(not(feature = "use_telemetry"))]
pub mod querys_nop;

#[cfg(not(feature = "use_telemetry"))]
pub use self::querys_nop as querys;

use c_types;
use core::ptr;
use win32;

pub unsafe trait Context { }

pub fn init(_: &Context) {
    gl::init();

    if cfg!(debug_assertions) {
        unsafe { gl::DebugMessageCallback(debug_callback, ptr::null_mut()) };
    }

    unsafe { gl::Enable(gl::CULL_FACE); }
    unsafe { gl::Enable(gl::DEPTH_TEST); }
}

pub fn clear(_: &Context, color: &[f32; 4]) {
    unsafe {
        gl::ClearBufferfv(gl::COLOR, 0, color as *const _);
        gl::ClearBufferfv(gl::DEPTH, 0, &[ 1.0f32 ] as *const _);
    }
}

pub fn viewport(_: &Context, width: i32, height: i32) {

    unsafe { gl::ViewportIndexedf(0, 0.0, 0.0, width as f32, height as f32) };
}

pub fn is_error(_: &Context) -> bool {

    tm_zone!("gfx::is_error");

    unsafe { gl::GetError() != 0 }
}

extern "system"
fn debug_callback(
    _source: gl::GLenum, _type: gl::GLenum, _id: gl::GLuint, _severity: gl::GLenum,
    _length: gl::GLsizei, message: *const gl::GLchar, _param: *mut c_types::c_void)
{
    win32::output_debug_string_raw(message);
    win32::output_debug_string(b"\n\0");
}