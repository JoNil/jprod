use gfx::framebuffer::Attachment;
use gfx::framebuffer::Framebuffer;
use gfx::texture::Format;
use gfx::texture::Texture;
use utils;
use window::Window;

pub struct GBuffer {
    framebuffer: Framebuffer,
    color_texture: Texture,
    pos_texture: Texture,
    normal_texture: Texture,
    _depth_texture: Texture,
}

impl GBuffer {
    pub fn new(window: &Window) -> GBuffer {

        let mut framebuffer = Framebuffer::new(window);
        
        let mut color_texture = Texture::new(window);
        let mut pos_texture = Texture::new(window);
        let mut normal_texture = Texture::new(window);
        let mut depth_texture = Texture::new(window);

        let size = window.get_size();

        color_texture.allocate(size.0, size.1, Format::RgbF32);
        pos_texture.allocate(size.0, size.1, Format::RgbF32);
        normal_texture.allocate(size.0, size.1, Format::RgbF32);
        depth_texture.allocate(size.0, size.1, Format::DepthF32);

        framebuffer.attach(&color_texture, Attachment::Color0);
        framebuffer.attach(&pos_texture, Attachment::Color1);
        framebuffer.attach(&normal_texture, Attachment::Color2);
        framebuffer.attach(&depth_texture, Attachment::Depth);

        utils::assert(framebuffer.is_compleate());

        GBuffer {
            framebuffer: framebuffer,
            color_texture: color_texture,
            pos_texture: pos_texture,
            normal_texture: normal_texture,
            _depth_texture: depth_texture,
        }
    }

    pub fn clear(&mut self) {
        self.framebuffer.clear(Attachment::Color0, &[ 0.0, 0.5, 0.0, 0.0 ]);
        self.framebuffer.clear(Attachment::Color1, &[ 0.0, 0.0, 0.0, 0.0 ]);
        self.framebuffer.clear(Attachment::Color2, &[ 0.0, 0.0, 0.0, 0.0 ]);
        self.framebuffer.clear_depth(&[ 1.0 ]);
    }

    pub fn get_framebuffer(&self) -> &Framebuffer {
        &self.framebuffer
    }

    pub fn get_color_texture(&self) -> &Texture {
        &self.color_texture
    }

    pub fn get_pos_texture(&self) -> &Texture {
        &self.pos_texture
    }

    pub fn get_normal_texture(&self) -> &Texture {
        &self.normal_texture
    }
}