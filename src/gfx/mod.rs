mod gl;
pub mod framebuffer;
pub mod mesh;
pub mod shader;
pub mod ssbo;
pub mod texture;

pub unsafe trait Context { }

pub fn init(_: &Context) {
    gl::init();

    unsafe { gl::Enable(gl::DEPTH_TEST) };
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