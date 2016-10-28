use win32::Module;
use win32_types::*;

pub type GlrcHandle = *mut Void;

static mut API: Option<Api> = None;

#[allow(non_snake_case)]
pub struct Api {

    #[allow(dead_code)]
    opengl32: Module,

    wglCreateContext: unsafe extern "system" fn(dc: DcHandle) -> GlrcHandle,
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

                opengl32: opengl32,
            })
        }
    } else {
        panic!();
    }
}

pub fn create_context() {

}