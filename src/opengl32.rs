use core::mem;
use module::Module;
use win32;
use win32_types::*;

pub type GlrcHandle = *mut Void;

static mut API: Option<Api> = None;

#[allow(non_snake_case)]
struct Api {

    #[allow(dead_code)]
    opengl32: Module,

    wglCreateContext: unsafe extern "system" fn(dc: DcHandle) -> GlrcHandle,

    wglMakeCurrent: unsafe extern "system" fn(dc: DcHandle, context: GlrcHandle) -> i32,

    wglGetProcAddress: unsafe extern "system" fn(name: *const u8) -> Proc,

    //wglChoosePixelFormatARB: Option<>,

    //wglCreateContextAttribsARB: Option<>,

    //wglSwapIntervalEXT: Option<>,

    wglGetExtensionsStringEXT: Option<unsafe extern "system" fn() -> *const u8>,
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
    
    if let Some(opengl32) = Module::new(b"Opengl32.dll\0") {

        unsafe {
            API = Some(Api {
                
                wglCreateContext: load_proc!(opengl32, 346),

                wglMakeCurrent: load_proc!(opengl32, 357),

                wglGetProcAddress: load_proc!(opengl32, 356),

                wglGetExtensionsStringEXT: None,

                opengl32: opengl32,
            })
        }
    } else {
        panic!();
    }
}

pub fn load_extensions() {

    unsafe {
        if let Some(ref mut api) = API {
            
            api.wglGetExtensionsStringEXT = Some(mem::transmute(get_proc_address(b"wglGetExtensionsStringEXT\0")));

            win32::output_debug_string_raw((api.wglGetExtensionsStringEXT.unwrap())());


        } else {
            panic!();
        }
    }
}

pub fn create_context(dc: DcHandle) -> GlrcHandle {
    
    unsafe { (api().wglCreateContext)(dc) }
}

pub fn make_current(dc: DcHandle, context: GlrcHandle) -> i32 {
    
    unsafe { (api().wglMakeCurrent)(dc, context) }
}

pub fn get_proc_address(name: &[u8]) -> Proc {
    
    unsafe { (api().wglGetProcAddress)(&name[0]) }
}