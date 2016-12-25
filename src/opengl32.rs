#![allow(dead_code)]

use core::ptr;
use module::Module;
use win32;
use win32_types::*;

#[allow(non_snake_case)]
struct Api {
    #[allow(dead_code)]
    opengl32: Module,

    wglCreateContext: unsafe extern "system" fn(dc: DcHandle) -> GlrcHandle,
    wglDeleteContext: unsafe extern "system" fn(glrc: GlrcHandle) -> i32,

    wglMakeCurrent: unsafe extern "system" fn(dc: DcHandle, context: GlrcHandle) -> i32,

    wglGetProcAddress: unsafe extern "system" fn(name: *const u8) -> Proc,
}

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

fn init() -> Api {

    if let Some(opengl32) = Module::new(b"Opengl32.dll\0") {

        unsafe {
            return Api {
                wglCreateContext: load_proc!(opengl32, 346),
                wglDeleteContext: load_proc!(opengl32, 348),

                wglMakeCurrent: load_proc!(opengl32, 357),

                wglGetProcAddress: load_proc!(opengl32, 356),

                opengl32: opengl32,
            }
        }
    } else {
        win32::debug_break();
    }
}

lazy_static! {
    static ref API: Api = init();
}

macro_rules! wgl_load_proc {
    ($name:expr) => {
        {
            use core;

            let procedure = get_proc_address($name);
            if procedure == ptr::null_mut() {
                win32::debug_break();
            }
            unsafe { core::intrinsics::transmute(procedure) }
        }
    }
}

fn init_ext() -> ExtApi {
    ExtApi {
        wglGetExtensionsStringEXT: wgl_load_proc!(b"wglGetExtensionsStringEXT\0"),

        wglChoosePixelFormatARB: wgl_load_proc!(b"wglChoosePixelFormatARB\0"),

        wglCreateContextAttribsARB: wgl_load_proc!(b"wglCreateContextAttribsARB\0"),

        wglSwapIntervalEXT: wgl_load_proc!(b"wglSwapIntervalEXT\0"),
    }
}

lazy_static! {
    static ref EXT_API: ExtApi = init_ext();
}

pub fn load_extensions() {
    &*EXT_API;
}

pub fn create_context(dc: DcHandle) -> GlrcHandle {

    unsafe { (API.wglCreateContext)(dc) }
}

pub fn delete_context(glrc: GlrcHandle) -> i32 {

    unsafe { (API.wglDeleteContext)(glrc) }
}

pub fn make_current(dc: DcHandle, context: GlrcHandle) -> i32 {

    unsafe { (API.wglMakeCurrent)(dc, context) }
}

pub fn get_proc_address(name: &[u8]) -> Proc {

    unsafe { (API.wglGetProcAddress)(&name[0]) }
}

pub fn get_extensions_string() -> *const u8 {

    unsafe { (EXT_API.wglGetExtensionsStringEXT)() }
}

pub fn choose_pixel_format(dc: DcHandle,
                           attrib_i_list: Option<&[i32]>,
                           attrib_f_list: Option<&[f32]>,
                           pixel_formats: &mut i32,
                           num_formats: &mut u32)
                           -> i32 {

    unsafe {
        (EXT_API.wglChoosePixelFormatARB)(
                dc,
                if let Some(i_attrib) = attrib_i_list { &i_attrib[0] } else { ptr::null() },
                if let Some(f_attrib) = attrib_f_list { &f_attrib[0] } else { ptr::null() },
                1,
                pixel_formats as *mut _,
                num_formats as *mut _)
    }
}

pub fn create_context_attribs(dc: DcHandle,
                              shared_context: GlrcHandle,
                              attrib_list: &[i32])
                              -> GlrcHandle {

    unsafe { (EXT_API.wglCreateContextAttribsARB)(dc, shared_context, &attrib_list[0]) }
}

#[allow(dead_code)]
pub fn swap_interval(interval: i32) -> i32 {

    unsafe { (EXT_API.wglSwapIntervalEXT)(interval) }
}
