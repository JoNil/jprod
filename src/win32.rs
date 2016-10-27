use core::intrinsics::transmute;
use core::mem;
use core::ptr;

macro_rules! load_proc {
    ($module:expr, $name:expr) => {
        {
            let procedure = get_proc_address($module, $name);
            if procedure == ptr::null_mut() {
                panic!();
            }
            unsafe { transmute(procedure) }
        }
    }
}

pub enum Void {}
pub type WindowHandle = *mut Void;
pub type WindowProc = extern "system" fn(window: WindowHandle, message: u32, wparam: u64, lparam: u64) -> u64;
type Atom = u16;
type BrushHandle = *mut Void;
type CursorHandle = *mut Void;
type GdiObjectHandle = *mut Void;
type Handle = *mut Void;
type IconHandle = *mut Void;
type InstanceHandle = *mut Void;
type MenuHandle = *mut Void;
type ModuleHandle = *mut Void;
type Proc = *mut Void;

pub const WM_ACTIVATEAPP: u32 = 0x001C;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_SIZE: u32 = 0x0005;

const CS_HREDRAW: u32 = 0x0002;
const CS_OWNDC: u32 = 0x0020;
const CS_VREDRAW: u32 = 0x0001;
const CW_USEDEFAULT: i32 = (0x80000000 as u32) as i32;
const IDC_ARROW: u64 = 32512;

const GWLP_USERDATA: i32 = -21;

const WS_CAPTION: u32 = 0x00C00000;
const WS_MAXIMIZEBOX: u32 = 0x00010000;
const WS_MINIMIZEBOX: u32 = 0x00020000;
const WS_OVERLAPPED: u32 = 0x00000000;
const WS_SYSMENU: u32 = 0x00080000;
const WS_THICKFRAME: u32 = 0x00040000;
const WS_VISIBLE: u32 = 0x10000000;
const WS_OVERLAPPEDWINDOW: u32 = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

#[link_args = "kernel32.lib"]
extern "system" {
    fn OutputDebugStringA(output_string: *const u8);
    fn ExitProcess(exit_code: u32) -> !;
    fn GetModuleHandleA(module_name: *const u8) -> ModuleHandle;

    fn LoadLibraryA(file_name: *const u8) -> ModuleHandle;
    fn FreeLibrary(module: ModuleHandle);
    fn GetProcAddress(module: ModuleHandle, proc_name: *const u8) -> Proc;
}

#[inline]
pub fn output_debug_string_a(string: &[u8]) {

    unsafe { OutputDebugStringA(&string[0]); }
}

#[inline]
pub fn exit_process(exit_code: u32) -> ! {
    
    unsafe { ExitProcess(exit_code); }
}

#[inline]
fn load_library_a(file_name: &[u8]) -> ModuleHandle {

    unsafe { LoadLibraryA(&file_name[0]) }
}

#[inline]
fn free_library(module: ModuleHandle) {

    unsafe { FreeLibrary(module) }
}

#[inline]
fn get_proc_address(module: ModuleHandle, proc_name: &[u8]) -> Proc {

    unsafe { GetProcAddress(module, &proc_name[0]) }   
}

#[repr(C)]
struct WindowClass {
    style: u32,
    window_proc: WindowProc,
    cls_extra: i32,
    wnd_extra: i32,
    instance: InstanceHandle,
    icon: IconHandle,
    cursor: CursorHandle,
    background: BrushHandle,
    menu_name: *const u8,
    class_name: *const u8,
}

#[repr(C)]
pub struct Point {
    x: i32,
    y: i32,
}

#[repr(C)]
pub struct Msg {
    window_handle: WindowHandle,
    message: u32,
    wparam: u64,
    lparam: u64,
    time: u32,
    point: Point,
}

#[allow(non_snake_case)]
pub struct Api {
    user32: ModuleHandle,

    MessageBoxA: unsafe extern "system" fn(window_handle: WindowHandle, text: *const u8, caption: *const u8, message_type: u32) -> i32,

    RegisterClassA: unsafe extern "system" fn(windowClass: *const WindowClass) -> Atom,
    CreateWindowExA: unsafe extern "system" fn(ex_style: u32, class_name: *const u8, window_name: *const u8, style: u32, x: i32, y: i32, width: i32, height: i32, parent_winodw: WindowHandle, menu: MenuHandle, instance: InstanceHandle, param: *mut Void) -> WindowHandle,

    GetMessageA: unsafe extern "system" fn(msg: *mut Msg, window_handle: WindowHandle, msg_filter_min: i32, msg_filter_max: i32) -> i32,
    TranslateMessage: unsafe extern "system" fn(msg: *const Msg) -> i32,
    DispatchMessageA: unsafe extern "system" fn(msg: *const Msg) -> i32,
    DefWindowProcA: unsafe extern "system" fn(window: WindowHandle, message: u32, wparam: u64, lparam: u64) -> u64,
    
    SetWindowLongPtrA: unsafe extern "system" fn(window: WindowHandle, index: i32, data: u64) -> u64,
    GetWindowLongPtrA: unsafe extern "system" fn(window: WindowHandle, index: i32) -> u64,

    LoadCursorA: unsafe extern "system" fn(instance: InstanceHandle, name: u64) -> CursorHandle,
}

impl Api {
    pub fn new() -> Api {
        
        let user32 = load_library_a(b"user32.dll\0");
        if user32 == ptr::null_mut() {
            panic!();
        }

        Api {
            user32: user32,
            
            MessageBoxA: load_proc!(user32,  b"MessageBoxA\0"),

            RegisterClassA: load_proc!(user32, b"RegisterClassA\0"),
            CreateWindowExA: load_proc!(user32, b"CreateWindowExA\0"),

            GetMessageA: load_proc!(user32, b"GetMessageA\0"),
            TranslateMessage: load_proc!(user32, b"TranslateMessage\0"),
            DispatchMessageA: load_proc!(user32, b"DispatchMessageA\0"),
            DefWindowProcA: load_proc!(user32, b"DefWindowProcA\0"),
            SetWindowLongPtrA: load_proc!(user32, b"SetWindowLongPtrA\0"),
            GetWindowLongPtrA: load_proc!(user32, b"GetWindowLongPtrA\0"),

            LoadCursorA: load_proc!(user32, b"LoadCursorA\0"),
        }
    }

    #[inline]
    pub fn message_box(&self, text: &[u8], caption: &[u8], box_type: u32) {
        unsafe {
            (self.MessageBoxA)(ptr::null_mut(), &text[0], &caption[0], box_type);
        }
    }

    #[inline]
    pub fn register_class(&self, name: &[u8], window_proc: WindowProc) -> bool {
        let window_class = WindowClass {
            style: CS_VREDRAW | CS_HREDRAW | CS_OWNDC,
            window_proc: window_proc,
            cls_extra: 0,
            wnd_extra: 0,
            instance: unsafe { GetModuleHandleA(ptr::null_mut()) },
            icon: ptr::null_mut(),
            cursor: unsafe { (self.LoadCursorA)(ptr::null_mut(), IDC_ARROW) },
            background: ptr::null_mut(),
            menu_name: ptr::null(),
            class_name: &name[0],
        };

        unsafe { (self.RegisterClassA)(&window_class) != 0 }
    }

    #[inline]
    pub fn create_window(&self, class_name: &[u8], name: &[u8]) -> WindowHandle {
        unsafe {
            (self.CreateWindowExA)(
                    0,
                    &class_name[0],
                    &name[0],
                    WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    ptr::null_mut(),
                    ptr::null_mut(),
                    GetModuleHandleA(ptr::null_mut()),
                    ptr::null_mut())
        }
    }

    #[inline]
    pub fn get_message(&self) -> Option<Msg> {
        let mut msg: Msg = unsafe { mem::uninitialized() };
            
        let msg_result = unsafe { (self.GetMessageA)(&mut msg, ptr::null_mut(), 0, 0) };

        if msg_result != 0 {
            Some(msg)
        } else {
            None
        }
    }

    #[inline]
    pub fn translate_and_dispatch_message(&self, msg: &Msg) {
        unsafe { 
            (self.TranslateMessage)(msg as *const Msg);
            (self.DispatchMessageA)(msg as *const Msg);
        }
    }

    #[inline]
    pub fn def_window_proc(&self, window: WindowHandle, message: u32, wparam: u64, lparam: u64) -> u64 {

        unsafe { (self.DefWindowProcA)(window, message, wparam, lparam) }
    }

    #[inline]
    pub fn set_window_user_data(&self, window: WindowHandle, data: u64) {

        unsafe { (self.SetWindowLongPtrA)(window, GWLP_USERDATA, data) };
    }

    #[inline]
    pub fn get_window_user_data(&self, window: WindowHandle) -> u64 {

        unsafe { (self.GetWindowLongPtrA)(window, GWLP_USERDATA) }
    }

}

impl Drop for Api {
    fn drop(&mut self) {
        if self.user32 != ptr::null_mut() {
            free_library(self.user32);
            self.user32 = ptr::null_mut();
        }
    }
}