use super::{gl, Context};
use crate::{c_types::c_void, utils};
use core::marker::PhantomData;
use core::mem;

pub struct Ssbo {
    handle: u32,
    marker: PhantomData<*const u32>,
}

impl Ssbo {
    #[inline]
    pub fn new(_: &dyn Context) -> Ssbo {
        let mut handle = 0;
        unsafe {
            gl::GenBuffers(1, &mut handle as *mut _);
        }

        utils::assert(handle != 0);

        Ssbo {
            handle,
            marker: PhantomData,
        }
    }

    #[inline]
    fn upload_inner(&mut self, data: *const c_void, size: isize) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.handle);

            gl::BufferData(gl::SHADER_STORAGE_BUFFER, size, data, gl::STATIC_DRAW);

            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
    }

    #[inline]
    pub fn upload<T: Copy>(&mut self, data: &T) {
        self.upload_inner(
            data as *const T as *const c_void,
            mem::size_of::<T>() as isize,
        );
    }

    #[inline]
    pub fn upload_slice<T: Copy>(&mut self, data: &[T]) {
        unsafe {
            self.upload_inner(
                &*data.get_unchecked(0) as *const T as *const c_void,
                (data.len() * mem::size_of::<T>()) as isize,
            )
        };
    }

    #[inline]
    pub(super) fn get_handle(&self) -> u32 {
        self.handle
    }
}

impl Drop for Ssbo {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.handle as *mut _);
        }
    }
}
