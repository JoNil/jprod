use core::intrinsics::transmute;
use core::mem;
use core::ptr;
use win32_types::*;

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

pub struct Module {
    handle: ModuleHandle
}

impl Module {

    pub fn new(file_name: &[u8]) -> Option<Module> {

        let handle = unsafe { LoadLibraryA(&file_name[0]) }; 

        if handle == ptr::null_mut() {
            None
        } else {
            Some(Module {
                handle: handle,
            })
        }
    }

    pub fn get_proc_address(&self, proc_name: &[u8]) -> Proc {

        unsafe { GetProcAddress(self.handle, &proc_name[0]) }   
    }
}

impl Drop for Module {
    fn drop(&mut self) {
         unsafe { FreeLibrary(self.handle) }
    }
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
    wparam: usize,
    lparam: usize,
    time: u32,
    point: Point,
}

#[allow(non_snake_case)]
pub struct Api {
    
    #[allow(dead_code)]
    user32: Module,

    MessageBoxA: unsafe extern "system" fn(window_handle: WindowHandle, text: *const u8, caption: *const u8, message_type: u32) -> i32,

    RegisterClassA: unsafe extern "system" fn(windowClass: *const WindowClass) -> Atom,
    CreateWindowExA: unsafe extern "system" fn(ex_style: u32, class_name: *const u8, window_name: *const u8, style: u32, x: i32, y: i32, width: i32, height: i32, parent_winodw: WindowHandle, menu: MenuHandle, instance: InstanceHandle, param: *mut Void) -> WindowHandle,

    GetMessageA: unsafe extern "system" fn(msg: *mut Msg, window_handle: WindowHandle, msg_filter_min: i32, msg_filter_max: i32) -> i32,
    TranslateMessage: unsafe extern "system" fn(msg: *const Msg) -> i32,
    DispatchMessageA: unsafe extern "system" fn(msg: *const Msg) -> i32,
    DefWindowProcA: unsafe extern "system" fn(window: WindowHandle, message: u32, wparam: usize, lparam: usize) -> usize,

    LoadCursorA: unsafe extern "system" fn(instance: InstanceHandle, name: usize) -> CursorHandle,
}

impl Api {
    pub fn new() -> Api {
        
        if let Some(user32) = Module::new(b"user32.dll\0") {

            Api {
                MessageBoxA: load_proc!(user32,  b"MessageBoxA\0"),

                RegisterClassA: load_proc!(user32, b"RegisterClassA\0"),
                CreateWindowExA: load_proc!(user32, b"CreateWindowExA\0"),

                GetMessageA: load_proc!(user32, b"GetMessageA\0"),
                TranslateMessage: load_proc!(user32, b"TranslateMessage\0"),
                DispatchMessageA: load_proc!(user32, b"DispatchMessageA\0"),
                DefWindowProcA: load_proc!(user32, b"DefWindowProcA\0"),

                LoadCursorA: load_proc!(user32, b"LoadCursorA\0"),

                user32: user32,
            }
        } else {
            panic!();
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
    pub fn def_window_proc(&self, window: WindowHandle, message: u32, wparam: usize, lparam: usize) -> usize {

        unsafe { (self.DefWindowProcA)(window, message, wparam, lparam) }
    }
}