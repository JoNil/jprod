use core::marker::PhantomData;
use core::ptr;
use super::Context;
use super::gl;
use utils;

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum Format {
    RgbF32,
    RgbF16,
    RgbaF16,
    RgbR11G11B10,
    DepthF32,
}

struct GlEnums {
    internal_format: u32,
    format: u32,
    component_type: u32,
}

impl Format {
    fn get_gl_enums(self) -> GlEnums {
        match self {
            Format::RgbF32 => {
                GlEnums {
                    internal_format: gl::RGB32F,
                    format: gl::RGB,
                    component_type: gl::FLOAT,
                }
            }
            Format::RgbF16 => {
                GlEnums {
                    internal_format: gl::RGB16F,
                    format: gl::RGB,
                    component_type: gl::FLOAT,
                }
            },
            Format::RgbaF16 => {
                GlEnums {
                    internal_format: gl::RGBA16F,
                    format: gl::RGBA,
                    component_type: gl::FLOAT,
                }
            },
            Format::RgbR11G11B10 => {
                GlEnums {
                    internal_format: gl::R11F_G11F_B10F,
                    format: gl::RGB,
                    component_type: gl::FLOAT,
                }
            }
            Format::DepthF32 => {
                GlEnums {
                    internal_format: gl::DEPTH_COMPONENT32F,
                    format: gl::DEPTH_COMPONENT,
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

        utils::assert(handle != 0);

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
    format: Option<Format>,
}

impl Texture {
    pub fn new(_: &Context) -> Texture {
        let texture = RawTexture::new();

        Texture {
            texture: texture,
            format: None
        }
    }

    pub fn allocate(&mut self, size: (i32, i32), format: Format) {

        tm_zone!("Texture::allocate");

        self.format = Some(format);

        let enums = format.get_gl_enums();

        unsafe {

            gl::BindTexture(gl::TEXTURE_2D, self.texture.handle);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                enums.internal_format as i32,
                size.0,
                size.1,
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

    pub fn get_format(&self) -> Option<Format> {
        self.format
    }

    pub(super) fn get_handle(&self) -> u32 {

        utils::assert(self.format.is_some());

        self.texture.handle
    }
}