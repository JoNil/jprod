pub enum Void {}

pub type Atom = u16;
pub type BrushHandle = *mut Void;
pub type CursorHandle = *mut Void;
pub type DcHandle = *mut Void;
pub type GdiObjectHandle = *mut Void;
pub type Handle = *mut Void;
pub type IconHandle = *mut Void;
pub type InstanceHandle = *mut Void;
pub type MenuHandle = *mut Void;
pub type ModuleHandle = *mut Void;
pub type Proc = *mut Void;
pub type WindowHandle = *mut Void;
pub type WindowProc = extern "system" fn(window: WindowHandle, message: u32, wparam: usize, lparam: usize) -> usize;

pub const CS_HREDRAW: u32 = 0x0002;
pub const CS_OWNDC: u32 = 0x0020;
pub const CS_VREDRAW: u32 = 0x0001;
pub const CW_USEDEFAULT: i32 = (0x80000000 as u32) as i32;

pub const IDC_ARROW: usize = 32512;

pub const WM_ACTIVATEAPP: u32 = 0x001C;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_SIZE: u32 = 0x0005;

pub const WS_CAPTION: u32 = 0x00C00000;
pub const WS_MAXIMIZEBOX: u32 = 0x00010000;
pub const WS_MINIMIZEBOX: u32 = 0x00020000;
pub const WS_OVERLAPPED: u32 = 0x00000000;
pub const WS_SYSMENU: u32 = 0x00080000;
pub const WS_THICKFRAME: u32 = 0x00040000;
pub const WS_VISIBLE: u32 = 0x10000000;
pub const WS_OVERLAPPEDWINDOW: u32 = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
pub struct Msg {
    pub window_handle: WindowHandle,
    pub message: u32,
    pub wparam: usize,
    pub lparam: usize,
    pub time: u32,
    pub point: Point,
}

#[repr(C)]
pub struct WindowClass {
    pub style: u32,
    pub window_proc: WindowProc,
    pub cls_extra: i32,
    pub wnd_extra: i32,
    pub instance: InstanceHandle,
    pub icon: IconHandle,
    pub cursor: CursorHandle,
    pub background: BrushHandle,
    pub menu_name: *const u8,
    pub class_name: *const u8,
}

pub const PFD_TYPE_RGBA: u8 = 0;
pub const PFD_MAIN_PLANE: u8 = 0;

pub const PFD_DOUBLEBUFFER: u32 = 0x00000001;
pub const PFD_DRAW_TO_WINDOW: u32 = 0x00000004;
pub const PFD_SUPPORT_OPENGL: u32 = 0x00000020;

#[repr(C)]
pub struct PixelFormatDescriptor {
      pub size: u16,
      pub version: u16,
      pub flags: u32,
      pub pixel_type: u8,
      pub color_bits: u8,
      pub red_bits: u8,
      pub red_shift: u8,
      pub green_bits: u8,
      pub green_shift: u8,
      pub blue_bits: u8,
      pub blue_shift: u8,
      pub alpha_bits: u8,
      pub alpha_shift: u8,
      pub accum_bits: u8,
      pub accum_red_bits: u8,
      pub accum_green_bits: u8,
      pub accum_blue_bits: u8,
      pub accum_alpha_bits: u8,
      pub depth_bits: u8,
      pub stencil_bits: u8,
      pub aux_buffers: u8,
      pub layer_type: u8,
      pub reserved: u8,
      pub layer_mask: u32,
      pub visible_mask: u32,
      pub damage_mask: u32,
}