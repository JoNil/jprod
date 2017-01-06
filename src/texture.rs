use core::marker::PhantomData;
use gl;
use utils;
use window::GlContext;

struct RawTexture {
    handle: u32,
    marker: PhantomData<*const u32>,
}

impl RawTexture {
    pub fn new() -> RawTexture {

        let mut handle = 0;

        unsafe { gl::GenTextures(1, &mut handle as *mut _) };

        utils::debug_trap_if(handle == 0);

        RawTexture { handle: handle, marker: PhantomData }
    }
}

impl Drop for RawTexture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &mut self.handle as *mut _) };
    }
}

pub struct Texture {
    texture: RawTexture,
}

impl Texture {
    pub fn new(_: &GlContext) -> Texture {
        let texture = RawTexture::new();

        Texture {
            texture: texture,
        }
    }
}