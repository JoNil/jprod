#![allow(dead_code)]

use c_types::*;
use core::ptr;
use module::Module;
use win32_types::*;

#[link_args = "kernel32.lib"]
#[allow(dead_code)]
extern "system" {
    fn OutputDebugStringA(output_string: *const u8);
    fn ExitProcess(exit_code: u32) -> !;
    fn GetModuleHandleA(module_name: *const u8) -> ModuleHandle;

    fn LoadLibraryA(file_name: *const u8) -> ModuleHandle;
    fn FreeLibrary(module: ModuleHandle);
    fn GetProcAddress(module: ModuleHandle, proc_name: *const u8) -> Proc;
}

pub fn output_debug_string(string: &[u8]) {

    unsafe {
        OutputDebugStringA(&string[0]);
    }
}

pub fn output_debug_string_raw(string: *const u8) {

    unsafe {
        OutputDebugStringA(string);
    }
}

pub fn exit_process(exit_code: u32) -> ! {

    unsafe {
        ExitProcess(exit_code);
    }
}

pub fn get_module_handle(module_name: &[u8]) -> ModuleHandle {

    unsafe { GetModuleHandleA(&module_name[0]) }
}

pub fn load_library(file_name: &[u8]) -> ModuleHandle {

    unsafe { LoadLibraryA(&file_name[0]) }
}

pub fn free_library(module: ModuleHandle) {

    unsafe {
        FreeLibrary(module);
    }
}

pub fn get_proc_address(module: ModuleHandle, proc_index: isize) -> Proc {

    unsafe { GetProcAddress(module, proc_index as *const u8) }
}

static mut API: Option<Api> = None;

#[allow(non_snake_case)]
struct Api {
    #[allow(dead_code)]
    user32: Module,

    MessageBoxA: unsafe extern "system" fn(window_handle: WindowHandle,
                                           text: *const u8,
                                           caption: *const u8,
                                           message_type: u32)
                                           -> i32,

    RegisterClassA: unsafe extern "system" fn(windowClass: *const WindowClass) -> Atom,
    CreateWindowExA: unsafe extern "system" fn(ex_style: u32,
                                               class_name: *const u8,
                                               window_name: *const u8,
                                               style: u32,
                                               x: i32,
                                               y: i32,
                                               width: i32,
                                               height: i32,
                                               parent_winodw: WindowHandle,
                                               menu: MenuHandle,
                                               instance: InstanceHandle,
                                               param: *mut c_void)
                                               -> WindowHandle,
    DestroyWindow: unsafe extern "system" fn(window_handle: WindowHandle) -> i32,

    GetDC: unsafe extern "system" fn(window: WindowHandle) -> DcHandle,
    ReleaseDC: unsafe extern "system" fn(window: WindowHandle, dc: DcHandle) -> i32,

    PeekMessageA: unsafe extern "system" fn(msg: *mut Msg,
                                            window_handle: WindowHandle,
                                            msg_filter_min: u32,
                                            msg_filter_max: u32,
                                            remove_message: u32)
                                            -> i32,
    TranslateMessage: unsafe extern "system" fn(msg: *const Msg) -> i32,
    DispatchMessageA: unsafe extern "system" fn(msg: *const Msg) -> i32,
    DefWindowProcA: unsafe extern "system" fn(window: WindowHandle,
                                              message: u32,
                                              wparam: usize,
                                              lparam: usize)
                                              -> usize,

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
                MessageBoxA: load_proc!(user32, 1501 + 617),

                RegisterClassA: load_proc!(user32, 1501 + 700),
                CreateWindowExA: load_proc!(user32, 1501 + 121),
                DestroyWindow: load_proc!(user32, 1501 + 183),

                GetDC: load_proc!(user32, 1501 + 322),
                ReleaseDC: load_proc!(user32, 1501 + 733),

                PeekMessageA: load_proc!(user32, 1501 + 654),
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

pub fn create_window(class_name: &[u8], name: &[u8], visible: bool) -> WindowHandle {
    unsafe {
        (api().CreateWindowExA)(0,
                                &class_name[0],
                                &name[0],
                                WS_OVERLAPPEDWINDOW | if visible { WS_VISIBLE } else { 0 },
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

pub fn destroy_window(window: WindowHandle) -> i32 {
    unsafe { (api().DestroyWindow)(window) }
}

pub fn get_dc(window: WindowHandle) -> DcHandle {
    unsafe { (api().GetDC)(window) }
}

pub fn release_dc(window: WindowHandle, dc: DcHandle) -> i32 {
    unsafe { (api().ReleaseDC)(window, dc) }
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

    let msg_result = unsafe { (api().PeekMessageA)(&mut msg, ptr::null_mut(), 0, 0, 1) };

    if msg_result != 0 { Some(msg) } else { None }
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
