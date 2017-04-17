use core::marker::PhantomData;
use super::Context;
use super::gl;
use super::texture::Format;
use super::texture::Texture;
use utils;

#[derive(Copy, Clone)]
pub enum Attachment {
    Color0 = gl::COLOR_ATTACHMENT0 as isize,
    Color1 = gl::COLOR_ATTACHMENT1 as isize,
    Color2 = gl::COLOR_ATTACHMENT2 as isize,
}

impl Attachment {
    #[inline]
    pub fn get_index(self) -> i32 {
        match self {
            Attachment::Color0 => 0,
            Attachment::Color1 => 1,
            Attachment::Color2 => 2,
        }
    }
}

struct RawFramebuffer {
    handle: u32,
    marker: PhantomData<*const u32>,
}

impl RawFramebuffer {
    #[inline]
    fn new() -> RawFramebuffer {

        let mut handle = 0;

        unsafe { gl::GenFramebuffers(1, &mut handle as *mut _) };

        utils::assert(handle != 0);

        RawFramebuffer { handle: handle, marker: PhantomData }
    }
}

impl Drop for RawFramebuffer {
    #[inline]
    fn drop(&mut self) {
        unsafe { gl::DeleteFramebuffers(1, &mut self.handle as *mut _) };
    }
}

pub struct Framebuffer {
    framebuffer: RawFramebuffer,
}

impl Framebuffer {
    #[inline]
    pub fn new(_: &Context) -> Framebuffer {

        let framebuffer = RawFramebuffer::new();

        Framebuffer {
            framebuffer: framebuffer,
        }
    }

    #[inline]
    pub fn attach(&mut self, texture: &Texture, attachment: Attachment) {

        utils::assert(texture.get_format() != Some(Format::DepthF32));
        
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer.handle);

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                attachment as u32,
                gl::TEXTURE_2D,
                texture.get_handle(),
                0);

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    #[inline]
    pub fn attach_depth(&mut self, texture: &Texture) {
        
        utils::assert(texture.get_format() == Some(Format::DepthF32));

        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer.handle);

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::DEPTH_ATTACHMENT,
                gl::TEXTURE_2D,
                texture.get_handle(),
                0);

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    #[inline]
    pub fn clear(&mut self, attachment: Attachment, value: &[f32; 4]) {

        tm_zone!("Framebuffer:clear");

        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer.handle);

            gl::ClearBufferfv(gl::COLOR, attachment.get_index(), value as *const _);

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    #[inline]
    pub fn clear_depth(&mut self, value: &[f32; 1]) {

        tm_zone!("Framebuffer:clear_depth");

        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer.handle);

            gl::ClearBufferfv(gl::DEPTH, 0, value as *const _);

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    #[inline]
    pub fn is_compleate(&self) -> bool {
        unsafe {

            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer.handle);

            let res = gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE;

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            res
        }
    }

    #[inline]
    pub(super) fn get_handle(&self) -> u32 {
        self.framebuffer.handle
    }
}
