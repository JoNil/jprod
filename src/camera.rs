use math::Mat4;
use math::Vec4;
use math;
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
    pub fn new(window: &Window) -> Camera {
        Camera {
            projection: Mat4::identity(),

            pos: Vec4::xyz(0.0, 0.0, 1.0),

            x_angle: 0.0,
            y_angle: 0.0,

            right: Vec4::zero(),
            up: Vec4::zero(),
            forward: Vec4::zero(),

            previus_mouse_offset: get_mouse_offset(window),
        }
    }

    pub fn update(&mut self, window: &Window, dt: f32) {

        tm_zone!("Camera::update");

        {
            let size = window.get_size();

            self.projection = Mat4::perspective(90.0, size.0 as f32 / size.1 as f32, 0.01, 1000.0);
        }

        let actions = window.get_actions();

        if actions.forward.active {
            self.pos = self.pos.sub(self.forward.mul(dt));
        }
        if actions.backward.active {
            self.pos = self.pos.add(self.forward.mul(dt));
        }
        if actions.right.active {
            self.pos = self.pos.sub(self.right.mul(dt));
        }
        if actions.left.active {
            self.pos = self.pos.add(self.right.mul(dt));
        }
        if actions.up.active {
            self.pos = self.pos.sub(self.up.mul(dt));
        }
        if actions.down.active {
            self.pos = self.pos.add(self.up.mul(dt));
        }

        if actions.reset_camera.active || actions.reset_camera.half_transition_count > 1  {
            self.pos = Vec4::xyz(0.0, 0.0, 1.0);
            self.x_angle = 0.0;
            self.y_angle = 0.0;
        }

        {
            let mouse_offset = get_mouse_offset(window);

            if actions.left_mouse.active {
                self.x_angle += self.previus_mouse_offset.0 - mouse_offset.0;
                self.y_angle += self.previus_mouse_offset.1 - mouse_offset.1;
            }

            self.y_angle = math::clamp(self.y_angle, -math::FRAC_PI_2 + 0.01, math::FRAC_PI_2 - 0.01);
            //self.x_angle = self.x_angle % 2.0*math::consts::PI;

            let y_axis = Vec4::xyz(0.0, 1.0, 0.0);
            let neg_z_axis = Vec4::xyz(0.0, 0.0, -1.0);

            let rotated_dir_x = Mat4::rotate(self.x_angle, y_axis).transform(neg_z_axis.with_w_1());

            self.right = rotated_dir_x.cross(y_axis).normalized();
            self.forward = Mat4::rotate(self.y_angle, self.right).transform(rotated_dir_x.with_w_1()).normalized();
            self.up = self.right.cross(self.forward).normalized();

            self.previus_mouse_offset = mouse_offset;
        }
    }

    pub fn get_view_projection(&self) -> Mat4 {

        tm_zone!("Camera::get_view_projection");

        let pos = Mat4::translate(self.pos);
        let rot = Mat4::axis(self.right, self.up, self.forward.neg());

        self.projection.mul(pos.mul(rot).inverted())
    }

    pub fn get_camera_pos(&self) -> Vec4 {
        self.pos
    }
}

fn get_mouse_offset(window: &Window) -> (f32, f32) {
    
    let mouse = window.get_mouse_pos();
    let size = window.get_size();

    let mouse_x = mouse.0 as f32;
    let mouse_y = mouse.1 as f32;
    let width = size.0 as f32;
    let height = size.1 as f32;

    (
        2.0 * mouse_x / width - 1.0,
        2.0 * mouse_y / height + 1.0,
    )
}