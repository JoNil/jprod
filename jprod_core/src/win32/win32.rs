use crate::{c_types::*, utils, win32::types::*};
use core::{mem, ptr};

#[link(name = "kernel32")]
#[allow(dead_code)]
unsafe extern "system" {
    unsafe fn OutputDebugStringA(output_string: *const u8);
    unsafe fn ExitProcess(exit_code: u32) -> !;
    unsafe fn GetModuleHandleA(module_name: *const u8) -> ModuleHandle;

    unsafe fn LoadLibraryA(file_name: *const u8) -> ModuleHandle;
    unsafe fn FreeLibrary(module: ModuleHandle);
    unsafe fn GetProcAddress(module: ModuleHandle, proc_name: *const u8) -> Proc;

    unsafe fn GetFileAttributesExA(
        file_name: *const u8,
        info_level_id: i32,
        file_information: *mut FileAttributeData,
    ) -> i32;
    unsafe fn CompareFileTime(file_time_1: *const Filetime, file_time_2: *const Filetime) -> isize;

    unsafe fn CreateFileA(
        file_name: *const u8,
        desired_access: u32,
        share_mode: u32,
        security_attributes: *const c_void,
        creation_disposition: u32,
        flags_and_attributes: u32,
        template_file: Handle,
    ) -> Handle;
    unsafe fn CloseHandle(handle: Handle) -> i32;
    unsafe fn GetFileSize(handle: Handle, file_size_high: *mut u32) -> u32;
    unsafe fn ReadFile(
        handle: Handle,
        buffer: *mut c_void,
        bytes_to_read: u32,
        bytes_read: *mut u32,
        overlapped: *mut c_void,
    ) -> i32;

    unsafe fn VirtualAlloc(
        base_address: *mut c_void,
        size: usize,
        allocation_type: u32,
        protect: u32,
    ) -> *mut c_void;
    unsafe fn VirtualFree(address: *mut c_void, size: usize, free_type: u32) -> i32;

    unsafe fn QueryPerformanceCounter(time: *mut i64) -> i32;
    unsafe fn QueryPerformanceFrequency(frequency: *mut i64) -> i32;
}

#[inline]
pub fn output_debug_string(string: &[u8]) {
    unsafe {
        OutputDebugStringA(string.get_unchecked(0));
    }
}

/// # Safety
///
/// string has to be a valid pointer to a null terminated c string.
#[inline]
pub unsafe fn output_debug_string_raw(string: *const u8) {
    unsafe { OutputDebugStringA(string) };
}

#[inline]
pub fn exit_process(exit_code: u32) -> ! {
    unsafe {
        ExitProcess(exit_code);
    }
}

#[inline]
pub fn get_module_handle(module_name: &[u8]) -> ModuleHandle {
    unsafe { GetModuleHandleA(module_name.get_unchecked(0)) }
}

#[inline]
pub fn load_library(file_name: &[u8]) -> ModuleHandle {
    let handle = unsafe { LoadLibraryA(file_name.get_unchecked(0)) };

    utils::assert(!handle.is_null());

    handle
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn free_library(module: ModuleHandle) {
    unsafe {
        FreeLibrary(module);
    }
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn get_proc_address(module: ModuleHandle, proc_index: isize) -> Proc {
    let ptr = unsafe { GetProcAddress(module, proc_index as *const u8) };

    utils::assert(!ptr.is_null());

    ptr
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn get_proc_address_name(module: ModuleHandle, name: &[u8]) -> Proc {
    unsafe { GetProcAddress(module, name.get_unchecked(0)) }
}

#[inline]
pub fn get_file_attributes(
    file_name: &[u8],
    info_level_id: i32,
    file_information: &mut FileAttributeData,
) -> i32 {
    unsafe {
        GetFileAttributesExA(
            file_name.get_unchecked(0),
            info_level_id,
            file_information as *mut _,
        )
    }
}

#[inline]
pub fn open_file(file_name: &[u8]) -> Handle {
    unsafe {
        CreateFileA(
            file_name.get_unchecked(0),
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            ptr::null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            ptr::null_mut(),
        )
    }
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn close_file(handle: Handle) {
    unsafe {
        CloseHandle(handle);
    };
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn get_file_size(handle: Handle) -> u64 {
    let mut size_high: u32 = 0;

    let size_low = unsafe { GetFileSize(handle, &mut size_high as *mut u32) };

    ((size_high as u64) << 32) | (size_low as u64)
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn read_file(handle: Handle, buffer: &mut [u8]) -> i32 {
    unsafe {
        ReadFile(
            handle,
            &mut *buffer.get_unchecked_mut(0) as *mut u8 as *mut c_void,
            buffer.len() as u32,
            ptr::null_mut(),
            ptr::null_mut(),
        )
    }
}

#[inline]
pub fn compare_file_time(file_time_1: &Filetime, file_time_2: &Filetime) -> isize {
    unsafe { CompareFileTime(file_time_1 as *const _, file_time_2 as *const _) }
}

#[inline]
pub fn virtual_alloc(size: usize) -> *mut c_void {
    unsafe {
        VirtualAlloc(
            ptr::null_mut(),
            size,
            MEM_RESERVE | MEM_COMMIT,
            PAGE_READWRITE,
        )
    }
}

/// # Safety
///
/// address has to be a valid pointer
#[inline]
pub unsafe fn virtual_free(address: *mut c_void) {
    unsafe {
        VirtualFree(address, 0, MEM_RELEASE);
    }
}

#[inline]
pub fn query_performance_counter() -> i64 {
    let mut time = 0;

    unsafe {
        QueryPerformanceCounter(&mut time as *mut _);
    }

    time
}

#[inline]
pub fn query_performance_frequency() -> i64 {
    let mut frequency = 0;

    unsafe {
        QueryPerformanceFrequency(&mut frequency as *mut _);
    }

    frequency
}

#[link(name = "user32")]
#[allow(dead_code)]
unsafe extern "system" {
    unsafe fn MessageBoxA(
        window_handle: WindowHandle,
        text: *const u8,
        caption: *const u8,
        message_type: u32,
    ) -> i32;
    unsafe fn RegisterClassA(window_class: *const WindowClass) -> Atom;
    unsafe fn CreateWindowExA(
        ex_style: u32,
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
        param: *mut c_void,
    ) -> WindowHandle;
    unsafe fn DestroyWindow(window_handle: WindowHandle) -> i32;
    unsafe fn GetDC(window: WindowHandle) -> DcHandle;
    unsafe fn ReleaseDC(window: WindowHandle, dc: DcHandle) -> i32;
    unsafe fn PeekMessageA(
        msg: *mut Msg,
        window_handle: WindowHandle,
        msg_filter_min: u32,
        msg_filter_max: u32,
        remove_message: u32,
    ) -> i32;
    unsafe fn TranslateMessage(msg: *const Msg) -> i32;
    unsafe fn DispatchMessageA(msg: *const Msg) -> i32;
    unsafe fn DefWindowProcA(
        window: WindowHandle,
        message: u32,
        wparam: usize,
        lparam: usize,
    ) -> usize;
    unsafe fn LoadCursorA(instance: InstanceHandle, name: usize) -> CursorHandle;
    unsafe fn GetClientRect(window: WindowHandle, rect: *mut Rect) -> i32;
    unsafe fn GetCursorPos(point: *mut Point) -> i32;
    unsafe fn ScreenToClient(window: WindowHandle, point: *mut Point) -> i32;
    unsafe fn GetKeyState(key_code: i32) -> i16;
}

#[inline]
pub fn message_box(text: &[u8], caption: &[u8]) {
    unsafe {
        MessageBoxA(
            ptr::null_mut(),
            text.get_unchecked(0),
            caption.get_unchecked(0),
            0x00000030,
        );
    }
}

#[inline]
pub fn register_class(name: &[u8], window_proc: WindowProc) -> bool {
    let window_class = WindowClass {
        style: CS_VREDRAW | CS_HREDRAW | CS_OWNDC,
        window_proc,
        cls_extra: 0,
        wnd_extra: 0,
        instance: unsafe { GetModuleHandleA(ptr::null_mut()) },
        icon: ptr::null_mut(),
        cursor: load_cursor(ptr::null_mut(), IDC_ARROW),
        background: ptr::null_mut(),
        menu_name: ptr::null(),
        class_name: unsafe { name.get_unchecked(0) },
    };

    unsafe { RegisterClassA(&window_class) != 0 }
}

#[inline]
pub fn create_window(class_name: &[u8], name: &[u8], visible: bool) -> WindowHandle {
    unsafe {
        CreateWindowExA(
            0,
            class_name.get_unchecked(0),
            name.get_unchecked(0),
            WS_OVERLAPPEDWINDOW | if visible { WS_VISIBLE } else { 0 },
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            ptr::null_mut(),
            ptr::null_mut(),
            GetModuleHandleA(ptr::null_mut()),
            ptr::null_mut(),
        )
    }
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn destroy_window(window: WindowHandle) -> i32 {
    unsafe { DestroyWindow(window) }
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn get_dc(window: WindowHandle) -> DcHandle {
    unsafe { GetDC(window) }
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn release_dc(window: WindowHandle, dc: DcHandle) -> i32 {
    unsafe { ReleaseDC(window, dc) }
}

#[inline]
pub fn get_message() -> Option<Msg> {
    let mut msg: Msg = Msg {
        window_handle: ptr::null_mut(),
        message: 0,
        wparam: 0,
        lparam: 0,
        time: 0,
        point: Point { x: 0, y: 0 },
    };

    let msg_result = unsafe { PeekMessageA(&mut msg, ptr::null_mut(), 0, 0, 1) };

    if msg_result != 0 { Some(msg) } else { None }
}

#[inline]
pub fn translate_and_dispatch_message(msg: &Msg) {
    unsafe {
        TranslateMessage(msg as *const Msg);
        DispatchMessageA(msg as *const Msg);
    }
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn def_window_proc(window: WindowHandle, message: u32, wparam: usize, lparam: usize) -> usize {
    unsafe { DefWindowProcA(window, message, wparam, lparam) }
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn load_cursor(instance: InstanceHandle, name: usize) -> CursorHandle {
    unsafe { LoadCursorA(instance, name) }
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn get_window_client_rect(window: WindowHandle) -> (i32, i32, i32, i32) {
    let mut rect: Rect = Default::default();

    unsafe {
        GetClientRect(window, &mut rect as *mut _);
    }

    (
        rect.left,
        rect.top,
        rect.right - rect.left,
        rect.bottom - rect.top,
    )
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn get_mouse_pos(window: WindowHandle) -> (i32, i32) {
    let mut point: Point = Default::default();

    unsafe {
        GetCursorPos(&mut point as *mut _);
    }
    unsafe {
        ScreenToClient(window, &mut point as *mut _);
    }

    (point.x, point.y)
}

#[inline]
pub fn get_key_down(key_code: i32) -> bool {
    unsafe { GetKeyState(key_code) < 0 }
}

#[link(name = "gdi32")]
#[allow(dead_code)]
unsafe extern "system" {
    unsafe fn ChoosePixelFormat(dc: DcHandle, descriptor: *const PixelFormatDescriptor) -> i32;
    unsafe fn DescribePixelFormat(
        dc: DcHandle,
        pixel_format: i32,
        bytes: u32,
        descriptor: *mut PixelFormatDescriptor,
    ) -> i32;
    unsafe fn SetPixelFormat(
        dc: DcHandle,
        pixel_format: i32,
        descriptor: *const PixelFormatDescriptor,
    ) -> i32;
    unsafe fn SwapBuffers(dc: DcHandle) -> i32;
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn choose_pixel_format(dc: DcHandle, descriptor: &PixelFormatDescriptor) -> i32 {
    unsafe { ChoosePixelFormat(dc, descriptor as *const PixelFormatDescriptor) }
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn describe_pixel_format(
    dc: DcHandle,
    pixel_format: i32,
    bytes: u32,
    descriptor: *mut PixelFormatDescriptor,
) -> i32 {
    unsafe { DescribePixelFormat(dc, pixel_format, bytes, descriptor) }
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn set_pixel_format(
    dc: DcHandle,
    pixel_format: i32,
    descriptor: *const PixelFormatDescriptor,
) -> i32 {
    unsafe { SetPixelFormat(dc, pixel_format, descriptor) }
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn swap_buffers(dc: DcHandle) -> bool {
    unsafe { SwapBuffers(dc) != 0 }
}

#[link(name = "opengl32")]
#[allow(dead_code)]
unsafe extern "system" {
    unsafe fn wglCreateContext(dc: DcHandle) -> GlrcHandle;
    unsafe fn wglDeleteContext(glrc: GlrcHandle) -> i32;
    unsafe fn wglMakeCurrent(dc: DcHandle, context: GlrcHandle) -> i32;
    unsafe fn wglGetProcAddress(name: *const u8) -> Proc;
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn wgl_create_context(dc: DcHandle) -> GlrcHandle {
    unsafe { wglCreateContext(dc) }
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn wgl_delete_context(glrc: GlrcHandle) -> i32 {
    unsafe { wglDeleteContext(glrc) }
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn wgl_make_current(dc: DcHandle, context: GlrcHandle) -> i32 {
    unsafe { wglMakeCurrent(dc, context) }
}

#[inline]
pub fn wgl_get_proc_address(name: &[u8]) -> Proc {
    let mut ptr = unsafe { wglGetProcAddress(name.get_unchecked(0)) };

    if ptr.is_null() {
        ptr = unsafe { get_proc_address_name(OPENGL32, name) };
    }

    utils::assert(!ptr.is_null());

    ptr
}

type WglGetExtensionsStringEXTTy = unsafe extern "system" fn() -> *const u8;
type WglChoosePixelFormatARBTy = unsafe extern "system" fn(
    dc: DcHandle,
    attrib_i_list: *const i32,
    attrib_f_list: *const f32,
    max_formats: u32,
    pixel_formats: *mut i32,
    num_formats: *mut u32,
) -> i32;
type WglCreateContextAttribsARBTy = unsafe extern "system" fn(
    dc: DcHandle,
    shared_context: GlrcHandle,
    attrib_list: *const i32,
) -> GlrcHandle;
type WglSwapIntervalEXTTy = unsafe extern "system" fn(interval: i32) -> i32;

const GL_EXT_FUNCTION_COUNT: usize = 4;

static mut GL_EXT_API: [usize; GL_EXT_FUNCTION_COUNT] = [0; GL_EXT_FUNCTION_COUNT];

static GL_EXT_NAMES: [&[u8]; GL_EXT_FUNCTION_COUNT] = [
    b"wglGetExtensionsStringEXT\0",
    b"wglChoosePixelFormatARB\0",
    b"wglCreateContextAttribsARB\0",
    b"wglSwapIntervalEXT\0",
];

#[inline]
pub fn wgl_get_extensions_string() -> *const u8 {
    unsafe { mem::transmute::<_, WglGetExtensionsStringEXTTy>(*GL_EXT_API.get_unchecked(0))() }
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn wgl_choose_pixel_format(
    dc: DcHandle,
    attrib_i_list: Option<&[i32]>,
    attrib_f_list: Option<&[f32]>,
    pixel_formats: &mut i32,
    num_formats: &mut u32,
) -> i32 {
    unsafe {
        mem::transmute::<_, WglChoosePixelFormatARBTy>(*GL_EXT_API.get_unchecked(1))(
            dc,
            if let Some(i_attrib) = attrib_i_list {
                i_attrib.get_unchecked(0)
            } else {
                ptr::null()
            },
            if let Some(f_attrib) = attrib_f_list {
                f_attrib.get_unchecked(0)
            } else {
                ptr::null()
            },
            1,
            pixel_formats as *mut _,
            num_formats as *mut _,
        )
    }
}

#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn wgl_create_context_attribs(
    dc: DcHandle,
    shared_context: GlrcHandle,
    attrib_list: &[i32],
) -> GlrcHandle {
    unsafe {
        mem::transmute::<_, WglCreateContextAttribsARBTy>(*GL_EXT_API.get_unchecked(2))(
            dc,
            shared_context,
            attrib_list.get_unchecked(0),
        )
    }
}

#[inline]
pub fn wgl_swap_interval(interval: i32) -> i32 {
    unsafe { mem::transmute::<_, WglSwapIntervalEXTTy>(*GL_EXT_API.get_unchecked(3))(interval) }
}

static mut OPENGL32: ModuleHandle = 0 as *mut _;

#[inline]
pub fn init() {
    unsafe {
        OPENGL32 = load_library(b"Opengl32.dll\0");
    }
}

#[inline]
pub fn wgl_load_extensions() {
    for (i, name) in GL_EXT_NAMES.iter().enumerate() {
        unsafe {
            (*GL_EXT_API.get_unchecked_mut(i)) = wgl_get_proc_address(name) as usize;
        }
    }
}
