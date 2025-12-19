use super::{
    Context,
    framebuffer::{Attachment, Framebuffer},
    gl,
    texture::{Format, Texture},
};
use crate::{math::Vec4, utils};

pub struct Target {
    framebuffer: Framebuffer,
    texture1: Option<Texture>,
    texture2: Option<Texture>,
    texture3: Option<Texture>,
    depth: Option<Texture>,
    size: (i32, i32),
}

impl Target {
    #[inline]
    pub fn new(
        ctx: &dyn Context,
        size: (i32, i32),
        formats: &[Option<Format>; 3],
        use_depth: bool,
    ) -> Target {
        let mut framebuffer = Framebuffer::new(ctx);
        let mut texture1 = None;
        let mut texture2 = None;
        let mut texture3 = None;
        let mut depth = None;

        if let &Some(format) = unsafe { formats.get_unchecked(0) } {
            let mut texture = Texture::new(ctx);
            texture.allocate(size, format);
            framebuffer.attach(&texture, Attachment::Color0);
            texture1 = Some(texture);
        }

        if let &Some(format) = unsafe { formats.get_unchecked(1) } {
            let mut texture = Texture::new(ctx);
            texture.allocate(size, format);
            framebuffer.attach(&texture, Attachment::Color1);
            texture2 = Some(texture);
        }

        if let &Some(format) = unsafe { formats.get_unchecked(2) } {
            let mut texture = Texture::new(ctx);
            texture.allocate(size, format);
            framebuffer.attach(&texture, Attachment::Color2);
            texture3 = Some(texture);
        }

        if use_depth {
            let mut depth_texture = Texture::new(ctx);
            depth_texture.allocate(size, Format::DepthF32);
            framebuffer.attach_depth(&depth_texture);
            depth = Some(depth_texture);
        }

        utils::assert(framebuffer.is_compleate());

        Target {
            framebuffer,
            texture1,
            texture2,
            texture3,
            depth,
            size,
        }
    }

    #[inline]
    pub fn clear(&mut self, color: Vec4) {
        let color_slice = &[color.x(), color.y(), color.z(), color.w()];

        if self.texture1.is_some() {
            self.framebuffer.clear(Attachment::Color0, color_slice);
        }

        if self.texture2.is_some() {
            self.framebuffer.clear(Attachment::Color1, color_slice);
        }

        if self.texture3.is_some() {
            self.framebuffer.clear(Attachment::Color2, color_slice);
        }

        if self.depth.is_some() {
            self.framebuffer.clear_depth(&[1.0]);
        }
    }

    #[inline]
    pub fn get_texture(&self, index: i32) -> Option<&Texture> {
        match index {
            0 => self.texture1.as_ref(),
            1 => self.texture2.as_ref(),
            2 => self.texture3.as_ref(),
            _ => None,
        }
    }

    #[inline]
    pub fn get_depth_texture(&self) -> Option<&Texture> {
        self.depth.as_ref()
    }

    #[inline]
    pub(super) fn get_framebuffer(&self) -> &Framebuffer {
        &self.framebuffer
    }

    #[inline]
    pub(super) fn get_size(&self) -> (i32, i32) {
        self.size
    }

    #[inline]
    pub(super) fn get_draw_buffer_spec(&self) -> (i32, [u32; 3]) {
        let mut storage = [
            gl::COLOR_ATTACHMENT0,
            gl::COLOR_ATTACHMENT0,
            gl::COLOR_ATTACHMENT0,
        ];

        let mut next_out_index = 0;

        if self.texture1.is_some() {
            unsafe {
                *storage.get_unchecked_mut(next_out_index) = Attachment::Color0 as gl::GLenum;
            }
            next_out_index += 1;
        }

        if self.texture2.is_some() {
            unsafe {
                *storage.get_unchecked_mut(next_out_index) = Attachment::Color1 as gl::GLenum;
            }
            next_out_index += 1;
        }

        if self.texture3.is_some() {
            unsafe {
                *storage.get_unchecked_mut(next_out_index) = Attachment::Color2 as gl::GLenum;
            }
            next_out_index += 1;
        }

        (next_out_index as i32, storage)
    }
}
