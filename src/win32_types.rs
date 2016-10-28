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