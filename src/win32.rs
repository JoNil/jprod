use core::intrinsics::transmute;
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
pub type WindowHandle = *mut Void;
pub type WindowProc = extern "system" fn(window: WindowHandle, message: u32, wparam: u64, lparam: u64);

const CS_VREDRAW: u32 = 0x0001;
const CS_HREDRAW: u32 = 0x0002;
const CS_OWNDC:   u32 = 0x0020;
const IDC_ARROW:  u64 = 32512;

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
pub fn output_debug_string_a(string: &[u8])
{
    unsafe { OutputDebugStringA(&string[0]); }
}

#[inline]
pub fn exit_process(exit_code: u32) -> !
{
    unsafe { ExitProcess(exit_code); }
}

#[inline]
fn load_library_a(file_name: &[u8]) -> ModuleHandle
{
    unsafe { LoadLibraryA(&file_name[0]) }
}

#[inline]
fn free_library(module: ModuleHandle)
{
    unsafe { FreeLibrary(module) }
}

#[inline]
fn get_proc_address(module: ModuleHandle, proc_name: &[u8]) -> Proc
{
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

#[allow(non_snake_case)]
pub struct User32 {
    module: ModuleHandle,

    MessageBoxA: unsafe extern "system" fn(window_handle: WindowHandle, text: *const u8, caption: *const u8, message_type: u32) -> i32,

    RegisterClassA: unsafe extern "system" fn(windowClass: *const WindowClass) -> Atom,
    CreateWindowExA: unsafe extern "system" fn(ex_style: u32, class_name: *const u8, window_name: *const u8, style: u32, x: i32, y: i32, width: i32, height: i32, parent_winodw: WindowHandle, menu: MenuHandle, instance: InstanceHandle, param: *mut Void) -> WindowHandle,

    LoadCursorA: unsafe extern "system" fn(instance: InstanceHandle, name: u64) -> CursorHandle,
}

impl User32 {
    pub fn message_box(&self, text: &[u8], caption: &[u8], box_type: u32)
    {
        unsafe {
            (self.MessageBoxA)(ptr::null_mut(), &text[0], &caption[0], box_type);
        }
    }

    pub fn register_class(&self, name: &[u8], window_proc: WindowProc) -> bool
    {
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

    pub fn create_window(&self)
    {
        
    }
}

impl Drop for User32 {
    fn drop(&mut self) {
        if self.module != ptr::null_mut() {
            free_library(self.module);
            self.module = ptr::null_mut();
        }
    }
}

pub struct Api {
    pub user32: User32,
}

impl Api {
    pub fn new() -> Api {
        
        let user32_module = load_library_a(b"user32.dll\0");
        if user32_module == ptr::null_mut() {
            panic!();
        }

        Api {
            user32: User32 {
                module: user32_module,
                MessageBoxA: load_proc!(user32_module,  b"MessageBoxA\0"),
                RegisterClassA: load_proc!(user32_module, b"RegisterClassA\0"),
                CreateWindowExA: load_proc!(user32_module, b"CreateWindowExA\0"),
                LoadCursorA: load_proc!(user32_module, b"LoadCursorA\0"),
            }
        }
    }
}