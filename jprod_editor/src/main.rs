#[macro_use]
extern crate imgui;

extern crate jprod_core;

use jprod_core::camera::Camera;
use jprod_core::gfx;
use jprod_core::time;
use jprod_core::utils;
use jprod_core::win32;
use jprod_core::window::Window;

mod imgui_impl;

fn main_window_gui<'a>(ui: &imgui::Ui<'a>) {
    ui.window(im_str!("Hello world"))
        .size((300.0, 100.0), imgui::ImGuiSetCond_FirstUseEver)
        .build(|| {
            ui.text(im_str!("Hello world!"));
            ui.text(im_str!("This...is...imgui-rs!"));
            ui.separator();
            let mouse_pos = ui.imgui().mouse_pos();
            ui.text(im_str!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos.0,
                mouse_pos.1
            ));
        })
}

fn main() {
    win32::init();

    let mut window = Window::new();
    let mut gui = imgui_impl::ImGuiImpl::new(&window);
    let mut camera = Camera::new(&window);

    let start = time::now_s();
    let mut last = start;

    loop {
        window.update();

        let dt = {
            let now = time::now_s();
            let dt = last - now;
            last = now;

            dt
        };

        camera.update(&window, dt as f32);

        window.update_viewport();
        window.clear(&[0.5, 0.1, 0.2, 1.0]);
        gui.render(main_window_gui, &window, dt as f32);
        window.swap();

        utils::assert(!gfx::is_error(&window));
    }
}
