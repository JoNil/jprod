use c_types::c_void;
use core::marker::PhantomData;
use core::mem;
use super::Context;
use super::gl;
use utils;

pub struct Ssbo {
    handle: u32,
    marker: PhantomData<*const u32>,
}

impl Ssbo {
    pub fn new(_: &Context) -> Ssbo {
        
        let mut handle = 0;
        unsafe { gl::GenBuffers(1, &mut handle as *mut _); }

        utils::assert(handle == 0);

        Ssbo { handle: handle, marker: PhantomData }
    }

    fn upload_inner(&mut self, data: *const c_void, size: isize) {
        unsafe {

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.handle);

            gl::BufferData(
                gl::SHADER_STORAGE_BUFFER,
                size,
                data,
                gl::STATIC_DRAW);

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
    }

    pub fn upload<T: Copy>(&mut self, data: &T) {

        self.upload_inner(data as *const T as *const c_void, mem::size_of::<T>() as isize);
    }

    pub fn upload_slice<T: Copy>(&mut self, data: &[T]) {

        unsafe { self.upload_inner(&*data.get_unchecked(0) as *const T as *const c_void, (data.len() * mem::size_of::<T>()) as isize) };
    }

    pub(super) fn get_handle(&self) -> u32 {
        self.handle
    }
}

impl Drop for Ssbo {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &mut self.handle as *mut _); }
    }
}