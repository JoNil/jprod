#![allow(dead_code)]

use mat4::Mat4;
use window::Window;

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

	pub fn update(&mut self, window: &Window, dt: f32) {
	
		let actions = window.get_actions();

		let mouse = window.get_mouse_pos();
		let size = window.get_size();

		let mouse_offset = {

            let mouse_x = mouse.0 as f32;
            let mouse_y = mouse.1 as f32;
            let width = size.0 as f32;
            let height = size.1 as f32;

            (
                2.0 * mouse_x / width - 1.0,
                -2.0 * mouse_y / height + 1.0,
            )
        };

	}

	pub fn get_view_projection(&self) -> Mat4 {
		self.projection_matrix * self.view_matrix.inverted().unwrap()
	}
}