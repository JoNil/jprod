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
    pub handle: ModuleHandle
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

    pub fn get_proc_address(&self, proc_name: *const u8) -> Proc {

        unsafe { GetProcAddress(self.handle, proc_name) }   
    }
}

/*impl Drop for Module {
    fn drop(&mut self) {
         unsafe { FreeLibrary(self.handle) }
    }
}*/

static mut API: Option<Api> = None;

#[allow(non_snake_case)]
struct Api {
    
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

#[inline]
fn api() -> &'static Api {
    unsafe {
        if let Some(ref api) = API {
            api
        } else {
            panic!();
        }
    }
}

pub fn init() {

        if let Some(user32) = Module::new(b"user32.dll\0") {

            unsafe {
                API = Some(Api {
                    MessageBoxA: load_proc!(user32,  1501 + 617),

                    RegisterClassA: load_proc!(user32, 1501 + 700),
                    CreateWindowExA: load_proc!(user32, 1501 + 121),

                    GetMessageA: load_proc!(user32, 1501 + 383),
                    TranslateMessage: load_proc!(user32, 1501 + 897),
                    DispatchMessageA: load_proc!(user32, 1501 + 190),
                    DefWindowProcA: load_proc!(user32, 1501 + 170),

                    LoadCursorA: load_proc!(user32, 1501 + 577),

                    user32: user32,
                })
            }
        } else {
            panic!();
        }

}

pub fn message_box(text: &[u8], caption: &[u8], box_type: u32) {
    unsafe {
        (api().MessageBoxA)(ptr::null_mut(), &text[0], &caption[0], box_type);
    }
}

pub fn register_class(name: &[u8], window_proc: WindowProc) -> bool {
    let window_class = WindowClass {
        style: CS_VREDRAW | CS_HREDRAW | CS_OWNDC,
        window_proc: window_proc,
        cls_extra: 0,
        wnd_extra: 0,
        instance: unsafe { GetModuleHandleA(ptr::null_mut()) },
        icon: ptr::null_mut(),
        cursor: unsafe { (api().LoadCursorA)(ptr::null_mut(), IDC_ARROW) },
        background: ptr::null_mut(),
        menu_name: ptr::null(),
        class_name: &name[0],
    };

    unsafe { (api().RegisterClassA)(&window_class) != 0 }
}

pub fn create_window(class_name: &[u8], name: &[u8]) -> WindowHandle {
    unsafe {
        (api().CreateWindowExA)(
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

pub fn get_message() -> Option<Msg> {
    let mut msg: Msg = Msg {
        window_handle: ptr::null_mut(),
        message: 0,
        wparam: 0,
        lparam: 0,
        time: 0,
        point: Point { x: 0, y: 0 },
    };
        
    let msg_result = unsafe { (api().GetMessageA)(&mut msg, ptr::null_mut(), 0, 0) };

    if msg_result != 0 {
        Some(msg)
    } else {
        None
    }
}

pub fn translate_and_dispatch_message(msg: &Msg) {
    unsafe { 
        (api().TranslateMessage)(msg as *const Msg);
        (api().DispatchMessageA)(msg as *const Msg);
    }
}

pub fn def_window_proc(window: WindowHandle, message: u32, wparam: usize, lparam: usize) -> usize {

    unsafe { (api().DefWindowProcA)(window, message, wparam, lparam) }
}