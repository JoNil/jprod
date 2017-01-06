use core::marker::PhantomData;
use core::ptr;
use gl;
use utils;
use window::GlContext;

pub enum TextureFormat {
    Rgb_f32,
}

struct GlEnums {
    internal_format: u32,
    format: u32,
    component_type: u32,
}

impl TextureFormat {
    fn get_gl_enums(&self) -> GlEnums {
        match self {
            &TextureFormat::Rgb_f32 => {
                GlEnums {
                    internal_format: gl::RGB32F,
                    format: gl::RGB,
                    component_type: gl::FLOAT,
                }
            }
        }
    }
}

struct RawTexture {
    handle: u32,
    marker: PhantomData<*const u32>,
}

impl RawTexture {
    fn new() -> RawTexture {

        let mut handle = 0;
        unsafe { gl::GenTextures(1, &mut handle as *mut _) };

        utils::debug_trap_if(handle == 0);

        RawTexture { handle: handle, marker: PhantomData }
    }
}

impl Drop for RawTexture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &mut self.handle as *mut _) };
    }
}

pub struct Texture {
    texture: RawTexture,
}

impl Texture {
    pub fn new(_: &GlContext) -> Texture {
        let texture = RawTexture::new();

        Texture {
            texture: texture,
        }
    }

    pub fn allocate(&mut self, width: i32, height: i32, format: TextureFormat) {

        let enums = format.get_gl_enums();

        unsafe {

            gl::BindTexture(gl::TEXTURE_2D, self.texture.handle);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                enums.internal_format as i32,
                width,
                height,
                0,
                enums.format,
                enums.component_type,
                ptr::null_mut());

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER , gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}