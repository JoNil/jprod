use core::default::Default;
use core::mem;
use core::ptr;
use gfx;
use utils;
use win32::types::*;
use win32;

static WINDOW_NAME: &'static [u8] = b"JProd\n\0";
static WINDOW_CLASS: &'static [u8] = b"C\0";

static WGL_ATTRIBS: &'static [i32] = &[
    WGL_CONTEXT_MAJOR_VERSION_ARB, 4,
    WGL_CONTEXT_MINOR_VERSION_ARB, 5,
    WGL_CONTEXT_FLAGS_ARB, WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB | WGL_CONTEXT_DEBUG_BIT_ARB,
    WGL_CONTEXT_PROFILE_MASK_ARB, WGL_CONTEXT_CORE_PROFILE_BIT_ARB,
    0,
];

static WINDOW_ATTRIBS: &'static [i32] = &[
    WGL_DRAW_TO_WINDOW_ARB, 1,
    WGL_ACCELERATION_ARB, WGL_FULL_ACCELERATION_ARB,
    WGL_SUPPORT_OPENGL_ARB, 1,
    WGL_DOUBLE_BUFFER_ARB, 1,
    WGL_PIXEL_TYPE_ARB, WGL_TYPE_RGBA_ARB,
    0,
];

struct RawWindow {
    handle: WindowHandle,
}

impl RawWindow {
    fn new(visible: bool) -> RawWindow {
        let window = win32::create_window(WINDOW_CLASS, WINDOW_NAME, visible);

        utils::debug_trap_if(window.is_null());

        RawWindow { handle: window }
    }

    fn get_dc(self) -> RawDc {

        let dc = win32::get_dc(self.handle);

        utils::debug_trap_if(dc.is_null());

        RawDc {
            window: self,
            handle: dc,
        }
    }
}

impl Drop for RawWindow {
    fn drop(&mut self) {
        utils::debug_trap_if(win32::destroy_window(self.handle) == 0);
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
            win32::wgl_create_context(self.handle)
        } else {
            win32::wgl_create_context_attribs(self.handle, ptr::null_mut(), WGL_ATTRIBS)
        };

        utils::debug_trap_if(context.is_null());

        RawContext {
            dc: self,
            handle: context,
        }
    }
}

impl Drop for RawDc {
    fn drop(&mut self) {
        utils::debug_trap_if(win32::release_dc(self.window.handle, self.handle) == 0);
    }
}

struct RawContext {
    dc: RawDc,
    handle: GlrcHandle,
}

impl RawContext {
    pub fn make_current(&self) {
        utils::debug_trap_if(win32::wgl_make_current(self.dc.handle, self.handle) == 0);
    }
}

impl Drop for RawContext {
    fn drop(&mut self) {
        utils::debug_trap_if(win32::wgl_make_current(ptr::null_mut(), ptr::null_mut()) == 0);
        utils::debug_trap_if(win32::wgl_delete_context(self.handle) == 0);
    }
}

#[derive(Default)]
pub struct ButtonState {
    pub active: bool,
    pub half_transition_count: i32,
}

impl ButtonState {
    fn process_message(&mut self, is_down: bool) {
        if self.active != is_down {
            self.active = is_down;
            self.half_transition_count += 1;
        }
    }
}

#[derive(Default)]
pub struct Actions {
    pub forward: ButtonState,
    pub backward: ButtonState,
    pub left: ButtonState,
    pub right: ButtonState,
    pub up: ButtonState,
    pub down: ButtonState,

    pub left_mouse: ButtonState,

    pub reset_camera: ButtonState,
}

impl Actions {
    fn reset(&mut self) {
        self.forward.half_transition_count = 0;
        self.backward.half_transition_count = 0;
        self.left.half_transition_count = 0;
        self.right.half_transition_count = 0;
        self.up.half_transition_count = 0;
        self.down.half_transition_count = 0;

        self.left_mouse.half_transition_count = 0;

        self.reset_camera.half_transition_count = 0;
    }
}

pub struct Window {
    context: RawContext,
    actions: Actions,
}

impl Window {
    pub fn new() -> Window {

        utils::debug_trap_if(!win32::register_class(WINDOW_CLASS, window_proc));

        {
            let initial_winow = RawWindow::new(false);
            let dc = initial_winow.get_dc();
            let context = dc.create_context(true);

            context.make_current();

            win32::wgl_load_extensions();
        }

        let window = RawWindow::new(true);
        let dc = window.get_dc();
        let context = dc.create_context(false);

        context.make_current();

        let res = Window { context: context, actions: Default::default() };

        gfx::init(&res);

        res
    }

    pub fn update(&mut self) {

        self.actions.reset();

        {
            let left_down = win32::get_key_down(VK_LBUTTON);
            self.actions.left_mouse.process_message(left_down);
        }

        while let Some(msg) = win32::get_message() {

            match msg.message {
                
                WM_SYSKEYDOWN | WM_SYSKEYUP | WM_KEYDOWN | WM_KEYUP => {
                    
                    let key_code = msg.wparam as u8 as char;

                    let is_down = (msg.lparam & (1 << 31)) == 0;

                    match key_code {
                        'R' => self.actions.forward.process_message(is_down),
                        'F' => self.actions.backward.process_message(is_down),
                        'D' => self.actions.left.process_message(is_down),
                        'G' => self.actions.right.process_message(is_down),
                        'A' => self.actions.up.process_message(is_down),
                        'Z' => self.actions.down.process_message(is_down),
                        'Q' => self.actions.reset_camera.process_message(is_down),
                        _ => (),
                    }
                }

                _ => ()
            }

            win32::translate_and_dispatch_message(&msg);
        }
    }

    pub fn get_actions(&self) -> &Actions {
        &self.actions
    }

    pub fn get_size(&self) -> (i32, i32) {
        let rect = win32::get_window_client_rect(self.context.dc.window.handle);
        
        (rect.2, rect.3)
    }

    pub fn get_mouse_pos(&self) -> (i32, i32) {
        win32::get_mouse_pos(self.context.dc.window.handle)
    }

    pub fn update_viewport(&self) {
        let size = self.get_size();
        gfx::viewport(self, size.0, size.1);
    }

    pub fn clear(&self, color: &[f32; 4]) {
        gfx::clear(self, color);
    }

    pub fn swap(&self) {
        utils::debug_trap_if(!win32::swap_buffers(self.context.dc.handle));
    }
}

unsafe impl gfx::Context for Window {}

extern "system" fn window_proc(handle: WindowHandle, msg: u32, wparam: usize, lparam: usize) -> usize {

    match msg {

        WM_CLOSE => { win32::exit_process(0); }

        _ => {
            return win32::def_window_proc(handle, msg, wparam, lparam);
        }
    }
}

fn set_pixel_format(dc: DcHandle, initial: bool) {

    let mut suggested_pixel_format_index = 0;
    let mut extended_pick = 0;

    if !initial {
        utils::debug_trap_if(win32::wgl_choose_pixel_format(dc,
                Some(WINDOW_ATTRIBS),
                None,
                &mut suggested_pixel_format_index,
                &mut extended_pick) == 0);
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

        suggested_pixel_format_index = win32::choose_pixel_format(dc, &desired_pixel_format);
        
        utils::debug_trap_if(suggested_pixel_format_index == 0);
    }

    let mut suggested_pixel_format = unsafe { mem::uninitialized() };
    utils::debug_trap_if(
            win32::describe_pixel_format(
                    dc,
                    suggested_pixel_format_index,
                    mem::size_of::<PixelFormatDescriptor>() as u32,
                    &mut suggested_pixel_format) == 0);

    utils::debug_trap_if(win32::set_pixel_format(dc, suggested_pixel_format_index, &suggested_pixel_format) == 0);
}
