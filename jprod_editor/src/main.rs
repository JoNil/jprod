#[cfg(feature = "use_telemetry")]
extern crate telemetry;

#[macro_use]
extern crate telemetry_macro;

#[macro_use]
extern crate imgui;

extern crate jprod_core;

use imgui::*;
use jprod_core::camera::Camera;
use jprod_core::gfx::querys::QueryManager;
use jprod_core::gfx;
use jprod_core::time;
use jprod_core::utils;
use jprod_core::win32;
use jprod_core::window::Window;

mod imgui_render;

fn gui<'a>(ui: &Ui<'a>) {
    ui.window(im_str!("Hello world"))
        .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
        .build(|| {
            ui.text(im_str!("Hello world!"));
            ui.text(im_str!("This...is...imgui-rs!"));
            ui.separator();
            let mouse_pos = ui.imgui().mouse_pos();
            ui.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
        })
}

fn main() {

    win32::init();

    let mut window = Window::new();
    let mut camera = Camera::new(&window);
    let mut query_manager = QueryManager::new(&window);

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
        window.clear(&[ 0.5, 0.1, 0.2, 1.0 ]);
        window.swap();

        query_manager.submit_zones();

        utils::assert(!gfx::is_error(&window));

        tm_tick!();
    }
}