#![allow(dead_code)]

use mat4::Mat4;

pub struct Camera {
	projection_matrix: Mat4,
	view_matrix: Mat4,
}

impl Camera {
	pub fn new() -> Camera {
		Camera {
			projection_matrix: Mat4::identity(),
			view_matrix: Mat4::identity(),
		}
	}

	pub fn update(&mut self, dt: f32) {
		
	}

	pub fn get_view_projection(&self) -> Mat4 {
		self.projection_matrix * self.view_matrix.inverted().unwrap()
	}
}