use core::intrinsics::transmute;
use core::ptr;

enum Void {}

type Handle = *mut Void;
type WindowHandle = *mut Void;
type Module = *mut Void;
type Proc = *mut Void;

#[link_args = "kernel32.lib"]
extern "system" {
    fn OutputDebugStringA(output_string: *const u8);
    fn ExitProcess(exit_code: u32) -> !;

    fn LoadLibraryA(file_name: *const u8) -> Module;
    fn FreeLibrary(module: Module);
    fn GetProcAddress(module: Module, proc_name: *const u8) -> Proc;
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
fn load_library_a(file_name: &[u8]) -> Module
{
    unsafe { LoadLibraryA(&file_name[0]) }
}

#[inline]
fn free_library(module: Module)
{
    unsafe { FreeLibrary(module) }
}

#[inline]
fn get_proc_address(module: Module, proc_name: &[u8]) -> Proc
{
    unsafe { GetProcAddress(module, &proc_name[0]) }   
}

#[allow(non_snake_case)]
pub struct User32 {
    module: Module,

    MessageBoxA: unsafe extern "system" fn(window_handle: WindowHandle, text: *const u8, caption: *const u8, message_type: u32) -> i32,
}

impl User32 {
    pub fn message_box(&self, text: &[u8], caption: &[u8], box_type: u32)
    {
        unsafe {
            (self.MessageBoxA)(ptr::null_mut(), &text[0], &caption[0], box_type);
        }
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

        let user32_messagebox = get_proc_address(user32_module, b"MessageBoxA\0");
        if user32_messagebox == ptr::null_mut() {
            panic!();
        }

        Api {
            user32: User32 {
                module: user32_module,
                MessageBoxA: unsafe { transmute(user32_messagebox) },
            }
        }
    }
}