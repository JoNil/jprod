use gfx::framebuffer::Attachment;
use gfx::framebuffer::Framebuffer;
use gfx::texture::Format;
use gfx::texture::Texture;
use window::Window;

pub struct GBuffer {
	framebuffer: Framebuffer,
	texture: Texture,
}

impl GBuffer {
	pub fn new(window: &Window) -> GBuffer {

		let mut framebuffer = Framebuffer::new(window);
		let mut texture = Texture::new(window);

        let size = window.get_size();

        texture.allocate(size.0, size.1, Format::Rgb_f32);

        framebuffer.attach(&texture, Attachment::Color_0);

		GBuffer {
			framebuffer: framebuffer,
			texture: texture,
		}
	}
}