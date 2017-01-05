#![allow(dead_code)]

use c_types::*;
use core::default::Default;
use core::mem;
use core::ptr;
use utils;
use win32_types::*;

#[link(name = "kernel32")]
#[allow(dead_code)]
extern "system" {
    fn OutputDebugStringA(output_string: *const u8);
    fn ExitProcess(exit_code: u32) -> !;
    fn GetModuleHandleA(module_name: *const u8) -> ModuleHandle;

    fn LoadLibraryA(file_name: *const u8) -> ModuleHandle;
    fn FreeLibrary(module: ModuleHandle);
    fn GetProcAddress(module: ModuleHandle, proc_name: *const u8) -> Proc;

    fn GetFileAttributesExA(file_name: *const u8, info_level_id: i32, file_information: *mut FileAttributeData) -> i32;
    fn CompareFileTime(file_time_1: *const Filetime, file_time_2: *const Filetime) -> isize;

    fn CreateFileA(
            file_name: *const u8,
            desired_access: u32,
            share_mode: u32,
            security_attributes: *const c_void,
            creation_disposition: u32,
            flags_and_attributes: u32,
            template_file: Handle) -> Handle;
    fn CloseHandle(handle: Handle) -> i32;
    fn GetFileSize(handle: Handle, file_size_high: *mut u32) -> u32;
    fn ReadFile(handle: Handle, buffer: *mut c_void, bytes_to_read: u32, bytes_read: *mut u32, overlapped: *mut c_void) -> i32;

    fn VirtualAlloc(base_address: *mut c_void, size: usize, allocation_type: u32, protect: u32) -> *mut c_void;
    fn VirtualFree(address: *mut c_void, size: usize, free_type: u32) -> i32;

    fn QueryPerformanceCounter(time: *mut i64) -> i32;
    fn QueryPerformanceFrequency(frequency: *mut i64) -> i32;
}

pub fn output_debug_string(string: &[u8]) {

    unsafe { OutputDebugStringA(&*string.get_unchecked(0)); }
}

pub fn output_debug_string_raw(string: *const u8) {

    unsafe { OutputDebugStringA(string); }
}

pub fn exit_process(exit_code: u32) -> ! {

    unsafe { ExitProcess(exit_code); }
}

pub fn get_module_handle(module_name: &[u8]) -> ModuleHandle {

    unsafe { GetModuleHandleA(&*module_name.get_unchecked(0)) }
}

pub fn load_library(file_name: &[u8]) -> ModuleHandle {

    let handle = unsafe { LoadLibraryA(&*file_name.get_unchecked(0)) };

    utils::debug_trap_if(handle.is_null());

    handle
}

pub fn free_library(module: ModuleHandle) {

    unsafe { FreeLibrary(module); }
}

pub fn get_proc_address(module: ModuleHandle, proc_index: isize) -> Proc {

    let ptr = unsafe { GetProcAddress(module, proc_index as *const u8) };

    utils::debug_trap_if(ptr.is_null());

    ptr
}

pub fn get_proc_address_name(module: ModuleHandle, name: &[u8]) -> Proc {

    unsafe { GetProcAddress(module, &*name.get_unchecked(0)) }
}

pub fn get_file_attributes(file_name: &[u8], info_level_id: i32, file_information: &mut FileAttributeData) -> i32 {

    unsafe { GetFileAttributesExA(&*file_name.get_unchecked(0), info_level_id, file_information as *mut _) }
}

pub fn open_file(file_name: &[u8]) -> Handle {
    unsafe {
        CreateFileA(
            &*file_name.get_unchecked(0),
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            ptr::null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            ptr::null_mut())
    }
}

pub fn close_file(handle: Handle) {
    unsafe { CloseHandle(handle); };
}

pub fn get_file_size(handle: Handle) -> u64 {

    let mut size_high: u32 = 0;

    let size_low = unsafe { GetFileSize(handle, &mut size_high as *mut u32) };

    ((size_high as u64) << 32) | (size_low as u64)
}

pub fn read_file(handle: Handle, buffer: &mut [u8]) -> i32 {

    unsafe { ReadFile(handle, &mut *buffer.get_unchecked_mut(0) as *mut u8 as *mut c_void, buffer.len() as u32, ptr::null_mut(), ptr::null_mut()) }
}

pub fn compare_file_time(file_time_1: &Filetime, file_time_2: &Filetime) -> isize {

    unsafe { CompareFileTime(file_time_1 as *const _, file_time_2 as *const _) }
}

pub fn virtual_alloc(size: usize) -> *mut c_void {

    unsafe { VirtualAlloc(ptr::null_mut(), size,  MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE) }
}

pub fn virtual_free(address: *mut c_void)  {

    unsafe { VirtualFree(address, 0, MEM_RELEASE) };
}

pub fn query_performance_counter() -> i64 {

    let mut time = 0;

    unsafe { QueryPerformanceCounter(&mut time as *mut _); }

    time
}

pub fn query_performance_frequency() -> i64 {

    let mut frequency = 0;

    unsafe { QueryPerformanceFrequency(&mut frequency as *mut _); }

    frequency
}

type MessageBoxATy = unsafe extern "system" fn(window_handle: WindowHandle, text: *const u8, caption: *const u8, message_type: u32) -> i32;
type RegisterClassATy = unsafe extern "system" fn(window_class: *const WindowClass) -> Atom;
type CreateWindowExATy = unsafe extern "system" fn(ex_style: u32, class_name: *const u8, window_name: *const u8, style: u32, x: i32, y: i32, width: i32, height: i32, parent_winodw: WindowHandle, menu: MenuHandle, instance: InstanceHandle, param: *mut c_void) -> WindowHandle;
type DestroyWindowTy = unsafe extern "system" fn(window_handle: WindowHandle) -> i32;
type GetDCTy = unsafe extern "system" fn(window: WindowHandle) -> DcHandle;
type ReleaseDCTy = unsafe extern "system" fn(window: WindowHandle, dc: DcHandle) -> i32;
type PeekMessageATy = unsafe extern "system" fn(msg: *mut Msg, window_handle: WindowHandle, msg_filter_min: u32, msg_filter_max: u32, remove_message: u32) -> i32;
type TranslateMessageTy = unsafe extern "system" fn(msg: *const Msg) -> i32;
type DispatchMessageATy =  unsafe extern "system" fn(msg: *const Msg) -> i32;
type DefWindowProcATy = unsafe extern "system" fn(window: WindowHandle, message: u32, wparam: usize, lparam: usize) -> usize;
type LoadCursorATy = unsafe extern "system" fn(instance: InstanceHandle, name: usize) -> CursorHandle;
type SetWindowLongPtrATy = unsafe extern "system" fn(window: WindowHandle, index: i32, data: usize) -> usize;
type GetWindowLongPtrATy = unsafe extern "system" fn(window: WindowHandle, index: i32) -> usize;
type GetClientRectTy = unsafe extern "system" fn(window: WindowHandle, rect: *mut Rect) -> i32;
type GetCursorPosTy = unsafe extern "system" fn(point: *mut Point) -> i32;
type ScreenToClientTy = unsafe extern "system" fn(window: WindowHandle, point: *mut Point) -> i32;
type GetKeyStateTy = unsafe extern "system" fn(key_code: i32) -> i16;

const USER_FUNCTION_COUNT: usize = 17;

static mut USER_API: [usize; USER_FUNCTION_COUNT] = [ 0; USER_FUNCTION_COUNT];

static USER_FUNCTION_ORDINALS: [u16; USER_FUNCTION_COUNT] = [
    1501 + 617, // MessageBoxA

    1501 + 700, // RegisterClassA
    1501 + 121, // CreateWindowExA
    1501 + 183, // DestroyWindow

    1501 + 322, // GetDC
    1501 + 733, // ReleaseDC

    1501 + 654, // PeekMessageA
    1501 + 897, // TranslateMessage
    1501 + 190, // DispatchMessageA
    1501 + 170, // DefWindowProcA

    1501 + 577, // LoadCursorA

    1501 + 844, // SetWindowLongPtrA
    1501 + 473, // GetWindowLongPtrA

    1501 + 307, // GetClientRect
    1501 + 321, // GetCursorPos
    1501 + 743, // ScreenToClient

    1501 + 355, // GetKeyState
];

pub fn message_box(text: &[u8], caption: &[u8]) {
    unsafe {
        mem::transmute::<_, MessageBoxATy>(*USER_API.get_unchecked(0))(ptr::null_mut(), &*text.get_unchecked(0), &*caption.get_unchecked(0), 0x00000030);
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
        cursor: load_cursor(ptr::null_mut(), IDC_ARROW),
        background: ptr::null_mut(),
        menu_name: ptr::null(),
        class_name: unsafe { &*name.get_unchecked(0) },
    };

    unsafe { mem::transmute::<_, RegisterClassATy>(*USER_API.get_unchecked(1))(&window_class) != 0 }
}

pub fn create_window(class_name: &[u8], name: &[u8], visible: bool) -> WindowHandle {
    unsafe {
        mem::transmute::<_, CreateWindowExATy>(*USER_API.get_unchecked(2))(
            0,
            &*class_name.get_unchecked(0),
            &*name.get_unchecked(0),
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
    unsafe { mem::transmute::<_, DestroyWindowTy>(*USER_API.get_unchecked(3))(window) }
}

pub fn get_dc(window: WindowHandle) -> DcHandle {
    unsafe { mem::transmute::<_, GetDCTy>(*USER_API.get_unchecked(4))(window) }
}

pub fn release_dc(window: WindowHandle, dc: DcHandle) -> i32 {
    unsafe { mem::transmute::<_, ReleaseDCTy>(*USER_API.get_unchecked(5))(window, dc) }
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

    let msg_result = unsafe { mem::transmute::<_, PeekMessageATy>(*USER_API.get_unchecked(6))(&mut msg, ptr::null_mut(), 0, 0, 1) };

    if msg_result != 0 { Some(msg) } else { None }
}

pub fn translate_and_dispatch_message(msg: &Msg) {
    unsafe {
        mem::transmute::<_, TranslateMessageTy>(*USER_API.get_unchecked(7))(msg as *const Msg);
        mem::transmute::<_, DispatchMessageATy>(*USER_API.get_unchecked(8))(msg as *const Msg);
    }
}

pub fn def_window_proc(window: WindowHandle, message: u32, wparam: usize, lparam: usize) -> usize {

    unsafe { mem::transmute::<_, DefWindowProcATy>(*USER_API.get_unchecked(9))(window, message, wparam, lparam) }
}

pub fn load_cursor(instance: InstanceHandle, name: usize) -> CursorHandle {

    unsafe { mem::transmute::<_, LoadCursorATy>(*USER_API.get_unchecked(10))(instance, name) }
}

pub fn set_window_user_data(window: WindowHandle, data: usize) {

    unsafe { mem::transmute::<_, SetWindowLongPtrATy>(*USER_API.get_unchecked(11))(window, GWLP_USERDATA, data); }
}

pub fn get_window_user_data(window: WindowHandle) -> usize {

    unsafe { mem::transmute::<_, GetWindowLongPtrATy>(*USER_API.get_unchecked(12))(window, GWLP_USERDATA) }
}

pub fn get_window_client_rect(window: WindowHandle) -> (i32, i32, i32, i32) {

    let mut rect: Rect = Default::default();

    unsafe { mem::transmute::<_, GetClientRectTy>(*USER_API.get_unchecked(13))(window, &mut rect as *mut _); }

    (rect.left, rect.top, rect.right - rect.left, rect.bottom - rect.top)
}

pub fn get_mouse_pos(window: WindowHandle) -> (i32, i32) {

    let mut point: Point = Default::default();

    unsafe { mem::transmute::<_, GetCursorPosTy>(*USER_API.get_unchecked(14))(&mut point as *mut _); }
    unsafe { mem::transmute::<_, ScreenToClientTy>(*USER_API.get_unchecked(15))(window, &mut point as *mut _); }    

    (point.x, point.y)
}

pub fn get_key_down(key_code: i32) -> bool {

    unsafe { mem::transmute::<_, GetKeyStateTy>(*USER_API.get_unchecked(16))(key_code) < 0}
}

type ChoosePixelFormatTy = unsafe extern "system" fn(dc: DcHandle, descriptor: *const PixelFormatDescriptor) -> i32;
type DescribePixelFormatTy = unsafe extern "system" fn(dc: DcHandle, pixel_format: i32, bytes: u32, descriptor: *mut PixelFormatDescriptor) -> i32;
type SetPixelFormatTy = unsafe extern "system" fn(dc: DcHandle, pixel_format: i32, descriptor: *const PixelFormatDescriptor) -> i32;
type SwapBuffersTy = unsafe extern "system" fn(dc: DcHandle) -> i32;

const GDI_FUNCTION_COUNT: usize = 4;

static mut GDI_API: [usize; GDI_FUNCTION_COUNT] = [ 0; GDI_FUNCTION_COUNT];

static GDI_FUNCTION_ORDINALS: [u16; GDI_FUNCTION_COUNT] = [
    999 + 45, // ChoosePixelFormat

    999 + 360, // DescribePixelFormat

    999 + 1491, // SetPixelFormat

    999 + 1528, // SwapBuffers
];

pub fn choose_pixel_format(dc: DcHandle, descriptor: &PixelFormatDescriptor) -> i32 {

    unsafe { mem::transmute::<_, ChoosePixelFormatTy>(*GDI_API.get_unchecked(0))(dc, descriptor as *const PixelFormatDescriptor) }
}

pub fn describe_pixel_format(dc: DcHandle,
                             pixel_format: i32,
                             bytes: u32,
                             descriptor: &mut PixelFormatDescriptor)
                             -> i32 {

    unsafe {
        mem::transmute::<_, DescribePixelFormatTy>(*GDI_API.get_unchecked(1))(
            dc,
            pixel_format,
            bytes,
            descriptor as *mut PixelFormatDescriptor)
    }
}

pub fn set_pixel_format(dc: DcHandle,
                        pixel_format: i32,
                        descriptor: *const PixelFormatDescriptor)
                        -> i32 {

    unsafe { mem::transmute::<_, SetPixelFormatTy>(*GDI_API.get_unchecked(2))(dc, pixel_format, descriptor as *const PixelFormatDescriptor) }
}

pub fn swap_buffers(dc: DcHandle) -> bool {

    unsafe { mem::transmute::<_, SwapBuffersTy>(*GDI_API.get_unchecked(3))(dc) != 0 }
}

type WglCreateContextTy = unsafe extern "system" fn(dc: DcHandle) -> GlrcHandle;
type WglDeleteContextTy = unsafe extern "system" fn(glrc: GlrcHandle) -> i32;
type WglMakeCurrentTy = unsafe extern "system" fn(dc: DcHandle, context: GlrcHandle) -> i32;
type WglGetProcAddressTy = unsafe extern "system" fn(name: *const u8) -> Proc;

const GL_FUNCTION_COUNT: usize = 4;

static mut GL_API: [usize; GL_FUNCTION_COUNT] = [ 0; GL_FUNCTION_COUNT];

static GL_FUNCTION_ORDINALS: [u16; GL_FUNCTION_COUNT] = [
    346, // wglCreateContext
    
    348, // wglDeleteContext

    357, // wglMakeCurrent

    356, // wglGetProcAddress
];

pub fn wgl_create_context(dc: DcHandle) -> GlrcHandle {

    unsafe { mem::transmute::<_, WglCreateContextTy>(*GL_API.get_unchecked(0))(dc) }
}

pub fn wgl_delete_context(glrc: GlrcHandle) -> i32 {

    unsafe { mem::transmute::<_, WglDeleteContextTy>(*GL_API.get_unchecked(1))(glrc) }
}

pub fn wgl_make_current(dc: DcHandle, context: GlrcHandle) -> i32 {

    unsafe { mem::transmute::<_, WglMakeCurrentTy>(*GL_API.get_unchecked(2))(dc, context) }
}

pub fn wgl_get_proc_address(name: &[u8]) -> Proc {

    let mut ptr = unsafe { mem::transmute::<_, WglGetProcAddressTy>(*GL_API.get_unchecked(3))(&*name.get_unchecked(0)) };

    if ptr.is_null() {
        ptr = unsafe { get_proc_address_name(OPENGL32, name) };
    }

    utils::debug_trap_if(ptr.is_null());

    ptr
}

type WglGetExtensionsStringEXTTy = unsafe extern "system" fn() -> *const u8;
type WglChoosePixelFormatARBTy = unsafe extern "system" fn(dc: DcHandle, attrib_i_list: *const i32, attrib_f_list: *const f32, max_formats: u32, pixel_formats: *mut i32, num_formats: *mut u32) -> i32;
type WglCreateContextAttribsARBTy = unsafe extern "system" fn(dc: DcHandle, shared_context: GlrcHandle, attrib_list: *const i32) -> GlrcHandle;
type WglSwapIntervalEXTTy = unsafe extern "system" fn(interval: i32) -> i32;

const GL_EXT_FUNCTION_COUNT: usize = 4;

static mut GL_EXT_API: [usize; GL_EXT_FUNCTION_COUNT] = [ 0; GL_EXT_FUNCTION_COUNT];

static GL_EXT_NAMES: [&'static [u8]; GL_EXT_FUNCTION_COUNT] = [
    b"wglGetExtensionsStringEXT\0",

    b"wglChoosePixelFormatARB\0",

    b"wglCreateContextAttribsARB\0",

    b"wglSwapIntervalEXT\0",
];

pub fn wgl_get_extensions_string() -> *const u8 {

    unsafe { mem::transmute::<_, WglGetExtensionsStringEXTTy>(*GL_EXT_API.get_unchecked(0))() }
}

pub fn wgl_choose_pixel_format(dc: DcHandle,
                           attrib_i_list: Option<&[i32]>,
                           attrib_f_list: Option<&[f32]>,
                           pixel_formats: &mut i32,
                           num_formats: &mut u32)
                           -> i32 {

    unsafe {
        mem::transmute::<_, WglChoosePixelFormatARBTy>(*GL_EXT_API.get_unchecked(1))(
                dc,
                if let Some(i_attrib) = attrib_i_list { &*i_attrib.get_unchecked(0) } else { ptr::null() },
                if let Some(f_attrib) = attrib_f_list { &*f_attrib.get_unchecked(0) } else { ptr::null() },
                1,
                pixel_formats as *mut _,
                num_formats as *mut _)
    }
}

pub fn wgl_create_context_attribs(dc: DcHandle,
                              shared_context: GlrcHandle,
                              attrib_list: &[i32])
                              -> GlrcHandle {

    unsafe { mem::transmute::<_, WglCreateContextAttribsARBTy>(*GL_EXT_API.get_unchecked(2))(dc, shared_context, &*attrib_list.get_unchecked(0)) }
}

#[allow(dead_code)]
pub fn wgl_swap_interval(interval: i32) -> i32 {

    unsafe { mem::transmute::<_, WglSwapIntervalEXTTy>(*GL_EXT_API.get_unchecked(3))(interval) }
}

static mut USER32: ModuleHandle = 0 as *mut _;
static mut GDI32: ModuleHandle = 0 as *mut _;
static mut OPENGL32: ModuleHandle = 0 as *mut _;

pub fn init() {

    unsafe {
        USER32 = load_library(b"user32.dll\0");
        GDI32 = load_library(b"Gdi32.dll\0");
        OPENGL32 = load_library(b"Opengl32.dll\0");
    }

    for (i, ordinal) in USER_FUNCTION_ORDINALS.iter().enumerate() {
        unsafe {
            (*USER_API.get_unchecked_mut(i)) = get_proc_address(USER32, *ordinal as isize) as usize;
        }
    }

    for (i, ordinal) in GDI_FUNCTION_ORDINALS.iter().enumerate() {
        unsafe {
            (*GDI_API.get_unchecked_mut(i)) = get_proc_address(GDI32, *ordinal as isize) as usize;
        }
    }

    for (i, ordinal) in GL_FUNCTION_ORDINALS.iter().enumerate() {
        unsafe {
            (*GL_API.get_unchecked_mut(i)) = get_proc_address(OPENGL32, *ordinal as isize) as usize;
        }
    }
}

pub fn wgl_load_extensions() {
    for (i, name) in GL_EXT_NAMES.iter().enumerate() {
        unsafe {
            (*GL_EXT_API.get_unchecked_mut(i)) = wgl_get_proc_address(*name) as usize;
        }
    }
}