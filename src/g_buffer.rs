use framebuffer::Framebuffer;
use texture::Texture;
use texture::TextureFormat;
use window::Window;

pub struct GBuffer {
	framebuffer: Framebuffer,
	texture: Texture,
}

impl GBuffer {
	pub fn new(window: &Window) -> GBuffer {

		let framebuffer = Framebuffer::new(window);
		let mut texture = Texture::new(window);

        let size = window.get_size();

        texture.allocate(size.0, size.1, TextureFormat::Rgb_f32);

		GBuffer {
			framebuffer: framebuffer,
			texture: texture,
		}
	}
}