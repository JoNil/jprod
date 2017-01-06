use framebuffer::Framebuffer;
use texture::Texture;
use window::GlContext;

pub struct GBuffer {
	framebuffer: Framebuffer,
	texture: Texture,
}

impl GBuffer {
	pub fn new(context: &GlContext) -> GBuffer {

		let framebuffer = Framebuffer::new(context);
		let texture = Texture::new(context);

		GBuffer {
			framebuffer: framebuffer,
			texture: texture,
		}
	}
}