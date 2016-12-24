use c_types::c_void;
use core::marker::PhantomData;
use core::mem;
use gl;
use win32;
use window::GlContext;

pub struct Ssbo {
    handle: u32,
    marker: PhantomData<*const u32>,
}

impl Ssbo {
    pub fn new(_: &GlContext) -> Ssbo {
        let mut handle = 0;
        unsafe { gl::GenBuffers(1, &mut handle as *mut _); }
        if handle == 0 {
            win32::debug_break();
        }

        Ssbo { handle: handle, marker: PhantomData }
    }

    pub fn upload<T>(&mut self, data: &[T]) {

        unsafe { gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.handle); }

        unsafe { gl::BufferData(gl::SHADER_STORAGE_BUFFER,
            (data.len() * mem::size_of::<T>()) as isize,
            &data[0] as *const T as *const c_void,
            gl::STATIC_DRAW); }

        unsafe { gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0); }
    }

    // TODO(jonil): Should not be public! Make module for raw gl abstractions
    pub fn get_handle(&self) -> u32 {
        self.handle
    }

}

impl Drop for Ssbo {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &mut self.handle as *mut _); }
    }
}