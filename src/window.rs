use core::mem;
use core::ptr;
use gdi32;
use opengl32;
use win32;
use win32_types::*;

static WINDOW_NAME: &'static [u8] = b"JProd\n\0";
static WINDOW_CLASS: &'static [u8] = b"C\0";

static WGL_ATTRIBS: &'static [i32] =
&[
    WGL_CONTEXT_MAJOR_VERSION_ARB, 4,
    WGL_CONTEXT_MINOR_VERSION_ARB, 5,
    WGL_CONTEXT_FLAGS_ARB, WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB | WGL_CONTEXT_DEBUG_BIT_ARB,
    WGL_CONTEXT_PROFILE_MASK_ARB, WGL_CONTEXT_CORE_PROFILE_BIT_ARB,
    0,
];

static WINDOW_ATTRIBS: &'static [i32] =
&[
    WGL_DRAW_TO_WINDOW_ARB, 1,
    WGL_ACCELERATION_ARB, WGL_FULL_ACCELERATION_ARB,
    WGL_SUPPORT_OPENGL_ARB, 1,
    WGL_DOUBLE_BUFFER_ARB, 1,
    WGL_PIXEL_TYPE_ARB, WGL_TYPE_RGBA_ARB,
    0,
];

extern "system" fn window_proc(window: WindowHandle,
                               msg: u32,
                               wparam: usize,
                               lparam: usize)
                               -> usize {

    match msg {

        WM_SIZE => {
            win32::output_debug_string(b"WM_SIZE\n\0");
        }

        WM_CLOSE => {
            win32::output_debug_string(b"WM_CLOSE\n\0");
            win32::exit_process(0);
        }

        WM_ACTIVATEAPP => {
            win32::output_debug_string(b"WM_ACTIVATEAPP\n\0");
        }

        WM_DESTROY => {
            win32::output_debug_string(b"WM_DESTROY\n\0");
        }

        _ => {
            return win32::def_window_proc(window, msg, wparam, lparam);
        }
    }

    return 0;
}

fn set_pixel_format(dc: DcHandle, initial: bool) {

    let mut suggested_pixel_format_index = 0;
    let mut extended_pick = 0;

    if !initial {
        if opengl32::choose_pixel_format(dc, Some(WINDOW_ATTRIBS), None, &mut suggested_pixel_format_index, &mut extended_pick) == 0 {
            panic!();
        }
    }

    if extended_pick == 0 {

        let desired_pixel_format = PixelFormatDescriptor {
            size: mem::size_of::<PixelFormatDescriptor>() as u16,
            version: 1,
            flags: PFD_SUPPORT_OPENGL | PFD_DRAW_TO_WINDOW | PFD_DOUBLEBUFFER,
            pixel_type: PFD_TYPE_RGBA,
            color_bits: 32,
            red_bits: 0,
            red_shift: 0,
            green_bits: 0,
            green_shift: 0,
            blue_bits: 0,
            blue_shift: 0,
            alpha_bits: 8,
            alpha_shift: 0,
            accum_bits: 0,
            accum_red_bits: 0,
            accum_green_bits: 0,
            accum_blue_bits: 0,
            accum_alpha_bits: 0,
            depth_bits: 0,
            stencil_bits: 0,
            aux_buffers: 0,
            layer_type: PFD_MAIN_PLANE,
            reserved: 0,
            layer_mask: 0,
            visible_mask: 0,
            damage_mask: 0,
        };

        suggested_pixel_format_index = gdi32::choose_pixel_format(dc, &desired_pixel_format);
        if suggested_pixel_format_index == 0 {
            panic!();
        }
    }

    let mut suggested_pixel_format = unsafe { mem::uninitialized() };
    if gdi32::describe_pixel_format(dc,
                                    suggested_pixel_format_index,
                                    mem::size_of::<PixelFormatDescriptor>() as u32,
                                    &mut suggested_pixel_format) == 0 {
        panic!();
    }

    if gdi32::set_pixel_format(dc, suggested_pixel_format_index, &suggested_pixel_format) == 0 {
        panic!();
    }
}

struct RawWindow {
    handle: WindowHandle,
}

impl RawWindow {
    fn new(visible: bool) -> RawWindow {
        let window = win32::create_window(WINDOW_CLASS, WINDOW_NAME, visible);

        if window == ptr::null_mut() {
            panic!();
        }

        RawWindow { handle: window }
    }

    fn get_dc(self) -> RawDc {

        let dc = win32::get_dc(self.handle);
        if dc == ptr::null_mut() {
            panic!();
        }

        RawDc { window: self, handle: dc }
    }
}

impl Drop for RawWindow {
    fn drop(&mut self) {
        if win32::destroy_window(self.handle) == 0 {
            panic!();
        }
    }
}

struct RawDc {
    window: RawWindow,
    handle: DcHandle,
}

impl RawDc {
    fn create_context(self, inital: bool) -> RawContext {

        set_pixel_format(self.handle, inital);

        let context = if inital {
                opengl32::create_context(self.handle)
            } else {
                opengl32::create_context_attribs(self.handle, ptr::null_mut(), WGL_ATTRIBS)
            };

        if context == ptr::null_mut() {
            panic!();
        }

        RawContext { dc: self, handle: context }
    }
}

impl Drop for RawDc {
    fn drop(&mut self) {
        if win32::release_dc(self.window.handle, self.handle) == 0 {
            panic!();
        }
    }
}

struct RawContext {
    dc: RawDc,
    handle: GlrcHandle,
}

impl RawContext {
    pub fn make_current(&self) {
        if opengl32::make_current(self.dc.handle, self.handle) == 0 {
            panic!();
        }
    }
}

impl Drop for RawContext {
    fn drop(&mut self) {
        if opengl32::make_current(ptr::null_mut(), ptr::null_mut()) == 0 {
            panic!();
        }

        if opengl32::delete_context(self.handle) == 0 {
            panic!();
        }
    }
}

pub struct Window {
    context: RawContext,
}

impl Window {
    pub fn new() -> Window {

        if !win32::register_class(WINDOW_CLASS, window_proc) {
            panic!();
        }

        {
            let initial_winow = RawWindow::new(false);
            let dc = initial_winow.get_dc();
            let context = dc.create_context(true);

            context.make_current();

            opengl32::load_extensions();
        }

        let window = RawWindow::new(true);
        let dc = window.get_dc();
        let context = dc.create_context(false);

        context.make_current();

        Window { context: context }
    }

    pub fn process_messages(&self) {
        loop {
            if let Some(msg) = win32::get_message() {
                win32::translate_and_dispatch_message(&msg);
            }
        }
    }

}
