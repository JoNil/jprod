use crate::c_types::c_void;

pub type Atom = u16;
pub type BrushHandle = *mut c_void;
pub type CursorHandle = *mut c_void;
pub type DcHandle = *mut c_void;
pub type GdiObjectHandle = *mut c_void;
pub type GlrcHandle = *mut c_void;
pub type Handle = *mut c_void;
pub type IconHandle = *mut c_void;
pub type InstanceHandle = *mut c_void;
pub type MenuHandle = *mut c_void;
pub type ModuleHandle = *mut c_void;
pub type Proc = *mut c_void;
pub type WindowHandle = *mut c_void;
pub type WindowProc =
    extern "system" fn(window: WindowHandle, message: u32, wparam: usize, lparam: usize) -> usize;

pub const CS_HREDRAW: u32 = 0x0002;
pub const CS_OWNDC: u32 = 0x0020;
pub const CS_VREDRAW: u32 = 0x0001;
pub const CW_USEDEFAULT: i32 = 0x80000000_u32 as i32;
pub const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
pub const FILE_SHARE_DELETE: u32 = 0x00000004;
pub const FILE_SHARE_READ: u32 = 0x00000001;
pub const FILE_SHARE_WRITE: u32 = 0x00000002;
pub const GENERIC_READ: u32 = 0x80000000;
pub const GENERIC_WRITE: u32 = 0x40000000;
pub const GET_FILE_EX_INFO_STANDARD: i32 = 0;
pub const GWLP_USERDATA: i32 = -21;
pub const IDC_ARROW: usize = 32512;
pub const INVALID_HANDLE_VALUE: Handle = -1isize as usize as Handle;
pub const MEM_COMMIT: u32 = 0x00001000;
pub const MEM_RELEASE: u32 = 0x00008000;
pub const MEM_RESERVE: u32 = 0x00002000;
pub const OPEN_EXISTING: u32 = 3;
pub const PAGE_READWRITE: u32 = 0x04;
pub const PFD_DOUBLEBUFFER: u32 = 0x00000001;
pub const PFD_DRAW_TO_WINDOW: u32 = 0x00000004;
pub const PFD_MAIN_PLANE: u8 = 0;
pub const PFD_SUPPORT_OPENGL: u32 = 0x00000020;
pub const PFD_TYPE_RGBA: u8 = 0;
pub const VK_LBUTTON: i32 = 0x01;
pub const WGL_ACCELERATION_ARB: i32 = 0x2003;
pub const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: i32 = 0x00000001;
pub const WGL_CONTEXT_DEBUG_BIT_ARB: i32 = 0x0001;
pub const WGL_CONTEXT_FLAGS_ARB: i32 = 0x2094;
pub const WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: i32 = 0x0002;
pub const WGL_CONTEXT_MAJOR_VERSION_ARB: i32 = 0x2091;
pub const WGL_CONTEXT_MINOR_VERSION_ARB: i32 = 0x2092;
pub const WGL_CONTEXT_PROFILE_MASK_ARB: i32 = 0x9126;
pub const WGL_DOUBLE_BUFFER_ARB: i32 = 0x2011;
pub const WGL_DRAW_TO_WINDOW_ARB: i32 = 0x2001;
pub const WGL_FULL_ACCELERATION_ARB: i32 = 0x2027;
pub const WGL_PIXEL_TYPE_ARB: i32 = 0x2013;
pub const WGL_SUPPORT_OPENGL_ARB: i32 = 0x2010;
pub const WGL_TYPE_RGBA_ARB: i32 = 0x202B;
pub const WM_ACTIVATEAPP: u32 = 0x001C;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_KEYDOWN: u32 = 0x0100;
pub const WM_KEYUP: u32 = 0x0101;
pub const WM_LBUTTONDOWN: u32 = 0x0201;
pub const WM_LBUTTONUP: u32 = 0x0202;
pub const WM_SIZE: u32 = 0x0005;
pub const WM_SYSKEYDOWN: u32 = 0x0104;
pub const WM_SYSKEYUP: u32 = 0x0105;
pub const WS_CAPTION: u32 = 0x00C00000;
pub const WS_MAXIMIZEBOX: u32 = 0x00010000;
pub const WS_MINIMIZEBOX: u32 = 0x00020000;
pub const WS_OVERLAPPED: u32 = 0x00000000;
pub const WS_OVERLAPPEDWINDOW: u32 =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
pub const WS_SYSMENU: u32 = 0x00080000;
pub const WS_THICKFRAME: u32 = 0x00040000;
pub const WS_VISIBLE: u32 = 0x10000000;

#[repr(C)]
#[derive(Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Default)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
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

#[repr(C)]
#[derive(Default)]
pub struct Filetime {
    pub low_datetime: u32,
    pub high_datetime: u32,
}

#[repr(C)]
#[derive(Default)]
pub struct FileAttributeData {
    pub file_attributes: u32,
    pub creation_time: Filetime,
    pub last_access_time: Filetime,
    pub last_write_time: Filetime,
    pub file_size_high: u32,
    pub file_size_low: u32,
}
