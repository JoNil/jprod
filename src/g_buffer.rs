use gfx::framebuffer::Attachment;
use gfx::framebuffer::Framebuffer;
use gfx::texture::Format;
use gfx::texture::Texture;
use window::Window;

pub struct GBuffer {
    framebuffer: Framebuffer,
    _color_texture: Texture,
    _pos_texture: Texture,
    _depth_texture: Texture,
}

impl GBuffer {
    pub fn new(window: &Window) -> GBuffer {

        let mut framebuffer = Framebuffer::new(window);
        
        let mut color_texture = Texture::new(window);
        let mut pos_texture = Texture::new(window);
        let mut depth_texture = Texture::new(window);

        let size = window.get_size();

        color_texture.allocate(size.0, size.1, Format::RgbF32);
        pos_texture.allocate(size.0, size.1, Format::RgbF32);
        depth_texture.allocate(size.0, size.1, Format::DepthF32);

        framebuffer.attach(&color_texture, Attachment::Color0);
        framebuffer.attach(&pos_texture, Attachment::Color1);
        framebuffer.attach(&depth_texture, Attachment::Depth);

        GBuffer {
            framebuffer: framebuffer,
            _color_texture: color_texture,
            _pos_texture: pos_texture,
            _depth_texture: depth_texture,
        }
    }

    pub fn clear(&mut self) {
        self.framebuffer.clear(Attachment::Color0, &[ 0.0, 0.0, 0.0, 1.0 ]);
        self.framebuffer.clear(Attachment::Color1, &[ 0.0, 0.0, 0.0, 0.0 ]);
        self.framebuffer.clear_depth(&[ 1.0 ]);
    }
}