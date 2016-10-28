use core::intrinsics::transmute;
use core::ptr;
use win32::Module;
use win32;
use win32_types::*;

pub type GlrcHandle = *mut Void;

#[allow(non_snake_case)]
pub struct Api {

    #[allow(dead_code)]
    opengl32: Module,

    wglCreateContext: unsafe extern "system" fn(dc: DcHandle) -> GlrcHandle,
}

impl Api {

     pub fn new() -> Api {
        
        let opengl32 = Module::new(b"Opengl32.dll\0").unwrap();

        Api {
            
            wglCreateContext: load_proc!(opengl32,  b"wglCreateContext\0"),

            opengl32: opengl32,
        }
    }

    pub fn create_context(&self, w32: &win32::Api) {



    }
}