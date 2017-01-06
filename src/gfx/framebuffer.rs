use core::marker::PhantomData;
use super::Context;
use super::gl;
use super::texture::Texture;
use utils;

pub enum Attachment {
    Color0 = gl::COLOR_ATTACHMENT0 as isize,
}

struct RawFramebuffer {
    handle: u32,
    marker: PhantomData<*const u32>,
}

impl RawFramebuffer {
    pub fn new() -> RawFramebuffer {

        let mut handle = 0;

        unsafe { gl::GenFramebuffers(1, &mut handle as *mut _) };

        utils::debug_trap_if(handle == 0);

        RawFramebuffer { handle: handle, marker: PhantomData }
    }
}

impl Drop for RawFramebuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteFramebuffers(1, &mut self.handle as *mut _) };
    }
}

pub struct Framebuffer {
    framebuffer: RawFramebuffer,
}

impl Framebuffer {
    pub fn new(_: &Context) -> Framebuffer {
        let framebuffer = RawFramebuffer::new();

        Framebuffer {
            framebuffer: framebuffer,
        }
    }

    pub fn attach(&mut self, texture: &Texture, attachment_point: Attachment) {

        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer.handle);

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                attachment_point as u32,
                gl::TEXTURE_2D,
                texture.get_handle(),
                0);

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}