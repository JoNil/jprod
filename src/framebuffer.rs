use core::marker::PhantomData;
use gl;
use utils;
use window::GlContext;

struct RawFramebuffer {
    handle: u32,
    marker: PhantomData<*const u32>,
}

impl RawFramebuffer {
    pub fn new() -> RawFramebuffer {

        let mut handle = 0;

        unsafe { gl::GenFramebuffers(1, &mut handle as *mut _) };

        utils::debug_trap_if(handle == 0);

        RawFramebuffer { handle: handle, marker: PhantomData }
    }
}

impl Drop for RawFramebuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteFramebuffers(1, &mut self.handle as *mut _) };
    }
}

pub struct Framebuffer {
    framebuffer: RawFramebuffer,
}

impl Framebuffer {
    pub fn new(_: &GlContext) -> Framebuffer {
        let framebuffer = RawFramebuffer::new();

        Framebuffer {
            framebuffer: framebuffer,
        }
    }
}