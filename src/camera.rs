#![allow(dead_code)]

use mat4::Mat4;
use vec4::Vec4;
use window::Window;

pub struct Camera {
    projection: Mat4,

    pos: Vec4,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            projection: Mat4::identity(),
            pos: Vec4::xyz(0.0, 0.0, -1.0),
        }
    }

    pub fn update(&mut self, window: &Window, dt: f32) {
    
        let actions = window.get_actions();

        let mouse = window.get_mouse_pos();
        let size = window.get_size();

        self.projection = Mat4::perspective(90.0, size.0 as f32 / size.1 as f32, 0.01, 1000.0);

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

        if actions.forward.active {
            self.pos.z -= dt;
        }
        if actions.backward.active {
            self.pos.z += dt;
        }
        if actions.right.active {
            self.pos.x += dt;
        }
        if actions.left.active {
            self.pos.x -= dt;
        }
        if actions.up.active {
            self.pos.y += dt;
        }
        if actions.down.active {
            self.pos.y -= dt;
        }
    }

    pub fn get_view_projection(&self) -> Mat4 {

        let pos = Mat4::translate(self.pos);

        self.projection * pos
    }
}