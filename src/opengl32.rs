use module::Module;
use c_types::*;
use win32_types::*;

pub type GlrcHandle = *mut c_void;

static mut API: Option<Api> = None;

#[allow(non_snake_case)]
struct ExtApi {
    wglGetExtensionsStringEXT: unsafe extern "system" fn() -> *const u8,

    wglChoosePixelFormatARB: unsafe extern "system" fn(dc: DcHandle,
                                                       attrib_i_list: *const i32,
                                                       attrib_f_list: *const f32,
                                                       max_formats: u32,
                                                       pixel_formats: *mut i32,
                                                       num_formats: *mut u32)
                                                       -> i32,

    wglCreateContextAttribsARB: unsafe extern "system" fn(dc: DcHandle,
                                                          shared_context: GlrcHandle,
                                                          attrib_list: *const i32)
                                                          -> GlrcHandle,

    wglSwapIntervalEXT: unsafe extern "system" fn(interval: i32) -> i32,
}

#[allow(non_snake_case)]
struct Api {
    #[allow(dead_code)]
    opengl32: Module,

    wglCreateContext: unsafe extern "system" fn(dc: DcHandle) -> GlrcHandle,
    wglDeleteContext: unsafe extern "system" fn(glrc: GlrcHandle) -> i32,

    wglMakeCurrent: unsafe extern "system" fn(dc: DcHandle, context: GlrcHandle) -> i32,

    wglGetProcAddress: unsafe extern "system" fn(name: *const u8) -> Proc,

    ext: Option<ExtApi>,
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

#[inline]
fn ext_api() -> &'static ExtApi {
    if let Some(ref api) = api().ext {
        api
    } else {
        panic!();
    }
}

pub fn init() {

    if let Some(opengl32) = Module::new(b"Opengl32.dll\0") {

        unsafe {
            API = Some(Api {
                wglCreateContext: load_proc!(opengl32, 346),
                wglDeleteContext : load_proc!(opengl32, 348),

                wglMakeCurrent: load_proc!(opengl32, 357),

                wglGetProcAddress: load_proc!(opengl32, 356),

                ext: None,

                opengl32: opengl32,
            })
        }
    } else {
        panic!();
    }
}

macro_rules! wgl_load_proc {
    ($name:expr) => {
        {
            use core;

            let procedure = get_proc_address($name);
            if procedure == core::ptr::null_mut() {
                panic!();
            }
            #[allow(unused_unsafe)]
            unsafe { core::intrinsics::transmute(procedure) }
        }
    }
}

pub fn load_extensions() {

    unsafe {

        if let Some(ref mut api) = API {

            api.ext = Some(ExtApi {
                wglGetExtensionsStringEXT: wgl_load_proc!(b"wglGetExtensionsStringEXT\0"),

                wglChoosePixelFormatARB: wgl_load_proc!(b"wglChoosePixelFormatARB\0"),

                wglCreateContextAttribsARB: wgl_load_proc!(b"wglCreateContextAttribsARB\0"),

                wglSwapIntervalEXT: wgl_load_proc!(b"wglSwapIntervalEXT\0"),
            });

        } else {
            panic!();
        }
    }
}

pub fn create_context(dc: DcHandle) -> GlrcHandle {

    unsafe { (api().wglCreateContext)(dc) }
}

pub fn delete_context(glrc: GlrcHandle) -> i32 {

    unsafe { (api().wglDeleteContext)(glrc) }
}

pub fn make_current(dc: DcHandle, context: GlrcHandle) -> i32 {

    unsafe { (api().wglMakeCurrent)(dc, context) }
}

pub fn get_proc_address(name: &[u8]) -> Proc {

    unsafe { (api().wglGetProcAddress)(&name[0]) }
}

pub fn get_extensions_string() -> *const u8 {

    unsafe { (ext_api().wglGetExtensionsStringEXT)() }
}

pub fn choose_pixel_format(dc: DcHandle,
                           attrib_i_list: *const i32,
                           attrib_f_list: *const f32,
                           max_formats: u32,
                           pixel_formats: *mut i32,
                           num_formats: *mut u32)
                           -> i32 {

    unsafe {
        (ext_api().wglChoosePixelFormatARB)(dc,
                                            attrib_i_list,
                                            attrib_f_list,
                                            max_formats,
                                            pixel_formats,
                                            num_formats)
    }
}

pub fn create_context_attribs(dc: DcHandle,
                              shared_context: GlrcHandle,
                              attrib_list: *const i32)
                              -> GlrcHandle {

    unsafe { (ext_api().wglCreateContextAttribsARB)(dc, shared_context, attrib_list) }
}

pub fn swap_interval(interval: i32) -> i32 {

    unsafe { (ext_api().wglSwapIntervalEXT)(interval) }
}
