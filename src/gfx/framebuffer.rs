use core::marker::PhantomData;
use super::Context;
use super::gl;
use super::texture::Texture;
use utils;

pub enum Attachment {
    Color0 = gl::COLOR_ATTACHMENT0 as isize,
    Color1 = gl::COLOR_ATTACHMENT1 as isize,
    Color2 = gl::COLOR_ATTACHMENT2 as isize,
    Depth = gl::DEPTH_ATTACHMENT as isize,
}

impl Attachment {
    fn get_index(self) -> i32 {
        match self {
            Attachment::Color0 => 0,
            Attachment::Color1 => 1,
            Attachment::Color2 => 2,
            Attachment::Depth => utils::debug_trap(),
        }
    }
}

struct RawFramebuffer {
    handle: u32,
    marker: PhantomData<*const u32>,
}

impl RawFramebuffer {
    fn new() -> RawFramebuffer {

        let mut handle = 0;

        unsafe { gl::GenFramebuffers(1, &mut handle as *mut _) };

        utils::assert(handle != 0);

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
    render_targets: i32,
}

impl Framebuffer {
    pub fn new(_: &Context, render_targets: i32) -> Framebuffer {

        utils::assert(render_targets >= 0);
        utils::assert(render_targets < 4);

        let framebuffer = RawFramebuffer::new();

        Framebuffer {
            framebuffer: framebuffer,
            render_targets: render_targets,
        }
    }

    pub fn attach(&mut self, texture: &Texture, attachment: Attachment) {
        
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

    pub fn clear(&mut self, attachment: Attachment, value: &[f32; 4]) {

        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer.handle);

            gl::ClearBufferfv(gl::COLOR, attachment.get_index(), value as *const _);

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn clear_depth(&mut self, value: &[f32; 1]) {

        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer.handle);

            gl::ClearBufferfv(gl::DEPTH, 0, value as *const _);

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn is_compleate(&self) -> bool {
        unsafe {

            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer.handle);

            let res = gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE;

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            res
        }
    }

    pub(super) fn get_handle(&self) -> u32 {
        self.framebuffer.handle
    }

    pub(super) fn get_draw_buffer_spec(&self) -> (i32, [u32; 3]) {

        let storage = [ gl::COLOR_ATTACHMENT0, gl::COLOR_ATTACHMENT1, gl::COLOR_ATTACHMENT2 ];

        (self.render_targets, storage)
    }
}
