#![allow(dead_code)]

use f32;
use mat4::Mat4;
use vec4::Vec4;
use win32;
use window::Window;

pub struct Camera {
    projection: Mat4,

    pos: Vec4,

    x_angle: f32,
    y_angle: f32,

    right: Vec4,
    up: Vec4,
    forward: Vec4,

    previus_mouse_offset: (f32, f32),
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            projection: Mat4::identity(),

            pos: Vec4::xyz(0.0, 0.0, 1.0),

            x_angle: 0.0,
            y_angle: 0.0,

            right: Vec4::zero(),
            up: Vec4::zero(),
            forward: Vec4::zero(),

            previus_mouse_offset: (0.0, 0.0),
        }
    }

    pub fn update(&mut self, window: &Window, dt: f32) {
    
        let actions = window.get_actions();

        let mouse = window.get_mouse_pos();
        let size = window.get_size();

        self.projection = Mat4::perspective(90.0, size.0 as f32 / size.1 as f32, 0.01, 1000.0);

        {
            let mouse_offset = {

                let mouse_x = mouse.0 as f32;
                let mouse_y = mouse.1 as f32;
                let width = size.0 as f32;
                let height = size.1 as f32;

                (
                    2.0 * mouse_x / width - 1.0,
                    2.0 * mouse_y / height + 1.0,
                )
            };


            if actions.left_mouse.active {
                self.x_angle += self.previus_mouse_offset.0 - mouse_offset.0;
                self.y_angle += self.previus_mouse_offset.1 - mouse_offset.1;
            }

            self.y_angle = f32::clamp(self.y_angle, -f32::consts::FRAC_PI_2 + 0.01, f32::consts::FRAC_PI_2 - 0.01);
            //self.x_angle = self.x_angle % 2.0*f32::consts::PI;

            let y_axis = Vec4::xyz(0.0, 1.0, 0.0);
            let neg_z_axis = Vec4::xyz(0.0, 0.0, -1.0);

            let rotated_dir_x = Mat4::rotate(self.x_angle, y_axis) * neg_z_axis.with_w_1();

            self.right = rotated_dir_x.cross(y_axis).normalized();
            self.forward = (Mat4::rotate(self.y_angle, self.right) * rotated_dir_x.with_w_1()).normalized();
            self.up = self.right.cross(self.forward).normalized();

            self.previus_mouse_offset = mouse_offset;
        }

        if actions.forward.active {
            self.pos -= self.forward * dt;
        }
        if actions.backward.active {
            self.pos += self.forward * dt;
        }
        if actions.right.active {
            self.pos -= self.right * dt;
        }
        if actions.left.active {
            self.pos += self.right * dt;
        }
        if actions.up.active {
            self.pos -= self.up * dt;
        }
        if actions.down.active {
            self.pos += self.up * dt;
        }

        let t = format!("x {} y {} z {} xa {} ya {}\n\0", self.pos.x, self.pos.y, self.pos.z, self.x_angle, self.y_angle);

        win32::output_debug_string(t.as_bytes());

    }

    pub fn get_view_projection(&self) -> Mat4 {

        let pos = Mat4::translate(self.pos);
        let rot = Mat4::axis(self.right, self.up, -self.forward);

        self.projection * (pos * rot).inverted().unwrap_or(Mat4::identity())
    }
}