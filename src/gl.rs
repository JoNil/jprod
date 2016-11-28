#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_upper_case_globals)]

use c_types::*;
use core::mem;
use win32;

#[inline(never)]
fn metaloadfn(mut loadfn: &mut FnMut(&[u8]) -> *const c_void, symbol: &[u8], fallbacks: &[&[u8]]) -> *const c_void {
    let mut ptr = loadfn(symbol);
    if ptr.is_null() {
        for &sym in fallbacks {
            ptr = loadfn(sym);
            if !ptr.is_null() {
                break;
            }
        }
    }
    if ptr.is_null() {
        win32::debug_break();
    }
    ptr
}

pub mod types {

    #![allow(non_camel_case_types, non_snake_case, dead_code, missing_copy_implementations)]

    use c_types::*;

    // Common types from OpenGL 1.1
    pub type GLenum = c_uint;
    pub type GLboolean = c_uchar;
    pub type GLbitfield = c_uint;
    pub type GLvoid = c_void;
    pub type GLbyte = c_char;
    pub type GLshort = c_short;
    pub type GLint = c_int;
    pub type GLclampx = c_int;
    pub type GLubyte = c_uchar;
    pub type GLushort = c_ushort;
    pub type GLuint = c_uint;
    pub type GLsizei = c_int;
    pub type GLfloat = c_float;
    pub type GLclampf = c_float;
    pub type GLdouble = c_double;
    pub type GLclampd = c_double;
    pub type GLeglImageOES = *const c_void;
    pub type GLchar = c_char;
    pub type GLcharARB = c_char;

    #[cfg(target_os = "macos")]
    pub type GLhandleARB = *const c_void;
    #[cfg(not(target_os = "macos"))]
    pub type GLhandleARB = c_uint;

    pub type GLhalfARB = c_ushort;
    pub type GLhalf = c_ushort;

    // Must be 32 bits
    pub type GLfixed = GLint;

    pub type GLintptr = isize;
    pub type GLsizeiptr = isize;
    pub type GLint64 = i64;
    pub type GLuint64 = u64;
    pub type GLintptrARB = isize;
    pub type GLsizeiptrARB = isize;
    pub type GLint64EXT = i64;
    pub type GLuint64EXT = u64;

    pub enum __GLsync {}
    pub type GLsync = *const __GLsync;

    // compatible with OpenCL cl_context
    pub enum _cl_context {}
    pub enum _cl_event {}

    pub type GLDEBUGPROC = extern "system" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut c_void);
    pub type GLDEBUGPROCARB = extern "system" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut c_void);
    pub type GLDEBUGPROCKHR = extern "system" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut c_void);

    // Vendor extension types
    pub type GLDEBUGPROCAMD = extern "system" fn(id: GLuint, category: GLenum, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut c_void);
    pub type GLhalfNV = c_ushort;
    pub type GLvdpauSurfaceNV = GLintptr;
}

#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_ALPHA_TO_COVERAGE: types::GLenum = 0x809E;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_2D: types::GLenum = 0x9058;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_SUBROUTINE_UNIFORM: types::GLenum = 0x92F1;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: types::GLenum = 0x906B;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_PATCH_COMPONENTS: types::GLenum = 0x8E84;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPARE_MODE: types::GLenum = 0x884C;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_24_8: types::GLenum = 0x84FA;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_LSB_FIRST: types::GLenum = 0x0CF1;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_SEVERITY_MEDIUM: types::GLenum = 0x9147;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT18: types::GLenum = 0x8CF2;
#[allow(dead_code, non_upper_case_globals)]
pub const LOCATION: types::GLenum = 0x930E;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_TYPE_POP_GROUP: types::GLenum = 0x826A;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_STREAMS: types::GLenum = 0x8E71;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE25: types::GLenum = 0x84D9;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_CUBE: types::GLenum = 0x8DCC;
#[allow(dead_code, non_upper_case_globals)]
pub const HIGH_FLOAT: types::GLenum = 0x8DF2;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_RECTANGLE_TEXTURE_SIZE: types::GLenum = 0x84F8;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_IMAGE_TYPE: types::GLenum = 0x8290;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAMEBUFFER_HEIGHT: types::GLenum = 0x9316;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_VEC4: types::GLenum = 0x8DC8;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_GATHER_SHADOW: types::GLenum = 0x82A3;
#[allow(dead_code, non_upper_case_globals)]
pub const CONDITION_SATISFIED: types::GLenum = 0x911C;
#[allow(dead_code, non_upper_case_globals)]
pub const DITHER: types::GLenum = 0x0BD0;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_SHADING: types::GLenum = 0x8C36;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_REF: types::GLenum = 0x0B97;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_VIEW_NUM_LEVELS: types::GLenum = 0x82DC;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: types::GLenum = 0x8CD7;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLEBUFFER: types::GLenum = 0x0C32;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_SIZE: types::GLenum = 0x8623;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE6: types::GLenum = 0x84C6;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_BUFFER_BARRIER_BIT: types::GLenum = 0x00008000;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_SKIP_IMAGES: types::GLenum = 0x806D;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_TEST: types::GLenum = 0x0B71;
#[allow(dead_code, non_upper_case_globals)]
pub const PRIMITIVES_GENERATED: types::GLenum = 0x8C87;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAMEBUFFER_SAMPLES: types::GLenum = 0x9318;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_LOD_BIAS: types::GLenum = 0x8501;
#[allow(dead_code, non_upper_case_globals)]
pub const REFERENCED_BY_FRAGMENT_SHADER: types::GLenum = 0x930A;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92CE;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_BINDING: types::GLenum = 0x8919;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: types::GLenum = 0x8211;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_DEPTH_SIZE: types::GLenum = 0x8275;
#[allow(dead_code, non_upper_case_globals)]
pub const R8: types::GLenum = 0x8229;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER2: types::GLenum = 0x8827;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_STENCIL_ATTACHMENT: types::GLenum = 0x821A;
#[allow(dead_code, non_upper_case_globals)]
pub const PROGRAM_INPUT: types::GLenum = 0x92E3;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_FLAGS: types::GLenum = 0x821E;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COLOR_ATTACHMENTS: types::GLenum = 0x8CDF;
#[allow(dead_code, non_upper_case_globals)]
pub const SRC_COLOR: types::GLenum = 0x0300;
#[allow(dead_code, non_upper_case_globals)]
pub const DECR: types::GLenum = 0x1E03;
#[allow(dead_code, non_upper_case_globals)]
pub const FILL: types::GLenum = 0x1B02;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_INTEGER_SAMPLES: types::GLenum = 0x9110;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_SHORT_5_6_5: types::GLenum = 0x8363;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_VEC4: types::GLenum = 0x8FFE;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_CUBE: types::GLenum = 0x8DD4;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT4x3: types::GLenum = 0x8B6A;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER5: types::GLenum = 0x882A;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_RENDERABLE: types::GLenum = 0x8289;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT: types::GLenum = 0x1405;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_1D: types::GLenum = 0x8068;
#[allow(dead_code, non_upper_case_globals)]
pub const PROGRAM_SEPARABLE: types::GLenum = 0x8258;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_POSITIVE_Y: types::GLenum = 0x8517;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_MAP_POINTER: types::GLenum = 0x88BD;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER3: types::GLenum = 0x8828;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: types::GLenum = 0x906A;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_SOURCE_WINDOW_SYSTEM: types::GLenum = 0x8247;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SIGNED_R11_EAC: types::GLenum = 0x9271;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAGMENT_SUBROUTINE_UNIFORM: types::GLenum = 0x92F2;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_IMAGE_UNITS: types::GLenum = 0x8F38;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_MAT4x2: types::GLenum = 0x8F4D;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_TARGET: types::GLenum = 0x1006;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_VEC4: types::GLenum = 0x8B52;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_SKIP_ROWS: types::GLenum = 0x0D03;
#[allow(dead_code, non_upper_case_globals)]
pub const BGRA_INTEGER: types::GLenum = 0x8D9B;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VARYING_VECTORS: types::GLenum = 0x8DFC;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE31: types::GLenum = 0x84DF;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK: types::GLenum = 0x8E22;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SIGNED_RED_RGTC1: types::GLenum = 0x8DBC;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_VERTEX_ATTRIB: types::GLenum = 0x8626;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_SUBROUTINE_MAX_LENGTH: types::GLenum = 0x8E48;
#[allow(dead_code, non_upper_case_globals)]
pub const SRGB: types::GLenum = 0x8C40;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_RELATIVE_OFFSET: types::GLenum = 0x82D5;
#[allow(dead_code, non_upper_case_globals)]
pub const NEAREST: types::GLenum = 0x2600;
#[allow(dead_code, non_upper_case_globals)]
pub const FUNC_REVERSE_SUBTRACT: types::GLenum = 0x800B;
#[allow(dead_code, non_upper_case_globals)]
pub const DST_COLOR: types::GLenum = 0x0306;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VARYING_FLOATS: types::GLenum = 0x8B4B;
#[allow(dead_code, non_upper_case_globals)]
pub const READ_FRAMEBUFFER: types::GLenum = 0x8CA8;
#[allow(dead_code, non_upper_case_globals)]
pub const BACK_RIGHT: types::GLenum = 0x0403;
#[allow(dead_code, non_upper_case_globals)]
pub const ARRAY_BUFFER_BINDING: types::GLenum = 0x8894;
#[allow(dead_code, non_upper_case_globals)]
pub const RG8UI: types::GLenum = 0x8238;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_BYTE_3_3_2: types::GLenum = 0x8032;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_SHORT_5_6_5_REV: types::GLenum = 0x8364;
#[allow(dead_code, non_upper_case_globals)]
pub const PROGRAM_PIPELINE: types::GLenum = 0x82E4;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_1D_ARRAY: types::GLenum = 0x8DCE;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_RENDERABLE: types::GLenum = 0x8286;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_16_BITS: types::GLenum = 0x82CA;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_SOURCE_LENGTH: types::GLenum = 0x8B88;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_CONTROL_UNIFORM_BLOCKS: types::GLenum = 0x8E89;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SRGB_ALPHA_BPTC_UNORM: types::GLenum = 0x8E8D;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA32I: types::GLenum = 0x8D82;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAGMENT_SUBROUTINE: types::GLenum = 0x92EC;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: types::GLenum = 0x84F0;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: types::GLenum = 0x8214;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_INDEX: types::GLenum = 0x8A3A;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA8: types::GLenum = 0x8058;
#[allow(dead_code, non_upper_case_globals)]
pub const MIN: types::GLenum = 0x8007;
#[allow(dead_code, non_upper_case_globals)]
pub const FRONT_AND_BACK: types::GLenum = 0x0408;
#[allow(dead_code, non_upper_case_globals)]
pub const OFFSET: types::GLenum = 0x92FC;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_128_BITS: types::GLenum = 0x82C4;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: types::GLenum = 0x8A44;
#[allow(dead_code, non_upper_case_globals)]
pub const BACK_LEFT: types::GLenum = 0x0402;
#[allow(dead_code, non_upper_case_globals)]
pub const NEGATIVE_ONE_TO_ONE: types::GLenum = 0x935E;
#[allow(dead_code, non_upper_case_globals)]
pub const COPY: types::GLenum = 0x1503;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_UNPACK_BUFFER: types::GLenum = 0x88EC;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_WRAP_S: types::GLenum = 0x2802;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAGMENT_SHADER: types::GLenum = 0x8B30;
#[allow(dead_code, non_upper_case_globals)]
pub const R8_SNORM: types::GLenum = 0x8F94;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RED: types::GLenum = 0x8225;
#[allow(dead_code, non_upper_case_globals)]
pub const COPY_READ_BUFFER: types::GLenum = 0x8F36;
#[allow(dead_code, non_upper_case_globals)]
pub const KEEP: types::GLenum = 0x1E00;
#[allow(dead_code, non_upper_case_globals)]
pub const COPY_INVERTED: types::GLenum = 0x150C;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_MAT3x2: types::GLenum = 0x8F4B;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER12: types::GLenum = 0x8831;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_BYTE: types::GLenum = 0x1401;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_HEIGHT: types::GLenum = 0x1001;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_UNSUPPORTED: types::GLenum = 0x8CDD;
#[allow(dead_code, non_upper_case_globals)]
pub const PRIMITIVE_RESTART_FIXED_INDEX: types::GLenum = 0x8D69;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_SHORT_4_4_4_4: types::GLenum = 0x8033;
#[allow(dead_code, non_upper_case_globals)]
pub const ALL_BARRIER_BITS: types::GLenum = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_BINDING: types::GLenum = 0x8CA6;
#[allow(dead_code, non_upper_case_globals)]
pub const MIRROR_CLAMP_TO_EDGE: types::GLenum = 0x8743;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_SOURCE_OTHER: types::GLenum = 0x824B;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_USAGE: types::GLenum = 0x8765;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BUFFER_START: types::GLenum = 0x8A29;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_2_10_10_10_REV: types::GLenum = 0x8D9F;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT23: types::GLenum = 0x8CF7;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_WRAP_T: types::GLenum = 0x2803;
#[allow(dead_code, non_upper_case_globals)]
pub const INNOCENT_CONTEXT_RESET: types::GLenum = 0x8254;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER: types::GLenum = 0x0C01;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_INDEX: types::GLenum = 0x9301;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x910C;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_FAIL: types::GLenum = 0x0B94;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_HEIGHT: types::GLenum = 0x8D43;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_INTERNAL_FORMAT: types::GLenum = 0x8D44;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_CLEAR_VALUE: types::GLenum = 0x0B73;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_RECTANGLE: types::GLenum = 0x84F7;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL: types::GLenum = 0x1802;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_RED_SIZE: types::GLenum = 0x8D50;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA16: types::GLenum = 0x805B;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_MAT2x4: types::GLenum = 0x8F4A;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_LOOP: types::GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_STORAGE_BLOCK: types::GLenum = 0x92E6;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_ACCESS: types::GLenum = 0x88BB;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_MAG_FILTER: types::GLenum = 0x2800;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE17: types::GLenum = 0x84D1;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_INPUT_COMPONENTS: types::GLenum = 0x9123;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_COMPONENT: types::GLenum = 0x1902;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA8I: types::GLenum = 0x8D8E;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPRESSED_IMAGE_SIZE: types::GLenum = 0x86A0;
#[allow(dead_code, non_upper_case_globals)]
pub const DONT_CARE: types::GLenum = 0x1100;
#[allow(dead_code, non_upper_case_globals)]
pub const RG32F: types::GLenum = 0x8230;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_2D: types::GLenum = 0x904D;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_BINDING_LAYER: types::GLenum = 0x8F3D;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_WRITEMASK: types::GLenum = 0x0B98;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_STENCIL: types::GLenum = 0x84F9;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_1_X_32: types::GLenum = 0x82BB;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_SRC_RGB: types::GLenum = 0x80C9;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_VIEW_MIN_LAYER: types::GLenum = 0x82DD;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_UNIFORM_BLOCKS: types::GLenum = 0x8A2B;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_5_9_9_9_REV: types::GLenum = 0x8C3E;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT3x4: types::GLenum = 0x8B68;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90D8;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE29: types::GLenum = 0x84DD;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPUTE_WORK_GROUP_SIZE: types::GLenum = 0x8267;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE1: types::GLenum = 0x3001;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_WAIT_INVERTED: types::GLenum = 0x8E17;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_48_BITS: types::GLenum = 0x82C7;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_2_X_16: types::GLenum = 0x82BD;
#[allow(dead_code, non_upper_case_globals)]
pub const INCR: types::GLenum = 0x1E02;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_1D_ARRAY: types::GLenum = 0x9068;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_COVERAGE_INVERT: types::GLenum = 0x80AB;
#[allow(dead_code, non_upper_case_globals)]
pub const CLEAR_TEXTURE: types::GLenum = 0x9365;
#[allow(dead_code, non_upper_case_globals)]
pub const NUM_SHADER_BINARY_FORMATS: types::GLenum = 0x8DF9;
#[allow(dead_code, non_upper_case_globals)]
pub const UNKNOWN_CONTEXT_RESET: types::GLenum = 0x8255;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_PIXEL_FORMAT: types::GLenum = 0x82A9;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB16F: types::GLenum = 0x881B;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_FLAG_ROBUST_ACCESS_BIT: types::GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: types::GLenum = 0x8A42;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_BUFFER: types::GLenum = 0x8C2C;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_INVALIDATE_RANGE_BIT: types::GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_BY_REGION_NO_WAIT: types::GLenum = 0x8E16;
#[allow(dead_code, non_upper_case_globals)]
pub const INVALID_FRAMEBUFFER_OPERATION: types::GLenum = 0x0506;
#[allow(dead_code, non_upper_case_globals)]
pub const MEDIUM_INT: types::GLenum = 0x8DF4;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_DST_RGB: types::GLenum = 0x80C8;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: types::GLenum = 0x8F39;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_ATOMIC_COUNTER: types::GLenum = 0x92DB;
#[allow(dead_code, non_upper_case_globals)]
pub const READ_PIXELS_TYPE: types::GLenum = 0x828E;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE2: types::GLenum = 0x84C2;
#[allow(dead_code, non_upper_case_globals)]
pub const PROGRAM_BINARY_RETRIEVABLE_HINT: types::GLenum = 0x8257;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAMEBUFFER_WIDTH: types::GLenum = 0x9315;
#[allow(dead_code, non_upper_case_globals)]
pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: types::GLenum = 0x82AE;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_1D: types::GLenum = 0x904C;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_EVALUATION_SUBROUTINE: types::GLenum = 0x92EA;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_SUBROUTINE_UNIFORM: types::GLenum = 0x92EE;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_SAMPLES: types::GLenum = 0x8D57;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_IMMUTABLE_LEVELS: types::GLenum = 0x82DF;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_TYPE_PORTABILITY: types::GLenum = 0x824F;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: types::GLenum = 0x8CD4;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_IMAGE_LOAD: types::GLenum = 0x82A4;
#[allow(dead_code, non_upper_case_globals)]
pub const HIGH_INT: types::GLenum = 0x8DF5;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_CULL_DISTANCES: types::GLenum = 0x82F9;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_DST_COLOR: types::GLenum = 0x0307;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BARRIER_BIT: types::GLenum = 0x00001000;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_PROGRAM_TEXTURE_GATHER_OFFSET: types::GLenum = 0x8E5F;
#[allow(dead_code, non_upper_case_globals)]
pub const VERSION: types::GLenum = 0x1F02;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_VIEW_MIN_LEVEL: types::GLenum = 0x82DB;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: types::GLenum = 0x8C80;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_FETCH_BARRIER_BIT: types::GLenum = 0x00000008;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB8_SNORM: types::GLenum = 0x8F96;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_STORAGE_BUFFER_SIZE: types::GLenum = 0x90D5;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_PASS_DEPTH_PASS: types::GLenum = 0x0B96;
#[allow(dead_code, non_upper_case_globals)]
pub const IS_ROW_MAJOR: types::GLenum = 0x9300;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_3D: types::GLenum = 0x8070;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_COMPRESSED_BLOCK_SIZE: types::GLenum = 0x912E;
#[allow(dead_code, non_upper_case_globals)]
pub const UNDEFINED_VERTEX: types::GLenum = 0x8260;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_COMPATIBILITY_CLASS: types::GLenum = 0x82A8;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_LOGIC_OP: types::GLenum = 0x0BF2;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_IMAGE_HEIGHT: types::GLenum = 0x806E;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: types::GLenum = 0x8215;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE15: types::GLenum = 0x84CF;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92D0;
#[allow(dead_code, non_upper_case_globals)]
pub const MIN_SAMPLE_SHADING_VALUE: types::GLenum = 0x8C37;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_CLEAR_VALUE: types::GLenum = 0x0C22;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_CUBE: types::GLenum = 0x905B;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_COMPILER: types::GLenum = 0x8DFA;
#[allow(dead_code, non_upper_case_globals)]
pub const READ_PIXELS: types::GLenum = 0x828C;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT10: types::GLenum = 0x8CEA;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_SKIP_PIXELS: types::GLenum = 0x0D04;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_TEXTURE: types::GLenum = 0x829B;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_1_X_8: types::GLenum = 0x82C1;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x910D;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_UNIFORM_COMPONENTS: types::GLenum = 0x8263;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_3D_TEXTURE_SIZE: types::GLenum = 0x8073;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_OFFSET_POINT: types::GLenum = 0x2A01;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_CONTROL_UNIFORM_COMPONENTS: types::GLenum = 0x8E7F;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT2x4: types::GLenum = 0x8B66;
#[allow(dead_code, non_upper_case_globals)]
pub const CW: types::GLenum = 0x0900;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_SEVERITY_HIGH: types::GLenum = 0x9146;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: types::GLenum = 0x8C88;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ARRAY: types::GLenum = 0x8074;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_RED_TYPE: types::GLenum = 0x8278;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_UNIFORM_BLOCKS: types::GLenum = 0x8E8A;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT15: types::GLenum = 0x8CEF;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: types::GLenum = 0x90EC;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE12: types::GLenum = 0x84CC;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_FLAG_DEBUG_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_IMAGE_ACCESS_BARRIER_BIT: types::GLenum = 0x00000020;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: types::GLenum = 0x8243;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_ATOMIC_COUNTERS: types::GLenum = 0x92D4;
#[allow(dead_code, non_upper_case_globals)]
pub const SCISSOR_BOX: types::GLenum = 0x0C10;
#[allow(dead_code, non_upper_case_globals)]
pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: types::GLenum = 0x82AF;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_SAMPLES: types::GLenum = 0x8CAB;
#[allow(dead_code, non_upper_case_globals)]
pub const FIXED: types::GLenum = 0x140C;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RGB8_ETC2: types::GLenum = 0x9274;
#[allow(dead_code, non_upper_case_globals)]
pub const SRC1_COLOR: types::GLenum = 0x88F9;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT2: types::GLenum = 0x8B5A;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLES: types::GLenum = 0x80A9;
#[allow(dead_code, non_upper_case_globals)]
pub const OUT_OF_MEMORY: types::GLenum = 0x0505;
#[allow(dead_code, non_upper_case_globals)]
pub const R8UI: types::GLenum = 0x8232;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_SIZE: types::GLenum = 0x8764;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: types::GLenum = 0x8B8A;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_OUTPUT_COMPONENTS: types::GLenum = 0x9124;
#[allow(dead_code, non_upper_case_globals)]
pub const MINOR_VERSION: types::GLenum = 0x821C;
#[allow(dead_code, non_upper_case_globals)]
pub const ELEMENT_ARRAY_BUFFER: types::GLenum = 0x8893;
#[allow(dead_code, non_upper_case_globals)]
pub const MIPMAP: types::GLenum = 0x8293;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_1D: types::GLenum = 0x8DC9;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_CONTROL_SUBROUTINE: types::GLenum = 0x92E9;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_ALPHA_SIZE: types::GLenum = 0x8D53;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_TYPE_OTHER: types::GLenum = 0x8251;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_GATHER: types::GLenum = 0x82A2;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_ROW_LENGTH: types::GLenum = 0x0D02;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_2D_MULTISAMPLE: types::GLenum = 0x9109;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: types::GLenum = 0x92C8;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_MIN_LOD: types::GLenum = 0x813A;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_COMPRESSED_BLOCK_DEPTH: types::GLenum = 0x912D;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SRGB: types::GLenum = 0x8C48;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT20: types::GLenum = 0x8CF4;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT24: types::GLenum = 0x8CF8;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: types::GLenum = 0x8E49;
#[allow(dead_code, non_upper_case_globals)]
pub const TRUE: types::GLboolean = 1;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAGMENT_UNIFORM_VECTORS: types::GLenum = 0x8DFD;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_RESULT_AVAILABLE: types::GLenum = 0x8867;
#[allow(dead_code, non_upper_case_globals)]
pub const BACK: types::GLenum = 0x0405;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_DEFAULT_HEIGHT: types::GLenum = 0x9311;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_BINARY_FORMATS: types::GLenum = 0x8DF8;
#[allow(dead_code, non_upper_case_globals)]
pub const QUADS: types::GLenum = 0x0007;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_2D_ARRAY: types::GLenum = 0x8C1D;
#[allow(dead_code, non_upper_case_globals)]
pub const SRC_ALPHA: types::GLenum = 0x0302;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_TYPE_PUSH_GROUP: types::GLenum = 0x8269;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER7: types::GLenum = 0x882C;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_ACTIVE: types::GLenum = 0x8E24;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_SRC_ALPHA: types::GLenum = 0x80CB;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_TYPE_PERFORMANCE: types::GLenum = 0x8250;
#[allow(dead_code, non_upper_case_globals)]
pub const LINEAR_MIPMAP_NEAREST: types::GLenum = 0x2701;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERLEAVED_ATTRIBS: types::GLenum = 0x8C8C;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_RECTANGLE: types::GLenum = 0x84F6;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_INDEX: types::GLenum = 0x1901;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_BARRIER_BIT: types::GLenum = 0x00000400;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_CONTROL_SUBROUTINE_UNIFORM: types::GLenum = 0x92EF;
#[allow(dead_code, non_upper_case_globals)]
pub const STREAM_COPY: types::GLenum = 0x88E2;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BLUE_SIZE: types::GLenum = 0x805E;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_WORK_GROUP_SIZE: types::GLenum = 0x91BF;
#[allow(dead_code, non_upper_case_globals)]
pub const LOW_INT: types::GLenum = 0x8DF3;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_DEFAULT_SAMPLES: types::GLenum = 0x9313;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIENT_STORAGE_BIT: types::GLenum = 0x0200;
#[allow(dead_code, non_upper_case_globals)]
pub const LOSE_CONTEXT_ON_RESET: types::GLenum = 0x8252;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_BINDING_FORMAT: types::GLenum = 0x906E;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_CALLBACK_FUNCTION: types::GLenum = 0x8244;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_MASK: types::GLenum = 0x8E51;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_IMAGE_UNIFORMS: types::GLenum = 0x90CC;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_RESULT_NO_WAIT: types::GLenum = 0x9194;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_SHORT_1_5_5_5_REV: types::GLenum = 0x8366;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_32_BITS: types::GLenum = 0x82C8;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_CONTROL_ATOMIC_COUNTERS: types::GLenum = 0x92D3;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_NO_WAIT_INVERTED: types::GLenum = 0x8E18;
#[allow(dead_code, non_upper_case_globals)]
pub const PATCH_DEFAULT_INNER_LEVEL: types::GLenum = 0x8E73;
#[allow(dead_code, non_upper_case_globals)]
pub const READ_WRITE: types::GLenum = 0x88BA;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: types::GLenum = 0x9276;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_4_X_16: types::GLenum = 0x82BC;
#[allow(dead_code, non_upper_case_globals)]
pub const RG8_SNORM: types::GLenum = 0x8F95;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_BARRIER_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SHADOW: types::GLenum = 0x82A1;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: types::GLenum = 0x8C76;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_4_X_32: types::GLenum = 0x82B9;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BASE_LEVEL: types::GLenum = 0x813C;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_CUBE_MAP_ARRAY: types::GLenum = 0x900A;
#[allow(dead_code, non_upper_case_globals)]
pub const CLEAR_BUFFER: types::GLenum = 0x82B4;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_COHERENT_BIT: types::GLenum = 0x0080;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_UNDEFINED: types::GLenum = 0x8219;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_DEPTH_TEXTURE_SAMPLES: types::GLenum = 0x910F;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_UNIFORMS: types::GLenum = 0x8B86;
#[allow(dead_code, non_upper_case_globals)]
pub const SYNC_STATUS: types::GLenum = 0x9114;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER: types::GLenum = 0x82E6;
#[allow(dead_code, non_upper_case_globals)]
pub const ISOLINES: types::GLenum = 0x8E7A;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_BINDING_BUFFER: types::GLenum = 0x8F4F;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB16_SNORM: types::GLenum = 0x8F9A;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_UNIFORM_VECTORS: types::GLenum = 0x8DFB;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA8_SNORM: types::GLenum = 0x8F97;
#[allow(dead_code, non_upper_case_globals)]
pub const R16I: types::GLenum = 0x8233;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT3: types::GLenum = 0x8B5B;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_COMPRESSED_BLOCK_WIDTH: types::GLenum = 0x9127;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_BINDING_OFFSET: types::GLenum = 0x82D7;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT12: types::GLenum = 0x8CEC;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER: types::GLenum = 0x8C8E;
#[allow(dead_code, non_upper_case_globals)]
pub const SMOOTH_LINE_WIDTH_RANGE: types::GLenum = 0x0B22;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE0: types::GLenum = 0x84C0;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_2D_MULTISAMPLE: types::GLenum = 0x9060;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERER: types::GLenum = 0x1F01;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_INDEX1: types::GLenum = 0x8D46;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE: types::GLenum = 0x1702;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_SEVERITY_NOTIFICATION: types::GLenum = 0x826B;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_S3TC_DXT5_RGBA: types::GLenum = 0x82CF;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_SHADER: types::GLenum = 0x8B31;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE5: types::GLenum = 0x84C5;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_CONTROL_OUTPUT_VERTICES: types::GLenum = 0x8E75;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIENT_MAPPED_BUFFER_BARRIER_BIT: types::GLenum = 0x00004000;
#[allow(dead_code, non_upper_case_globals)]
pub const SRGB8: types::GLenum = 0x8C41;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: types::GLenum = 0x92C5;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_DST_ALPHA: types::GLenum = 0x0305;
#[allow(dead_code, non_upper_case_globals)]
pub const BLUE_INTEGER: types::GLenum = 0x8D96;
#[allow(dead_code, non_upper_case_globals)]
pub const TIMESTAMP: types::GLenum = 0x8E28;
#[allow(dead_code, non_upper_case_globals)]
pub const STATIC_READ: types::GLenum = 0x88E5;
#[allow(dead_code, non_upper_case_globals)]
pub const PATCHES: types::GLenum = 0x000E;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_EVALUATION_TEXTURE: types::GLenum = 0x829D;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_DRAW_BUFFERS: types::GLenum = 0x8824;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: types::GLenum = 0x886A;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_VEC3: types::GLenum = 0x8B54;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_3D: types::GLenum = 0x806F;
#[allow(dead_code, non_upper_case_globals)]
pub const ATTACHED_SHADERS: types::GLenum = 0x8B85;
#[allow(dead_code, non_upper_case_globals)]
pub const DYNAMIC_STORAGE_BIT: types::GLenum = 0x0100;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_ALPHA_SIZE: types::GLenum = 0x8274;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_1D: types::GLenum = 0x8063;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_RED_SIZE: types::GLenum = 0x805C;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: types::GLenum = 0x8C8F;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_2D: types::GLenum = 0x8DD2;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA2: types::GLenum = 0x8055;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_3D: types::GLenum = 0x904E;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_DIMENSIONS: types::GLenum = 0x8282;
#[allow(dead_code, non_upper_case_globals)]
pub const READ_FRAMEBUFFER_BINDING: types::GLenum = 0x8CAA;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_BINDING_DIVISOR: types::GLenum = 0x82D6;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH: types::GLenum = 0x1801;
#[allow(dead_code, non_upper_case_globals)]
pub const REFERENCED_BY_TESS_CONTROL_SHADER: types::GLenum = 0x9307;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_DEPTH_TYPE: types::GLenum = 0x827C;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_PACK_BUFFER: types::GLenum = 0x88EB;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_SUBROUTINE_UNIFORM_LOCATIONS: types::GLenum = 0x8DE8;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_ELEMENT_INDEX: types::GLenum = 0x8D6B;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_VEC4: types::GLenum = 0x8B55;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEWPORT_INDEX_PROVOKING_VERTEX: types::GLenum = 0x825F;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT0: types::GLenum = 0x8CE0;
#[allow(dead_code, non_upper_case_globals)]
pub const SRGB_WRITE: types::GLenum = 0x8298;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: types::GLenum = 0x8518;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SAMPLES: types::GLenum = 0x9106;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT6: types::GLenum = 0x8CE6;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_TYPE: types::GLenum = 0x8B4F;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_RECTANGLE: types::GLenum = 0x84F5;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAGMENT_IMAGE_UNIFORMS: types::GLenum = 0x90CE;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB4: types::GLenum = 0x804F;
#[allow(dead_code, non_upper_case_globals)]
pub const OBJECT_TYPE: types::GLenum = 0x9112;
#[allow(dead_code, non_upper_case_globals)]
pub const DYNAMIC_DRAW: types::GLenum = 0x88E8;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_FADE_THRESHOLD_SIZE: types::GLenum = 0x8128;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPILE_STATUS: types::GLenum = 0x8B81;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_INPUT_TYPE: types::GLenum = 0x8917;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_EVALUATION_SUBROUTINE_UNIFORM: types::GLenum = 0x92F0;
#[allow(dead_code, non_upper_case_globals)]
pub const PROGRAM_BINARY_LENGTH: types::GLenum = 0x8741;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_DST: types::GLenum = 0x0BE0;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VARYING_COMPONENTS: types::GLenum = 0x8B4B;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_IMAGE_UNIFORMS: types::GLenum = 0x90CF;
#[allow(dead_code, non_upper_case_globals)]
pub const DYNAMIC_READ: types::GLenum = 0x88E9;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_TEXEL_SIZE: types::GLenum = 0x82A7;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_SHADER_OUTPUT_RESOURCES: types::GLenum = 0x8F39;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ENCODING: types::GLenum = 0x8296;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPRESSED: types::GLenum = 0x86A1;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT28: types::GLenum = 0x8CFC;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_10F_11F_11F_REV: types::GLenum = 0x8C3B;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_SKIP_ROWS: types::GLenum = 0x0CF3;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT21: types::GLenum = 0x8CF5;
#[allow(dead_code, non_upper_case_globals)]
pub const REFERENCED_BY_VERTEX_SHADER: types::GLenum = 0x9306;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: types::GLenum = 0x8E85;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_DST_ALPHA: types::GLenum = 0x80CA;
#[allow(dead_code, non_upper_case_globals)]
pub const NUM_SAMPLE_COUNTS: types::GLenum = 0x9380;
#[allow(dead_code, non_upper_case_globals)]
pub const RG: types::GLenum = 0x8227;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE24: types::GLenum = 0x84D8;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_SUPPORTED: types::GLenum = 0x826F;
#[allow(dead_code, non_upper_case_globals)]
pub const LINEAR_MIPMAP_LINEAR: types::GLenum = 0x2703;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: types::GLenum = 0x8B49;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_1D: types::GLenum = 0x8B5D;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RG11_EAC: types::GLenum = 0x9272;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_GEN_SPACING: types::GLenum = 0x8E77;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT19: types::GLenum = 0x8CF3;
#[allow(dead_code, non_upper_case_globals)]
pub const MATRIX_STRIDE: types::GLenum = 0x92FF;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_CUBE_MAP_ARRAY: types::GLenum = 0x900C;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_COMPONENTS: types::GLenum = 0x8284;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_MAT4: types::GLenum = 0x8F48;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_VEC2: types::GLenum = 0x8FFC;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: types::GLenum = 0x9277;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_2D_ARRAY: types::GLenum = 0x9053;
#[allow(dead_code, non_upper_case_globals)]
pub const SMOOTH_LINE_WIDTH_GRANULARITY: types::GLenum = 0x0B23;
#[allow(dead_code, non_upper_case_globals)]
pub const BOOL_VEC3: types::GLenum = 0x8B58;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ARRAY_BINDING: types::GLenum = 0x85B5;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_SOURCE_API: types::GLenum = 0x8246;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_2D_MULTISAMPLE: types::GLenum = 0x9100;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_RED_TYPE: types::GLenum = 0x8C10;
#[allow(dead_code, non_upper_case_globals)]
pub const GET_TEXTURE_IMAGE_TYPE: types::GLenum = 0x8292;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPATIBLE_SUBROUTINES: types::GLenum = 0x8E4B;
#[allow(dead_code, non_upper_case_globals)]
pub const INVALID_INDEX: types::GLuint = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_STORAGE_BARRIER_BIT: types::GLenum = 0x00002000;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_SMOOTH: types::GLenum = 0x0B20;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_NAME_LENGTH: types::GLenum = 0x8A41;
#[allow(dead_code, non_upper_case_globals)]
pub const HALF_FLOAT: types::GLenum = 0x140B;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_SUBROUTINES: types::GLenum = 0x8DE7;
#[allow(dead_code, non_upper_case_globals)]
pub const WAIT_FAILED: types::GLenum = 0x911D;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_UPDATE_BARRIER_BIT: types::GLenum = 0x00000200;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_TYPE: types::GLenum = 0x8A37;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAGMENT_ATOMIC_COUNTERS: types::GLenum = 0x92D6;
#[allow(dead_code, non_upper_case_globals)]
pub const LEQUAL: types::GLenum = 0x0203;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT26: types::GLenum = 0x8CFA;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_SWAP_BYTES: types::GLenum = 0x0CF0;
#[allow(dead_code, non_upper_case_globals)]
pub const RG16_SNORM: types::GLenum = 0x8F99;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_S3TC_DXT1_RGB: types::GLenum = 0x82CC;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_DEPTH_SIZE: types::GLenum = 0x884A;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_COMPRESSED_BLOCK_HEIGHT: types::GLenum = 0x9128;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x8264;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_GEN_MODE: types::GLenum = 0x8E76;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER: types::GLenum = 0x8D40;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_CUBE_MAP_ARRAY_SHADOW: types::GLenum = 0x900D;
#[allow(dead_code, non_upper_case_globals)]
pub const OR_INVERTED: types::GLenum = 0x150D;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SIGNED_RG11_EAC: types::GLenum = 0x9273;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_DEFAULT_LAYERS: types::GLenum = 0x9312;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_CONTROL_IMAGE_UNIFORMS: types::GLenum = 0x90CB;
#[allow(dead_code, non_upper_case_globals)]
pub const RED: types::GLenum = 0x1903;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_ATTACHMENT: types::GLenum = 0x8D20;
#[allow(dead_code, non_upper_case_globals)]
pub const AND: types::GLenum = 0x1501;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_TEST: types::GLenum = 0x0B90;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_CONTROL_TEXTURE: types::GLenum = 0x829C;
#[allow(dead_code, non_upper_case_globals)]
pub const CONSTANT_COLOR: types::GLenum = 0x8001;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_MODE: types::GLenum = 0x0B40;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE30: types::GLenum = 0x84DE;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_CLAMP: types::GLenum = 0x864F;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT16: types::GLenum = 0x8CF0;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_MAT3: types::GLenum = 0x8F47;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_UNIFORM_BLOCKS: types::GLenum = 0x8A2E;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_OFFSET_FILL: types::GLenum = 0x8037;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: types::GLenum = 0x8A34;
#[allow(dead_code, non_upper_case_globals)]
pub const BOOL_VEC2: types::GLenum = 0x8B57;
#[allow(dead_code, non_upper_case_globals)]
pub const READ_PIXELS_FORMAT: types::GLenum = 0x828D;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_GROUP_STACK_DEPTH: types::GLenum = 0x826D;
#[allow(dead_code, non_upper_case_globals)]
pub const TIMEOUT_EXPIRED: types::GLenum = 0x911B;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP: types::GLenum = 0x8513;
#[allow(dead_code, non_upper_case_globals)]
pub const NUM_COMPATIBLE_SUBROUTINES: types::GLenum = 0x8E4A;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_VEC3: types::GLenum = 0x8FFD;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BACK_PASS_DEPTH_PASS: types::GLenum = 0x8803;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA32F: types::GLenum = 0x8814;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_ATOMIC_COUNTER_BUFFER_SIZE: types::GLenum = 0x92D8;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT: types::GLenum = 0x1406;
#[allow(dead_code, non_upper_case_globals)]
pub const MIN_FRAGMENT_INTERPOLATION_OFFSET: types::GLenum = 0x8E5B;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_BUFFER: types::GLenum = 0x8DD8;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_VARIABLES: types::GLenum = 0x9305;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_DEPTH_SIZE: types::GLenum = 0x8D54;
#[allow(dead_code, non_upper_case_globals)]
pub const COPY_WRITE_BUFFER_BINDING: types::GLenum = 0x8F37;
#[allow(dead_code, non_upper_case_globals)]
pub const TRIANGLE_STRIP_ADJACENCY: types::GLenum = 0x000D;
#[allow(dead_code, non_upper_case_globals)]
pub const NUM_PROGRAM_BINARY_FORMATS: types::GLenum = 0x87FE;
#[allow(dead_code, non_upper_case_globals)]
pub const REPEAT: types::GLenum = 0x2901;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_CLEAR_VALUE: types::GLenum = 0x0B91;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_SHADER_BIT: types::GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_1_X_16: types::GLenum = 0x82BE;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_2D_ARRAY: types::GLenum = 0x905E;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_SAMPLE_MASK_WORDS: types::GLenum = 0x8E59;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BORDER_COLOR: types::GLenum = 0x1004;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_WIDTH: types::GLenum = 0x827E;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB32UI: types::GLenum = 0x8D71;
#[allow(dead_code, non_upper_case_globals)]
pub const CULL_FACE_MODE: types::GLenum = 0x0B45;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: types::GLenum = 0x92C6;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_BINDING: types::GLenum = 0x82D4;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_2D_RECT: types::GLenum = 0x904F;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RGB: types::GLenum = 0x84ED;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BACK_FAIL: types::GLenum = 0x8801;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_ARRAY_STRIDE: types::GLenum = 0x8A3C;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_ATOMIC_COUNTERS: types::GLenum = 0x92D2;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RGBA8_ETC2_EAC: types::GLenum = 0x9278;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D_RECT_SHADOW: types::GLenum = 0x8B64;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_3D: types::GLenum = 0x806A;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_SWAP_BYTES: types::GLenum = 0x0D00;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: types::GLenum = 0x8C8B;
#[allow(dead_code, non_upper_case_globals)]
pub const BGR_INTEGER: types::GLenum = 0x8D9A;
#[allow(dead_code, non_upper_case_globals)]
pub const TIME_ELAPSED: types::GLenum = 0x88BF;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BACK_VALUE_MASK: types::GLenum = 0x8CA4;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT9: types::GLenum = 0x8CE9;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_SIZE: types::GLenum = 0x92C3;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D_MULTISAMPLE: types::GLenum = 0x9108;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_CUBE_MAP_TEXTURE_SIZE: types::GLenum = 0x851C;
#[allow(dead_code, non_upper_case_globals)]
pub const TIMEOUT_IGNORED: types::GLuint64 = 0xFFFFFFFFFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CUBE_MAP_ARRAY: types::GLenum = 0x9054;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_MAX_LEVEL: types::GLenum = 0x813D;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90D9;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_ATTRIB_STRIDE: types::GLenum = 0x82E5;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92CD;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D: types::GLenum = 0x8B5E;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_MAT3x4: types::GLenum = 0x8F4C;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_BLUE_TYPE: types::GLenum = 0x827A;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8872;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_MAT2x3: types::GLenum = 0x8F49;
#[allow(dead_code, non_upper_case_globals)]
pub const IMPLEMENTATION_COLOR_READ_TYPE: types::GLenum = 0x8B9A;
#[allow(dead_code, non_upper_case_globals)]
pub const INVERT: types::GLenum = 0x150A;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_DATA_SIZE: types::GLenum = 0x92C4;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: types::GLenum = 0x8216;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_DEPTH: types::GLenum = 0x8071;
#[allow(dead_code, non_upper_case_globals)]
pub const READ_ONLY: types::GLenum = 0x88B8;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB16: types::GLenum = 0x8054;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_BUFFERS: types::GLenum = 0x80A8;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_1D: types::GLenum = 0x9057;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: types::GLenum = 0x90ED;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TEXTURE_LOD_BIAS: types::GLenum = 0x84FD;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER8: types::GLenum = 0x882D;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_2D_MULTISAMPLE: types::GLenum = 0x9104;
#[allow(dead_code, non_upper_case_globals)]
pub const NUM_EXTENSIONS: types::GLenum = 0x821D;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_SHARED_SIZE: types::GLenum = 0x8277;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: types::GLenum = 0x8212;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_UNIFORM_BLOCKS: types::GLenum = 0x8A36;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_ALPHA_SIZE: types::GLenum = 0x805F;
#[allow(dead_code, non_upper_case_globals)]
pub const FRONT_RIGHT: types::GLenum = 0x0401;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_SHORT: types::GLenum = 0x1403;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D_SHADOW: types::GLenum = 0x8B62;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_SHADER: types::GLenum = 0x8DD9;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_DEPTH_TYPE: types::GLenum = 0x8C16;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_OUTPUT_SYNCHRONOUS: types::GLenum = 0x8242;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAGMENT_INTERPOLATION_OFFSET_BITS: types::GLenum = 0x8E5D;
#[allow(dead_code, non_upper_case_globals)]
pub const R16UI: types::GLenum = 0x8234;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_1D_ARRAY: types::GLenum = 0x8C19;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BUFFER: types::GLenum = 0x8C2A;
#[allow(dead_code, non_upper_case_globals)]
pub const READ_BUFFER: types::GLenum = 0x0C02;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_1D: types::GLenum = 0x8DD1;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK: types::GLenum = 0x92E2;
#[allow(dead_code, non_upper_case_globals)]
pub const RG16F: types::GLenum = 0x822F;
#[allow(dead_code, non_upper_case_globals)]
pub const SRGB8_ALPHA8: types::GLenum = 0x8C43;
#[allow(dead_code, non_upper_case_globals)]
pub const STACK_UNDERFLOW: types::GLenum = 0x0504;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE28: types::GLenum = 0x84DC;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_CONTROL_INPUT_COMPONENTS: types::GLenum = 0x886C;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_UNIFORM_LOCATIONS: types::GLenum = 0x826E;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_RELEASE_BEHAVIOR_FLUSH: types::GLenum = 0x82FC;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RG_RGTC2: types::GLenum = 0x8DBD;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_IMAGE_STORE: types::GLenum = 0x82A5;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_BLEND: types::GLenum = 0x828B;
#[allow(dead_code, non_upper_case_globals)]
pub const DECR_WRAP: types::GLenum = 0x8508;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_FRAMEBUFFER: types::GLenum = 0x8CA9;
#[allow(dead_code, non_upper_case_globals)]
pub const VALIDATE_STATUS: types::GLenum = 0x8B83;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_TYPE_ERROR: types::GLenum = 0x824C;
#[allow(dead_code, non_upper_case_globals)]
pub const OR: types::GLenum = 0x1507;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_CONSTANT_COLOR: types::GLenum = 0x8002;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_DIVISOR: types::GLenum = 0x88FE;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_ATTRIB_BINDINGS: types::GLenum = 0x82DA;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_NEGATIVE_X: types::GLenum = 0x8516;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90D6;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_ALPHA_TO_ONE: types::GLenum = 0x809F;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_2D_ARRAY: types::GLenum = 0x8DCF;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT2x3: types::GLenum = 0x8B65;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_VARYING: types::GLenum = 0x92F4;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_UNIFORM_COMPONENTS: types::GLenum = 0x8B4A;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_STENCIL_TYPE: types::GLenum = 0x827D;
#[allow(dead_code, non_upper_case_globals)]
pub const VENDOR: types::GLenum = 0x1F00;
#[allow(dead_code, non_upper_case_globals)]
pub const RIGHT: types::GLenum = 0x0407;
#[allow(dead_code, non_upper_case_globals)]
pub const BLUE: types::GLenum = 0x1905;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_CONTROL_SHADER: types::GLenum = 0x8E88;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SRGB8_ETC2: types::GLenum = 0x9275;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_SKIP_IMAGES: types::GLenum = 0x806B;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BUFFER_OFFSET_ALIGNMENT: types::GLenum = 0x919F;
#[allow(dead_code, non_upper_case_globals)]
pub const SET: types::GLenum = 0x150F;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_GREEN_SIZE: types::GLenum = 0x805D;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_CUBE_MAP: types::GLenum = 0x851B;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_COMPRESSED_BLOCK_DEPTH: types::GLenum = 0x9129;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_COMPLETE: types::GLenum = 0x8CD5;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT30: types::GLenum = 0x8CFE;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_DUAL_SOURCE_DRAW_BUFFERS: types::GLenum = 0x88FC;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: types::GLenum = 0x8CD0;
#[allow(dead_code, non_upper_case_globals)]
pub const FUNC_ADD: types::GLenum = 0x8006;
#[allow(dead_code, non_upper_case_globals)]
pub const NEAREST_MIPMAP_NEAREST: types::GLenum = 0x2700;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_IMAGE_UNIFORMS: types::GLenum = 0x90CD;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA16_SNORM: types::GLenum = 0x8F9B;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_2D_ARRAY: types::GLenum = 0x9069;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_OUTPUT_TYPE: types::GLenum = 0x8918;
#[allow(dead_code, non_upper_case_globals)]
pub const BOOL_VEC4: types::GLenum = 0x8B59;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT8: types::GLenum = 0x8CE8;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT29: types::GLenum = 0x8CFD;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_SIZE_GRANULARITY: types::GLenum = 0x0B13;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPUTE_SHADER_BIT: types::GLenum = 0x00000020;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_RESULT: types::GLenum = 0x8866;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT7: types::GLenum = 0x8CE7;
#[allow(dead_code, non_upper_case_globals)]
pub const IMPLEMENTATION_COLOR_READ_FORMAT: types::GLenum = 0x8B9B;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_LOGGED_MESSAGES: types::GLenum = 0x9145;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_MAT2: types::GLenum = 0x8F46;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA16I: types::GLenum = 0x8D88;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE20: types::GLenum = 0x84D4;
#[allow(dead_code, non_upper_case_globals)]
pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: types::GLenum = 0x82AC;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_PREFERRED: types::GLenum = 0x8270;
#[allow(dead_code, non_upper_case_globals)]
pub const FUNC_SUBTRACT: types::GLenum = 0x800A;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_10_10_10_2: types::GLenum = 0x82C3;
#[allow(dead_code, non_upper_case_globals)]
pub const DYNAMIC_COPY: types::GLenum = 0x88EA;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_RENDERABLE: types::GLenum = 0x8288;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BUFFER_BINDING: types::GLenum = 0x8C2A;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_FORMAT_COMPATIBILITY_TYPE: types::GLenum = 0x90C7;
#[allow(dead_code, non_upper_case_globals)]
pub const FILTER: types::GLenum = 0x829A;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_WRITEMASK: types::GLenum = 0x0C23;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR: types::GLenum = 0x1800;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SWIZZLE_G: types::GLenum = 0x8E43;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_1D_ARRAY_SHADOW: types::GLenum = 0x8DC3;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9056;
#[allow(dead_code, non_upper_case_globals)]
pub const XOR: types::GLenum = 0x1506;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_RGTC1_RED: types::GLenum = 0x82D0;
#[allow(dead_code, non_upper_case_globals)]
pub const NUM_SHADING_LANGUAGE_VERSIONS: types::GLenum = 0x82E9;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT22: types::GLenum = 0x8CF6;
#[allow(dead_code, non_upper_case_globals)]
pub const NUM_COMPRESSED_TEXTURE_FORMATS: types::GLenum = 0x86A2;
#[allow(dead_code, non_upper_case_globals)]
pub const PRIMITIVE_RESTART_INDEX: types::GLenum = 0x8F9E;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB32F: types::GLenum = 0x8815;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_INVALIDATE_BUFFER_BIT: types::GLenum = 0x0008;
#[allow(dead_code, non_upper_case_globals)]
pub const SCISSOR_TEST: types::GLenum = 0x0C11;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_IMAGE_HEIGHT: types::GLenum = 0x806C;
#[allow(dead_code, non_upper_case_globals)]
pub const ZERO: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_EQUATION_ALPHA: types::GLenum = 0x883D;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: types::GLenum = 0x8CDB;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_SIZE: types::GLenum = 0x8A38;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DEPTH_MODE: types::GLenum = 0x935D;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_VEC2: types::GLenum = 0x8B50;
#[allow(dead_code, non_upper_case_globals)]
pub const SRGB_ALPHA: types::GLenum = 0x8C42;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_SHORT_4_4_4_4_REV: types::GLenum = 0x8365;
#[allow(dead_code, non_upper_case_globals)]
pub const WRITE_ONLY: types::GLenum = 0x88B9;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_SEAMLESS: types::GLenum = 0x884F;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_ROW_LENGTH: types::GLenum = 0x0CF2;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TEXTURE_SIZE: types::GLenum = 0x0D33;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_POSITION: types::GLenum = 0x8E50;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_POINTER: types::GLenum = 0x8645;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: types::GLenum = 0x851A;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB16I: types::GLenum = 0x8D89;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_ARRAY: types::GLenum = 0x9009;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA: types::GLenum = 0x1908;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE: types::GLenum = 0x140A;
#[allow(dead_code, non_upper_case_globals)]
pub const REPLACE: types::GLenum = 0x1E01;
#[allow(dead_code, non_upper_case_globals)]
pub const LOGIC_OP_MODE: types::GLenum = 0x0BF0;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_POSITIVE_X: types::GLenum = 0x8515;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_OFFSET: types::GLenum = 0x8A3B;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_ATTRIBUTES: types::GLenum = 0x8B89;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPARE_REF_TO_TEXTURE: types::GLenum = 0x884E;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_IMMUTABLE_STORAGE: types::GLenum = 0x821F;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER: types::GLenum = 0x82E1;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE9: types::GLenum = 0x84C9;
#[allow(dead_code, non_upper_case_globals)]
pub const ANY_SAMPLES_PASSED_CONSERVATIVE: types::GLenum = 0x8D6A;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_ACCESS_FLAGS: types::GLenum = 0x911F;
#[allow(dead_code, non_upper_case_globals)]
pub const LOWER_LEFT: types::GLenum = 0x8CA1;
#[allow(dead_code, non_upper_case_globals)]
pub const PROVOKING_VERTEX: types::GLenum = 0x8E4F;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: types::GLenum = 0x8CD6;
#[allow(dead_code, non_upper_case_globals)]
pub const MIN_MAP_BUFFER_ALIGNMENT: types::GLenum = 0x90BC;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_BINDING_STRIDE: types::GLenum = 0x82D8;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BUFFER_SIZE: types::GLenum = 0x8A2A;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_RENDERABLE: types::GLenum = 0x8287;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_IMAGE_SAMPLES: types::GLenum = 0x906D;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_DATA_SIZE: types::GLenum = 0x9303;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: types::GLenum = 0x8E8F;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_VERTICES_OUT: types::GLenum = 0x8916;
#[allow(dead_code, non_upper_case_globals)]
pub const SYNC_GPU_COMMANDS_COMPLETE: types::GLenum = 0x9117;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE21: types::GLenum = 0x84D5;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: types::GLenum = 0x90DF;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH24_STENCIL8: types::GLenum = 0x88F0;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT11: types::GLenum = 0x8CEB;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_10_10_10_2: types::GLenum = 0x8036;
#[allow(dead_code, non_upper_case_globals)]
pub const BOOL: types::GLenum = 0x8B56;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAGMENT_SHADER_DERIVATIVE_HINT: types::GLenum = 0x8B8B;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH32F_STENCIL8: types::GLenum = 0x8CAD;
#[allow(dead_code, non_upper_case_globals)]
pub const NO_RESET_NOTIFICATION: types::GLenum = 0x8261;
#[allow(dead_code, non_upper_case_globals)]
pub const R16F: types::GLenum = 0x822D;
#[allow(dead_code, non_upper_case_globals)]
pub const TRIANGLE_FAN: types::GLenum = 0x0006;
#[allow(dead_code, non_upper_case_globals)]
pub const NONE: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)]
pub const ALREADY_SIGNALED: types::GLenum = 0x911A;
#[allow(dead_code, non_upper_case_globals)]
pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: types::GLenum = 0x82AD;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_OFFSET_LINE: types::GLenum = 0x2A02;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: types::GLenum = 0x8213;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8B4C;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_2D_ARRAY: types::GLenum = 0x8C1A;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_SRC1_COLOR: types::GLenum = 0x88FA;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE: types::GLenum = 1;
#[allow(dead_code, non_upper_case_globals)]
pub const STACK_OVERFLOW: types::GLenum = 0x0503;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_BINDING_LEVEL: types::GLenum = 0x8F3B;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER4: types::GLenum = 0x8829;
#[allow(dead_code, non_upper_case_globals)]
pub const R3_G3_B2: types::GLenum = 0x2A10;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: types::GLenum = 0x8217;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: types::GLenum = 0x82D9;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_TYPE_MARKER: types::GLenum = 0x8268;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_CORE_PROFILE_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92CC;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_COMPATIBILITY_CLASS: types::GLenum = 0x82B6;
#[allow(dead_code, non_upper_case_globals)]
pub const SYNC_FENCE: types::GLenum = 0x9116;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_INDEX4: types::GLenum = 0x8D47;
#[allow(dead_code, non_upper_case_globals)]
pub const TRIANGLES: types::GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_CLIP_DISTANCES: types::GLenum = 0x0D32;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEWPORT_BOUNDS_RANGE: types::GLenum = 0x825D;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE14: types::GLenum = 0x84CE;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_DEFAULT: types::GLenum = 0x8218;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90DC;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: types::GLenum = 0x8E1E;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_INDEX16: types::GLenum = 0x8D49;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_1D_ARRAY: types::GLenum = 0x8DC0;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_SUBROUTINES: types::GLenum = 0x8DE5;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COLOR_TEXTURE_SAMPLES: types::GLenum = 0x910E;
#[allow(dead_code, non_upper_case_globals)]
pub const SRC1_ALPHA: types::GLenum = 0x8589;
#[allow(dead_code, non_upper_case_globals)]
pub const FIRST_VERTEX_CONVENTION: types::GLenum = 0x8E4D;
#[allow(dead_code, non_upper_case_globals)]
pub const POINTS: types::GLenum = 0x0000;
#[allow(dead_code, non_upper_case_globals)]
pub const SRGB_READ: types::GLenum = 0x8297;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_UNPACK_BUFFER_BINDING: types::GLenum = 0x88EF;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_COVERAGE_VALUE: types::GLenum = 0x80AA;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_RED_SIZE: types::GLenum = 0x8271;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE19: types::GLenum = 0x84D3;
#[allow(dead_code, non_upper_case_globals)]
pub const PROGRAM_PIPELINE_BINDING: types::GLenum = 0x825A;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_32_UNSIGNED_INT_24_8_REV: types::GLenum = 0x8DAD;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_BINDING: types::GLenum = 0x92C1;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT31: types::GLenum = 0x8CFF;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: types::GLenum = 0x8A45;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_UNIFORM_COMPONENTS: types::GLenum = 0x8DDF;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_NUM_COMPATIBLE_SUBROUTINES: types::GLenum = 0x92F8;
#[allow(dead_code, non_upper_case_globals)]
pub const STATIC_DRAW: types::GLenum = 0x88E4;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_ALPHA_TYPE: types::GLenum = 0x827B;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_VALUE_MASK: types::GLenum = 0x0B93;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9061;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_24_BITS: types::GLenum = 0x82C9;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_SRC_COLOR: types::GLenum = 0x0301;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: types::GLenum = 0x8A43;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_SIZE: types::GLenum = 0x0B11;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_CONSTANT_ALPHA: types::GLenum = 0x8004;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_LSB_FIRST: types::GLenum = 0x0D01;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BACK_FUNC: types::GLenum = 0x8800;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER0: types::GLenum = 0x8825;
#[allow(dead_code, non_upper_case_globals)]
pub const TOP_LEVEL_ARRAY_SIZE: types::GLenum = 0x930C;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_SMOOTH_HINT: types::GLenum = 0x0C52;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_BY_REGION_WAIT_INVERTED: types::GLenum = 0x8E19;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_STENCIL_SIZE: types::GLenum = 0x8276;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT4: types::GLenum = 0x8CE4;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER: types::GLenum = 0x82E0;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_BINDING: types::GLenum = 0x8CA7;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE3: types::GLenum = 0x84C3;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_TYPE: types::GLenum = 0x8625;
#[allow(dead_code, non_upper_case_globals)]
pub const STEREO: types::GLenum = 0x0C33;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: types::GLenum = 0x92CA;
#[allow(dead_code, non_upper_case_globals)]
pub const NOOP: types::GLenum = 0x1505;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_2_10_10_10_REV: types::GLenum = 0x8368;
#[allow(dead_code, non_upper_case_globals)]
pub const ELEMENT_ARRAY_BARRIER_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)]
pub const LAST_VERTEX_CONVENTION: types::GLenum = 0x8E4E;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_COVERAGE: types::GLenum = 0x80A0;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_FRAMEBUFFER_BINDING: types::GLenum = 0x8CA6;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90D7;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_IMAGE_ATOMIC: types::GLenum = 0x82A6;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_ALIGNMENT: types::GLenum = 0x0D05;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_BUFFER_BIT: types::GLenum = 0x00000100;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_LOST: types::GLenum = 0x0507;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_INTEGER: types::GLenum = 0x88FD;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_BLUE_SIZE: types::GLenum = 0x8D52;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_ELEMENTS_VERTICES: types::GLenum = 0x80E8;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_IS_ROW_MAJOR: types::GLenum = 0x8A3E;
#[allow(dead_code, non_upper_case_globals)]
pub const DISPLAY_LIST: types::GLenum = 0x82E7;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE18: types::GLenum = 0x84D2;
#[allow(dead_code, non_upper_case_globals)]
pub const NOR: types::GLenum = 0x1508;
#[allow(dead_code, non_upper_case_globals)]
pub const INFO_LOG_LENGTH: types::GLenum = 0x8B84;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_PROGRAM: types::GLenum = 0x8B8D;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_DEBUG_GROUP_STACK_DEPTH: types::GLenum = 0x826C;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_SHADER_STORAGE_BUFFER_BINDINGS: types::GLenum = 0x90DD;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_WIDTH: types::GLenum = 0x0B21;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE8: types::GLenum = 0x84C8;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_VEC3: types::GLenum = 0x8DC7;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SHARED_SIZE: types::GLenum = 0x8C3F;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_ALPHA_TYPE: types::GLenum = 0x8C13;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_PROFILE_MASK: types::GLenum = 0x9126;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TRANSFORM_FEEDBACK_BUFFERS: types::GLenum = 0x8E70;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: types::GLenum = 0x8E86;
#[allow(dead_code, non_upper_case_globals)]
pub const MIN_PROGRAM_TEXEL_OFFSET: types::GLenum = 0x8904;
#[allow(dead_code, non_upper_case_globals)]
pub const MEDIUM_FLOAT: types::GLenum = 0x8DF1;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA16UI: types::GLenum = 0x8D76;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAMEBUFFER_LAYERS: types::GLenum = 0x9317;
#[allow(dead_code, non_upper_case_globals)]
pub const FULL_SUPPORT: types::GLenum = 0x82B7;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE5: types::GLenum = 0x3005;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPUTE_SUBROUTINE: types::GLenum = 0x92ED;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: types::GLenum = 0x9279;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT17: types::GLenum = 0x8CF1;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x910B;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPRESSED_BLOCK_SIZE: types::GLenum = 0x82B3;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_STENCIL_SIZE: types::GLenum = 0x88F1;
#[allow(dead_code, non_upper_case_globals)]
pub const INVALID_VALUE: types::GLenum = 0x0501;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAGMENT_INTERPOLATION_OFFSET: types::GLenum = 0x8E5C;
#[allow(dead_code, non_upper_case_globals)]
pub const MIRRORED_REPEAT: types::GLenum = 0x8370;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_VEC3: types::GLenum = 0x8B51;
#[allow(dead_code, non_upper_case_globals)]
pub const ALPHA: types::GLenum = 0x1906;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_TEXTURE: types::GLenum = 0x829E;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_SIZE_RANGE: types::GLenum = 0x0B12;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: types::GLenum = 0x8E47;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_STRIDE: types::GLenum = 0x934C;
#[allow(dead_code, non_upper_case_globals)]
pub const RG16I: types::GLenum = 0x8239;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_CUBE_MAP_ARRAY: types::GLenum = 0x900B;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_BUFFER_BINDING: types::GLenum = 0x9193;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER1: types::GLenum = 0x8826;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER11: types::GLenum = 0x8830;
#[allow(dead_code, non_upper_case_globals)]
pub const NUM_ACTIVE_VARIABLES: types::GLenum = 0x9304;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB5_A1: types::GLenum = 0x8057;
#[allow(dead_code, non_upper_case_globals)]
pub const ALWAYS: types::GLenum = 0x0207;
#[allow(dead_code, non_upper_case_globals)]
pub const INT: types::GLenum = 0x1404;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_PROGRAM_POINT_SIZE: types::GLenum = 0x8642;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_ATTACHMENT: types::GLenum = 0x8D00;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_GREEN_TYPE: types::GLenum = 0x8279;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_LAYERED: types::GLenum = 0x8DA7;
#[allow(dead_code, non_upper_case_globals)]
pub const PATCH_DEFAULT_OUTER_LEVEL: types::GLenum = 0x8E74;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_CUBE_MAP_ARRAY: types::GLenum = 0x900E;
#[allow(dead_code, non_upper_case_globals)]
pub const AUTO_GENERATE_MIPMAP: types::GLenum = 0x8295;
#[allow(dead_code, non_upper_case_globals)]
pub const SIGNALED: types::GLenum = 0x9119;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADING_LANGUAGE_VERSION: types::GLenum = 0x8B8C;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_START: types::GLenum = 0x8C84;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_SHADER_INVOCATIONS: types::GLenum = 0x887F;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_ARRAY_TEXTURE_LAYERS: types::GLenum = 0x88FF;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_SRC_ALPHA: types::GLenum = 0x0303;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_3D: types::GLenum = 0x8B5F;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA32UI: types::GLenum = 0x8D70;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_ORIGIN: types::GLenum = 0x935C;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_START: types::GLenum = 0x92C2;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RG: types::GLenum = 0x8226;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_INDIRECT_BUFFER_BINDING: types::GLenum = 0x8F43;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_CALLBACK_USER_PARAM: types::GLenum = 0x8245;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92CF;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_COMPONENT24: types::GLenum = 0x81A6;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BUFFER_SIZE: types::GLenum = 0x919E;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT13: types::GLenum = 0x8CED;
#[allow(dead_code, non_upper_case_globals)]
pub const LESS: types::GLenum = 0x0201;
#[allow(dead_code, non_upper_case_globals)]
pub const RG_INTEGER: types::GLenum = 0x8228;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_WORK_GROUP_COUNT: types::GLenum = 0x91BE;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA12: types::GLenum = 0x805A;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_SRC1_ALPHA: types::GLenum = 0x88FB;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_RGTC2_RG: types::GLenum = 0x82D1;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_STENCIL_SIZE: types::GLenum = 0x8D55;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_BUFFER: types::GLenum = 0x8DC2;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_PAUSED: types::GLenum = 0x8E23;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_2D_RECT: types::GLenum = 0x905A;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: types::GLenum = 0x90C9;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_2D: types::GLenum = 0x0DE1;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE13: types::GLenum = 0x84CD;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_2D_RECT: types::GLenum = 0x8DD5;
#[allow(dead_code, non_upper_case_globals)]
pub const FIXED_ONLY: types::GLenum = 0x891D;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE16: types::GLenum = 0x84D0;
#[allow(dead_code, non_upper_case_globals)]
pub const GREATER: types::GLenum = 0x0204;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB8UI: types::GLenum = 0x8D7D;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_ATOMIC_COUNTERS: types::GLenum = 0x8265;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8B4D;
#[allow(dead_code, non_upper_case_globals)]
pub const CLEAR: types::GLenum = 0x1500;
#[allow(dead_code, non_upper_case_globals)]
pub const R11F_G11F_B10F: types::GLenum = 0x8C3A;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_COMPONENT32: types::GLenum = 0x81A7;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: types::GLenum = 0x8C8A;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_FLUSH_EXPLICIT_BIT: types::GLenum = 0x0010;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE23: types::GLenum = 0x84D7;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_GEN_LEVEL: types::GLenum = 0x8E7E;
#[allow(dead_code, non_upper_case_globals)]
pub const R16_SNORM: types::GLenum = 0x8F98;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_STRIDE: types::GLenum = 0x8624;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: types::GLenum = 0x8A31;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_SOURCE_THIRD_PARTY: types::GLenum = 0x8249;
#[allow(dead_code, non_upper_case_globals)]
pub const FRACTIONAL_EVEN: types::GLenum = 0x8E7C;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_WIDTH: types::GLenum = 0x8D42;
#[allow(dead_code, non_upper_case_globals)]
pub const SEPARATE_ATTRIBS: types::GLenum = 0x8C8D;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_1D_ARRAY: types::GLenum = 0x905D;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_CLIP_AND_CULL_DISTANCES: types::GLenum = 0x82FA;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_2D_MULTISAMPLE: types::GLenum = 0x9055;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_ACTIVE: types::GLenum = 0x8E24;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_TEXTURE_FORMATS: types::GLenum = 0x86A3;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_CUBE_SHADOW: types::GLenum = 0x8DC5;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RED_RGTC1: types::GLenum = 0x8DBB;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_UNIFORM_MAX_LENGTH: types::GLenum = 0x8B87;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BUFFER_DATA_STORE_BINDING: types::GLenum = 0x8C2D;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_3D: types::GLenum = 0x8DCB;
#[allow(dead_code, non_upper_case_globals)]
pub const ALIASED_LINE_WIDTH_RANGE: types::GLenum = 0x846E;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_PIXEL_TYPE: types::GLenum = 0x82AA;
#[allow(dead_code, non_upper_case_globals)]
pub const NAND: types::GLenum = 0x150E;
#[allow(dead_code, non_upper_case_globals)]
pub const AND_REVERSE: types::GLenum = 0x1502;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPRESSION_HINT: types::GLenum = 0x84EF;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT25: types::GLenum = 0x8CF9;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPARE_FUNC: types::GLenum = 0x884D;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SWIZZLE_RGBA: types::GLenum = 0x8E46;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT1: types::GLenum = 0x8CE1;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_PROGRAM_TEXEL_OFFSET: types::GLenum = 0x8905;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE4: types::GLenum = 0x3004;
#[allow(dead_code, non_upper_case_globals)]
pub const REFERENCED_BY_GEOMETRY_SHADER: types::GLenum = 0x9309;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_ENABLED: types::GLenum = 0x8622;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_FUNC: types::GLenum = 0x0B74;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: types::GLenum = 0x8DE1;
#[allow(dead_code, non_upper_case_globals)]
pub const NEVER: types::GLenum = 0x0200;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_MATRIX_STRIDE: types::GLenum = 0x8A3D;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_CUBE: types::GLenum = 0x9066;
#[allow(dead_code, non_upper_case_globals)]
pub const RG32I: types::GLenum = 0x823B;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_SHADER_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_2_X_32: types::GLenum = 0x82BA;
#[allow(dead_code, non_upper_case_globals)]
pub const RG16UI: types::GLenum = 0x823A;
#[allow(dead_code, non_upper_case_globals)]
pub const LINK_STATUS: types::GLenum = 0x8B82;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: types::GLenum = 0x8DA8;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_2D: types::GLenum = 0x8DCA;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_INPUT_COMPONENTS: types::GLenum = 0x886D;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_GREEN_TYPE: types::GLenum = 0x8C11;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_VIEW_NUM_LAYERS: types::GLenum = 0x82DE;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_WRITE_BIT: types::GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE7: types::GLenum = 0x84C7;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BLUE_TYPE: types::GLenum = 0x8C12;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_SUBROUTINE: types::GLenum = 0x92EB;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_1D_ARRAY: types::GLenum = 0x8DD6;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BACK_REF: types::GLenum = 0x8CA3;
#[allow(dead_code, non_upper_case_globals)]
pub const LAYER_PROVOKING_VERTEX: types::GLenum = 0x825E;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_STENCIL_TEXTURE_MODE: types::GLenum = 0x90EA;
#[allow(dead_code, non_upper_case_globals)]
pub const FRONT_FACE: types::GLenum = 0x0B46;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAGMENT_SHADER_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA8UI: types::GLenum = 0x8D7C;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_STRIP_ADJACENCY: types::GLenum = 0x000B;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_SRGB: types::GLenum = 0x8DB9;
#[allow(dead_code, non_upper_case_globals)]
pub const TRIANGLE_STRIP: types::GLenum = 0x0005;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: types::GLenum = 0x92CB;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9105;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8E81;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_PAUSED: types::GLenum = 0x8E23;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_OUTPUT: types::GLenum = 0x92E0;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_LAYERS: types::GLenum = 0x8281;
#[allow(dead_code, non_upper_case_globals)]
pub const GREEN_INTEGER: types::GLenum = 0x8D95;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE0: types::GLenum = 0x3000;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_MAPPED: types::GLenum = 0x88BC;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_SUBROUTINE: types::GLenum = 0x92E8;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_DATA_SIZE: types::GLenum = 0x8A40;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_ALIGNMENT: types::GLenum = 0x0CF5;
#[allow(dead_code, non_upper_case_globals)]
pub const TYPE: types::GLenum = 0x92FA;
#[allow(dead_code, non_upper_case_globals)]
pub const REFERENCED_BY_TESS_EVALUATION_SHADER: types::GLenum = 0x9308;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY: types::GLenum = 0x82E3;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_DEPTH: types::GLenum = 0x8280;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_BYTE_2_3_3_REV: types::GLenum = 0x8362;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_SPRITE_COORD_ORIGIN: types::GLenum = 0x8CA0;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_2D: types::GLenum = 0x9063;
#[allow(dead_code, non_upper_case_globals)]
pub const SYNC_FLUSH_COMMANDS_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SIGNED_RG_RGTC2: types::GLenum = 0x8DBE;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D_ARRAY_SHADOW: types::GLenum = 0x8DC4;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_STRIP: types::GLenum = 0x0003;
#[allow(dead_code, non_upper_case_globals)]
pub const LOW_FLOAT: types::GLenum = 0x8DF0;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_COMPRESSED_BLOCK_HEIGHT: types::GLenum = 0x912C;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_1D_ARRAY: types::GLenum = 0x8C1C;
#[allow(dead_code, non_upper_case_globals)]
pub const UPPER_LEFT: types::GLenum = 0x8CA2;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8E82;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_COMPRESSED_BLOCK_SIZE: types::GLenum = 0x912A;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_SMOOTH: types::GLenum = 0x0B41;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_NAME_LENGTH: types::GLenum = 0x92F6;
#[allow(dead_code, non_upper_case_globals)]
pub const NEAREST_MIPMAP_LINEAR: types::GLenum = 0x2702;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_EVALUATION_SHADER_BIT: types::GLenum = 0x00000010;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_OUTPUT_COMPONENTS: types::GLenum = 0x9122;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB8: types::GLenum = 0x8051;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_ATTRIBS: types::GLenum = 0x8869;
#[allow(dead_code, non_upper_case_globals)]
pub const NOTEQUAL: types::GLenum = 0x0205;
#[allow(dead_code, non_upper_case_globals)]
pub const RASTERIZER_DISCARD: types::GLenum = 0x8C89;
#[allow(dead_code, non_upper_case_globals)]
pub const MANUAL_GENERATE_MIPMAP: types::GLenum = 0x8294;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SWIZZLE_B: types::GLenum = 0x8E44;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_SHADER_STORAGE_BLOCK_SIZE: types::GLenum = 0x90DE;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_8_8_8_8: types::GLenum = 0x8035;
#[allow(dead_code, non_upper_case_globals)]
pub const GEQUAL: types::GLenum = 0x0206;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_CUBE_MAP_ARRAY: types::GLenum = 0x905F;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_IMAGE_UNIFORMS: types::GLenum = 0x90CA;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_NAME_LENGTH: types::GLenum = 0x8A39;
#[allow(dead_code, non_upper_case_globals)]
pub const FRACTIONAL_ODD: types::GLenum = 0x8E7B;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_VEC2: types::GLenum = 0x8B53;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_2D_RECT: types::GLenum = 0x9065;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_WIDTH_GRANULARITY: types::GLenum = 0x0B23;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR: types::GLenum = 0x824E;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNALED: types::GLenum = 0x9118;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_INDEX8: types::GLenum = 0x8D48;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_GREEN_SIZE: types::GLenum = 0x8D51;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: types::GLenum = 0x90C8;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_2D_ARRAY: types::GLenum = 0x8C1B;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPRESSED_BLOCK_HEIGHT: types::GLenum = 0x82B2;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_WRITEMASK: types::GLenum = 0x0B72;
#[allow(dead_code, non_upper_case_globals)]
pub const AND_INVERTED: types::GLenum = 0x1504;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_3D: types::GLenum = 0x8DD3;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_BINDING_NAME: types::GLenum = 0x8F3A;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_UNIFORM_BLOCKS: types::GLenum = 0x91BB;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT3x2: types::GLenum = 0x8B67;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_OFFSET_FACTOR: types::GLenum = 0x8038;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE1: types::GLenum = 0x84C1;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_4_X_8: types::GLenum = 0x82BF;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_SUBROUTINE_UNIFORMS: types::GLenum = 0x8DE6;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAGMENT_TEXTURE: types::GLenum = 0x829F;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: types::GLenum = 0x8A46;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_DEFAULT_WIDTH: types::GLenum = 0x9310;
#[allow(dead_code, non_upper_case_globals)]
pub const DELETE_STATUS: types::GLenum = 0x8B80;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_1D: types::GLenum = 0x9062;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BARRIER_BIT: types::GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE2: types::GLenum = 0x3002;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_8_8_8_8_REV: types::GLenum = 0x8367;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER10: types::GLenum = 0x882F;
#[allow(dead_code, non_upper_case_globals)]
pub const TOP_LEVEL_ARRAY_STRIDE: types::GLenum = 0x930D;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEWPORT: types::GLenum = 0x0BA2;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RGB_BPTC_SIGNED_FLOAT: types::GLenum = 0x8E8E;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER6: types::GLenum = 0x882B;
#[allow(dead_code, non_upper_case_globals)]
pub const INVALID_ENUM: types::GLenum = 0x0500;
#[allow(dead_code, non_upper_case_globals)]
pub const BGR: types::GLenum = 0x80E0;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RGBA: types::GLenum = 0x84EE;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_COMPONENTS: types::GLenum = 0x8283;
#[allow(dead_code, non_upper_case_globals)]
pub const ALL_SHADER_BITS: types::GLenum = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)]
pub const R8I: types::GLenum = 0x8231;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x906C;
#[allow(dead_code, non_upper_case_globals)]
pub const SYNC_FLAGS: types::GLenum = 0x9115;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB8I: types::GLenum = 0x8D8F;
#[allow(dead_code, non_upper_case_globals)]
pub const PRIMITIVE_RESTART: types::GLenum = 0x8F9D;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_STORAGE_BUFFER_START: types::GLenum = 0x90D4;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_CUBE: types::GLenum = 0x8B60;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT5: types::GLenum = 0x8CE5;
#[allow(dead_code, non_upper_case_globals)]
pub const EQUIV: types::GLenum = 0x1509;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_SRC: types::GLenum = 0x0BE1;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER: types::GLenum = 0x8CDC;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_BINDING_LAYERED: types::GLenum = 0x8F3C;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_RENDERABLE_LAYERED: types::GLenum = 0x828A;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB10: types::GLenum = 0x8052;
#[allow(dead_code, non_upper_case_globals)]
pub const PATCH_VERTICES: types::GLenum = 0x8E72;
#[allow(dead_code, non_upper_case_globals)]
pub const COPY_READ_BUFFER_BINDING: types::GLenum = 0x8F36;
#[allow(dead_code, non_upper_case_globals)]
pub const PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: types::GLenum = 0x8221;
#[allow(dead_code, non_upper_case_globals)]
pub const BLOCK_INDEX: types::GLenum = 0x92FD;
#[allow(dead_code, non_upper_case_globals)]
pub const GREEN: types::GLenum = 0x1904;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB16UI: types::GLenum = 0x8D77;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_PATCH_VERTICES: types::GLenum = 0x8E7D;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPRESSED_BLOCK_WIDTH: types::GLenum = 0x82B1;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_2D_RECT: types::GLenum = 0x8DCD;
#[allow(dead_code, non_upper_case_globals)]
pub const BGRA: types::GLenum = 0x80E1;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_BUFFER: types::GLenum = 0x905C;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_POSITIVE_Z: types::GLenum = 0x8519;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_CUBE_MAP: types::GLenum = 0x8514;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_11_11_10: types::GLenum = 0x82C2;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE11: types::GLenum = 0x84CB;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_HEIGHT: types::GLenum = 0x827F;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT14: types::GLenum = 0x8CEE;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: types::GLenum = 0x8E1F;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: types::GLenum = 0x8CD1;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT: types::GLenum = 0x1B00;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BUFFER_BIT: types::GLenum = 0x00000400;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE27: types::GLenum = 0x84DB;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPUTE_SUBROUTINE_UNIFORM: types::GLenum = 0x92F3;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TEXTURE_BUFFER_SIZE: types::GLenum = 0x8C2B;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_S3TC_DXT1_RGBA: types::GLenum = 0x82CD;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_PASS_DEPTH_FAIL: types::GLenum = 0x0B95;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE: types::GLenum = 0x1B01;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB_INTEGER: types::GLenum = 0x8D98;
#[allow(dead_code, non_upper_case_globals)]
pub const DISPATCH_INDIRECT_BUFFER: types::GLenum = 0x90EE;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: types::GLenum = 0x8C85;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_SHARED_MEMORY_SIZE: types::GLenum = 0x8262;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_WIDTH_RANGE: types::GLenum = 0x0B22;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA_INTEGER: types::GLenum = 0x8D99;
#[allow(dead_code, non_upper_case_globals)]
pub const STREAM_DRAW: types::GLenum = 0x88E0;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_BUFFER: types::GLenum = 0x8DD0;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_INTERNAL_FORMAT: types::GLenum = 0x1003;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPUTE_TEXTURE: types::GLenum = 0x82A0;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_BUFFER: types::GLenum = 0x9067;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: types::GLenum = 0x92DA;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_INDEX: types::GLenum = 0x934B;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_COMPONENT16: types::GLenum = 0x81A5;
#[allow(dead_code, non_upper_case_globals)]
pub const NO_ERROR: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9103;
#[allow(dead_code, non_upper_case_globals)]
pub const TRIANGLES_ADJACENCY: types::GLenum = 0x000C;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_COMPONENT32F: types::GLenum = 0x8CAC;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: types::GLenum = 0x8A35;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: types::GLenum = 0x900F;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_STORAGE_BUFFER_BINDING: types::GLenum = 0x90D3;
#[allow(dead_code, non_upper_case_globals)]
pub const NAME_LENGTH: types::GLenum = 0x92F9;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_VARYINGS: types::GLenum = 0x8C83;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_IMAGE_UNIFORMS: types::GLenum = 0x91BD;
#[allow(dead_code, non_upper_case_globals)]
pub const PROGRAM_OUTPUT: types::GLenum = 0x92E4;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_WRAP_R: types::GLenum = 0x8072;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VIEWPORTS: types::GLenum = 0x825B;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_UNIFORM_BUFFER_BINDINGS: types::GLenum = 0x8A2F;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR: types::GLenum = 0x824D;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA4: types::GLenum = 0x8056;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB12: types::GLenum = 0x8053;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_OUTPUT_VERTICES: types::GLenum = 0x8DE0;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_TARGET: types::GLenum = 0x82EA;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_BY_REGION_WAIT: types::GLenum = 0x8E15;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: types::GLenum = 0x8E80;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BUFFER: types::GLenum = 0x8A11;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_MAP_OFFSET: types::GLenum = 0x9121;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_SHORT_5_5_5_1: types::GLenum = 0x8034;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_BINDING_ACCESS: types::GLenum = 0x8F3E;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAGMENT_INPUT_COMPONENTS: types::GLenum = 0x9125;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE22: types::GLenum = 0x84D6;
#[allow(dead_code, non_upper_case_globals)]
pub const RG16: types::GLenum = 0x822C;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_SOURCE_SHADER_COMPILER: types::GLenum = 0x8248;
#[allow(dead_code, non_upper_case_globals)]
pub const SUBPIXEL_BITS: types::GLenum = 0x0D50;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_WIDTH: types::GLenum = 0x1000;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_INDIRECT_BUFFER: types::GLenum = 0x8F3F;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_BINDING: types::GLenum = 0x8A3F;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_S3TC_DXT3_RGBA: types::GLenum = 0x82CE;
#[allow(dead_code, non_upper_case_globals)]
pub const MULTISAMPLE: types::GLenum = 0x809D;
#[allow(dead_code, non_upper_case_globals)]
pub const LINES: types::GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)]
pub const ELEMENT_ARRAY_BUFFER_BINDING: types::GLenum = 0x8895;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_OFFSET_UNITS: types::GLenum = 0x2A00;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BINDING: types::GLenum = 0x8E25;
#[allow(dead_code, non_upper_case_globals)]
pub const PROGRAM: types::GLenum = 0x82E2;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_IMMUTABLE_FORMAT: types::GLenum = 0x912F;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_CONTROL_SHADER_BIT: types::GLenum = 0x00000008;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_LONG: types::GLenum = 0x874E;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT27: types::GLenum = 0x8CFB;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB10_A2UI: types::GLenum = 0x906F;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_FUNC: types::GLenum = 0x0B92;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_LABEL_LENGTH: types::GLenum = 0x82E8;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CUBE: types::GLenum = 0x9050;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_STORAGE_FLAGS: types::GLenum = 0x8220;
#[allow(dead_code, non_upper_case_globals)]
pub const CULL_FACE: types::GLenum = 0x0B44;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_1D_ARRAY: types::GLenum = 0x9052;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_BINDING: types::GLenum = 0x9302;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_BUFFER_BARRIER_BIT: types::GLenum = 0x00000080;
#[allow(dead_code, non_upper_case_globals)]
pub const ARRAY_STRIDE: types::GLenum = 0x92FE;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX: types::GLenum = 0x8008;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_BPTC_FLOAT: types::GLenum = 0x82D3;
#[allow(dead_code, non_upper_case_globals)]
pub const COPY_WRITE_BUFFER: types::GLenum = 0x8F37;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_SMOOTH_HINT: types::GLenum = 0x0C53;
#[allow(dead_code, non_upper_case_globals)]
pub const FRONT_LEFT: types::GLenum = 0x0400;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_COMPATIBILITY_PROFILE_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE10: types::GLenum = 0x84CA;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPUTE_SHADER: types::GLenum = 0x91B9;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB9_E5: types::GLenum = 0x8C3D;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_UNIFORM_BLOCKS: types::GLenum = 0x8A2C;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAGMENT_UNIFORM_BLOCKS: types::GLenum = 0x8A2D;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE7: types::GLenum = 0x3007;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_ATOMIC_COUNTERS: types::GLenum = 0x92D5;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_PROGRAM: types::GLenum = 0x8259;
#[allow(dead_code, non_upper_case_globals)]
pub const BYTE: types::GLenum = 0x1400;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92D1;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: types::GLenum = 0x8D56;
#[allow(dead_code, non_upper_case_globals)]
pub const SRC_ALPHA_SATURATE: types::GLenum = 0x0308;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_ATOMIC_COUNTER_BUFFERS: types::GLenum = 0x92D9;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BUFFER_BINDING: types::GLenum = 0x8A28;
#[allow(dead_code, non_upper_case_globals)]
pub const MAJOR_VERSION: types::GLenum = 0x821B;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE26: types::GLenum = 0x84DA;
#[allow(dead_code, non_upper_case_globals)]
pub const CLAMP_READ_COLOR: types::GLenum = 0x891C;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_MAX_LOD: types::GLenum = 0x813B;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_EVALUATION_SHADER: types::GLenum = 0x8E87;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: types::GLenum = 0x910A;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_WAIT: types::GLenum = 0x8E13;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_MAT4x3: types::GLenum = 0x8F4E;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_FIXED_SAMPLE_LOCATIONS: types::GLenum = 0x9107;
#[allow(dead_code, non_upper_case_globals)]
pub const STATIC_COPY: types::GLenum = 0x88E6;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB: types::GLenum = 0x1907;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_2D: types::GLenum = 0x8064;
#[allow(dead_code, non_upper_case_globals)]
pub const LEFT: types::GLenum = 0x0406;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D_RECT: types::GLenum = 0x8B63;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_COUNTER_BITS: types::GLenum = 0x8864;
#[allow(dead_code, non_upper_case_globals)]
pub const SHORT: types::GLenum = 0x1402;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_MAP_LENGTH: types::GLenum = 0x9120;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_DEBUG_MESSAGE_LENGTH: types::GLenum = 0x9143;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: types::GLenum = 0x8DD7;
#[allow(dead_code, non_upper_case_globals)]
pub const IS_PER_PATCH: types::GLenum = 0x92E7;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_1D_ARRAY: types::GLenum = 0x8C18;
#[allow(dead_code, non_upper_case_globals)]
pub const INCR_WRAP: types::GLenum = 0x8507;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SWIZZLE_R: types::GLenum = 0x8E42;
#[allow(dead_code, non_upper_case_globals)]
pub const RG8: types::GLenum = 0x822B;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT4x2: types::GLenum = 0x8B69;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_MODE: types::GLenum = 0x8C7F;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_TEXTURE: types::GLenum = 0x84E0;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_UNIFORM_BLOCK_SIZE: types::GLenum = 0x8A30;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: types::GLenum = 0x8266;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_64_BITS: types::GLenum = 0x82C6;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_GREEN_SIZE: types::GLenum = 0x8272;
#[allow(dead_code, non_upper_case_globals)]
pub const ARRAY_BUFFER: types::GLenum = 0x8892;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_2_X_8: types::GLenum = 0x82C0;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB32I: types::GLenum = 0x8D83;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90DB;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_RANGE: types::GLenum = 0x0B70;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: types::GLenum = 0x92C7;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA16F: types::GLenum = 0x881A;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_SHADER_INVOCATIONS: types::GLenum = 0x8E5A;
#[allow(dead_code, non_upper_case_globals)]
pub const LOCATION_COMPONENT: types::GLenum = 0x934A;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM: types::GLenum = 0x92E1;
#[allow(dead_code, non_upper_case_globals)]
pub const EQUAL: types::GLenum = 0x0202;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB565: types::GLenum = 0x8D62;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_RELEASE_BEHAVIOR: types::GLenum = 0x82FB;
#[allow(dead_code, non_upper_case_globals)]
pub const QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: types::GLenum = 0x8E4C;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_IMAGE_FORMAT: types::GLenum = 0x828F;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BACK_WRITEMASK: types::GLenum = 0x8CA5;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: types::GLenum = 0x90DA;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_WORK_GROUP_INVOCATIONS: types::GLenum = 0x90EB;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_COMPONENTS: types::GLenum = 0x8285;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_NUM_ACTIVE_VARIABLES: types::GLenum = 0x92F7;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_1D: types::GLenum = 0x0DE0;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_QUERY: types::GLenum = 0x8865;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: types::GLenum = 0x8CD2;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT2: types::GLenum = 0x8CE2;
#[allow(dead_code, non_upper_case_globals)]
pub const STREAM_READ: types::GLenum = 0x88E1;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D_ARRAY: types::GLenum = 0x8DC1;
#[allow(dead_code, non_upper_case_globals)]
pub const INDEX: types::GLenum = 0x8222;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER15: types::GLenum = 0x8834;
#[allow(dead_code, non_upper_case_globals)]
pub const RED_INTEGER: types::GLenum = 0x8D94;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BUFFER_OFFSET: types::GLenum = 0x919D;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_EQUATION_RGB: types::GLenum = 0x8009;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND: types::GLenum = 0x0BE2;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SWIZZLE_A: types::GLenum = 0x8E45;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_2D: types::GLenum = 0x8069;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: types::GLenum = 0x92DC;
#[allow(dead_code, non_upper_case_globals)]
pub const LINEAR: types::GLenum = 0x2601;
#[allow(dead_code, non_upper_case_globals)]
pub const R32F: types::GLenum = 0x822E;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_BY_REGION_NO_WAIT_INVERTED: types::GLenum = 0x8E1A;
#[allow(dead_code, non_upper_case_globals)]
pub const INVALID_OPERATION: types::GLenum = 0x0502;
#[allow(dead_code, non_upper_case_globals)]
pub const GET_TEXTURE_IMAGE_FORMAT: types::GLenum = 0x8291;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_TEXTURE_IMAGE_UNITS: types::GLenum = 0x91BC;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_BUFFER: types::GLenum = 0x9192;
#[allow(dead_code, non_upper_case_globals)]
pub const LOCATION_INDEX: types::GLenum = 0x930F;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_2D_MULTISAMPLE: types::GLenum = 0x9101;
#[allow(dead_code, non_upper_case_globals)]
pub const RESET_NOTIFICATION_STRATEGY: types::GLenum = 0x8256;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_STORAGE_BUFFER: types::GLenum = 0x90D2;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_VIEW: types::GLenum = 0x82B5;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_GEN_VERTEX_ORDER: types::GLenum = 0x8E78;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB10_A2: types::GLenum = 0x8059;
#[allow(dead_code, non_upper_case_globals)]
pub const CAVEAT_SUPPORT: types::GLenum = 0x82B8;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT4: types::GLenum = 0x8B5C;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_SOURCE_APPLICATION: types::GLenum = 0x824A;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8C29;
#[allow(dead_code, non_upper_case_globals)]
pub const REFERENCED_BY_COMPUTE_SHADER: types::GLenum = 0x930B;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_BPTC_UNORM: types::GLenum = 0x82D2;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_96_BITS: types::GLenum = 0x82C5;
#[allow(dead_code, non_upper_case_globals)]
pub const MIN_PROGRAM_TEXTURE_GATHER_OFFSET: types::GLenum = 0x8E5E;
#[allow(dead_code, non_upper_case_globals)]
pub const DISPATCH_INDIRECT_BUFFER_BINDING: types::GLenum = 0x90EF;
#[allow(dead_code, non_upper_case_globals)]
pub const PROGRAM_POINT_SIZE: types::GLenum = 0x8642;
#[allow(dead_code, non_upper_case_globals)]
pub const RG8I: types::GLenum = 0x8237;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER13: types::GLenum = 0x8832;
#[allow(dead_code, non_upper_case_globals)]
pub const R32I: types::GLenum = 0x8235;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: types::GLenum = 0x9314;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BACK_PASS_DEPTH_FAIL: types::GLenum = 0x8802;
#[allow(dead_code, non_upper_case_globals)]
pub const SMOOTH_POINT_SIZE_GRANULARITY: types::GLenum = 0x0B13;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT3: types::GLenum = 0x8CE3;
#[allow(dead_code, non_upper_case_globals)]
pub const NICEST: types::GLenum = 0x1102;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_VEC2: types::GLenum = 0x8DC6;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLES_PASSED: types::GLenum = 0x8914;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_1D_SHADOW: types::GLenum = 0x8B61;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEWPORT_SUBPIXEL_BITS: types::GLenum = 0x825C;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_MIN_FILTER: types::GLenum = 0x2801;
#[allow(dead_code, non_upper_case_globals)]
pub const ANY_SAMPLES_PASSED: types::GLenum = 0x8C2F;
#[allow(dead_code, non_upper_case_globals)]
pub const SIGNED_NORMALIZED: types::GLenum = 0x8F9C;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: types::GLenum = 0x92C9;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_READ_BIT: types::GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)]
pub const CONSTANT_ALPHA: types::GLenum = 0x8003;
#[allow(dead_code, non_upper_case_globals)]
pub const OR_REVERSE: types::GLenum = 0x150B;
#[allow(dead_code, non_upper_case_globals)]
pub const GUILTY_CONTEXT_RESET: types::GLenum = 0x8253;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER: types::GLenum = 0x92C0;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_BLUE_SIZE: types::GLenum = 0x8273;
#[allow(dead_code, non_upper_case_globals)]
pub const CLAMP_TO_BORDER: types::GLenum = 0x812D;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: types::GLenum = 0x889F;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: types::GLenum = 0x8A32;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER: types::GLenum = 0x8D41;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_VARIABLE: types::GLenum = 0x92E5;
#[allow(dead_code, non_upper_case_globals)]
pub const ZERO_TO_ONE: types::GLenum = 0x935F;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VIEWPORT_DIMS: types::GLenum = 0x0D3A;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_RENDERBUFFER_SIZE: types::GLenum = 0x84E8;
#[allow(dead_code, non_upper_case_globals)]
pub const CLAMP_TO_EDGE: types::GLenum = 0x812F;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_R11_EAC: types::GLenum = 0x9270;
#[allow(dead_code, non_upper_case_globals)]
pub const COMMAND_BARRIER_BIT: types::GLenum = 0x00000040;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_PERSISTENT_BIT: types::GLenum = 0x0040;
#[allow(dead_code, non_upper_case_globals)]
pub const FRONT: types::GLenum = 0x0404;
#[allow(dead_code, non_upper_case_globals)]
pub const CCW: types::GLenum = 0x0901;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_BUFFER: types::GLenum = 0x9051;
#[allow(dead_code, non_upper_case_globals)]
pub const PROGRAM_BINARY_FORMATS: types::GLenum = 0x87FF;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_DEBUG_LOGGED_MESSAGES: types::GLenum = 0x9144;
#[allow(dead_code, non_upper_case_globals)]
pub const DST_ALPHA: types::GLenum = 0x0304;
#[allow(dead_code, non_upper_case_globals)]
pub const R16: types::GLenum = 0x822A;
#[allow(dead_code, non_upper_case_globals)]
pub const FASTEST: types::GLenum = 0x1101;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_ATOMIC_COUNTERS: types::GLenum = 0x92D7;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_BUFFER_BIT: types::GLenum = 0x00004000;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_NO_WAIT: types::GLenum = 0x8E14;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_3D: types::GLenum = 0x9059;
#[allow(dead_code, non_upper_case_globals)]
pub const EXTENSIONS: types::GLenum = 0x1F03;
#[allow(dead_code, non_upper_case_globals)]
pub const LINES_ADJACENCY: types::GLenum = 0x000A;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9102;
#[allow(dead_code, non_upper_case_globals)]
pub const SYNC_CONDITION: types::GLenum = 0x9113;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER9: types::GLenum = 0x882E;
#[allow(dead_code, non_upper_case_globals)]
pub const ARRAY_SIZE: types::GLenum = 0x92FB;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: types::GLenum = 0x8CD3;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_RESOURCES: types::GLenum = 0x92F5;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_UPDATE_BARRIER_BIT: types::GLenum = 0x00000100;
#[allow(dead_code, non_upper_case_globals)]
pub const FALSE: types::GLboolean = 0;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_CONTROL_OUTPUT_COMPONENTS: types::GLenum = 0x8E83;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_SERVER_WAIT_TIMEOUT: types::GLenum = 0x9111;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_UNSYNCHRONIZED_BIT: types::GLenum = 0x0020;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: types::GLenum = 0x84F1;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_MASK_VALUE: types::GLenum = 0x8E52;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BARRIER_BIT: types::GLenum = 0x00000800;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_COMPRESSED_BLOCK_WIDTH: types::GLenum = 0x912B;
#[allow(dead_code, non_upper_case_globals)]
pub const RG32UI: types::GLenum = 0x823C;
#[allow(dead_code, non_upper_case_globals)]
pub const SMOOTH_POINT_SIZE_RANGE: types::GLenum = 0x0B12;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SRGB_ALPHA: types::GLenum = 0x8C49;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: types::GLenum = 0x8210;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_ELEMENTS_INDICES: types::GLenum = 0x80E9;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_GEN_POINT_MODE: types::GLenum = 0x8E79;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE3: types::GLenum = 0x3003;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE6: types::GLenum = 0x3006;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: types::GLenum = 0x8A33;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE4: types::GLenum = 0x84C4;
#[allow(dead_code, non_upper_case_globals)]
pub const R32UI: types::GLenum = 0x8236;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_PACK_BUFFER_BINDING: types::GLenum = 0x88ED;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_SEVERITY_LOW: types::GLenum = 0x9148;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_3D: types::GLenum = 0x9064;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB5: types::GLenum = 0x8050;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_NORMALIZED: types::GLenum = 0x8C17;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER14: types::GLenum = 0x8833;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_8_BITS: types::GLenum = 0x82CB;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RGBA_BPTC_UNORM: types::GLenum = 0x8E8C;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_SKIP_PIXELS: types::GLenum = 0x0CF4;
/// Fallbacks: EndTransformFeedbackEXT, EndTransformFeedbackNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn EndTransformFeedback() -> () {
    mem::transmute::<_, extern "system" fn() -> ()>(storage::EndTransformFeedback.f)()
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetProgramResourceLocationIndex(program: types::GLuint, programInterface: types::GLenum, name: *const types::GLchar) -> types::GLint {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLchar) -> types::GLint>(storage::GetProgramResourceLocationIndex.f)(program, programInterface, name)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetProgramResourceLocation(program: types::GLuint, programInterface: types::GLenum, name: *const types::GLchar) -> types::GLint {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLchar) -> types::GLint>(storage::GetProgramResourceLocation.f)(program, programInterface, name)
}
/// Fallbacks: VertexAttribL3dEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribL3d(index: types::GLuint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::VertexAttribL3d.f)(index, x, y, z)
}
/// Fallbacks: ObjectPtrLabelKHR
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ObjectPtrLabel(ptr: *const c_void, length: types::GLsizei, label: *const types::GLchar) -> () {
    mem::transmute::<_, extern "system" fn(*const c_void, types::GLsizei, *const types::GLchar) -> ()>(storage::ObjectPtrLabel.f)(ptr, length, label)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ActiveShaderProgram(pipeline: types::GLuint, program: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(storage::ActiveShaderProgram.f)(pipeline, program)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindProgramPipeline(pipeline: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::BindProgramPipeline.f)(pipeline)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CreateProgramPipelines(n: types::GLsizei, pipelines: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::CreateProgramPipelines.f)(n, pipelines)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn NormalP3ui(type_: types::GLenum, coords: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::NormalP3ui.f)(type_, coords)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UseProgramStages(pipeline: types::GLuint, stages: types::GLbitfield, program: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLbitfield, types::GLuint) -> ()>(storage::UseProgramStages.f)(pipeline, stages, program)
}
/// Fallbacks: VertexAttribL2dEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribL2d(index: types::GLuint, x: types::GLdouble, y: types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble) -> ()>(storage::VertexAttribL2d.f)(index, x, y)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetnHistogram(target: types::GLenum, reset: types::GLboolean, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, values: *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLboolean, types::GLenum, types::GLenum, types::GLsizei, *mut c_void) -> ()>(storage::GetnHistogram.f)(target, reset, format, type_, bufSize, values)
}
/// Fallbacks: ScissorArrayvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ScissorArrayv(first: types::GLuint, count: types::GLsizei, v: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLint) -> ()>(storage::ScissorArrayv.f)(first, count, v)
}
/// Fallbacks: VertexAttribDivisorANGLE, VertexAttribDivisorARB, VertexAttribDivisorEXT, VertexAttribDivisorNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribDivisor(index: types::GLuint, divisor: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(storage::VertexAttribDivisor.f)(index, divisor)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetTexImage(target: types::GLenum, level: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLenum, *mut c_void) -> ()>(storage::GetTexImage.f)(target, level, format, type_, pixels)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn SamplerParameteri(sampler: types::GLuint, pname: types::GLenum, param: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLint) -> ()>(storage::SamplerParameteri.f)(sampler, pname, param)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TextureBarrier() -> () {
    mem::transmute::<_, extern "system" fn() -> ()>(storage::TextureBarrier.f)()
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TextureParameteri(texture: types::GLuint, pname: types::GLenum, param: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLint) -> ()>(storage::TextureParameteri.f)(texture, pname, param)
}
/// Fallbacks: GetObjectLabelKHR
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetObjectLabel(identifier: types::GLenum, name: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, label: *mut types::GLchar) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(storage::GetObjectLabel.f)(identifier, name, bufSize, length, label)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ReadBuffer(src: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::ReadBuffer.f)(src)
}
/// Fallbacks: StencilOpSeparateATI
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn StencilOpSeparate(face: types::GLenum, sfail: types::GLenum, dpfail: types::GLenum, dppass: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLenum) -> ()>(storage::StencilOpSeparate.f)(face, sfail, dpfail, dppass)
}
/// Fallbacks: TexSubImage2DEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexSubImage2D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *const c_void) -> ()>(storage::TexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, type_, pixels)
}
/// Fallbacks: GetTransformFeedbackVaryingEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetTransformFeedbackVarying(program: types::GLuint, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, size: *mut types::GLsizei, type_: *mut types::GLenum, name: *mut types::GLchar) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLsizei, *mut types::GLenum, *mut types::GLchar) -> ()>(storage::GetTransformFeedbackVarying.f)(program, index, bufSize, length, size, type_, name)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn MapNamedBufferRange(buffer: types::GLuint, offset: types::GLintptr, length: types::GLsizeiptr, access: types::GLbitfield) -> *mut c_void {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLintptr, types::GLsizeiptr, types::GLbitfield) -> *mut c_void>(storage::MapNamedBufferRange.f)(buffer, offset, length, access)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn SamplerParameteriv(sampler: types::GLuint, pname: types::GLenum, param: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLint) -> ()>(storage::SamplerParameteriv.f)(sampler, pname, param)
}
/// Fallbacks: ProgramUniform4fvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform4fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(storage::ProgramUniform4fv.f)(program, location, count, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UniformMatrix4x3dv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(storage::UniformMatrix4x3dv.f)(location, count, transpose, value)
}
/// Fallbacks: ScissorIndexedvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ScissorIndexedv(index: types::GLuint, v: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(storage::ScissorIndexedv.f)(index, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindImageTexture(unit: types::GLuint, texture: types::GLuint, level: types::GLint, layered: types::GLboolean, layer: types::GLint, access: types::GLenum, format: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLint, types::GLboolean, types::GLint, types::GLenum, types::GLenum) -> ()>(storage::BindImageTexture.f)(unit, texture, level, layered, layer, access, format)
}
/// Fallbacks: BlendColorEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BlendColor(red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::BlendColor.f)(red, green, blue, alpha)
}
/// Fallbacks: GetPointervEXT, GetPointervKHR
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetPointerv(pname: types::GLenum, params: *const *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *const *mut c_void) -> ()>(storage::GetPointerv.f)(pname, params)
}
/// Fallbacks: ProgramUniform2uivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform2uiv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLuint) -> ()>(storage::ProgramUniform2uiv.f)(program, location, count, value)
}
/// Fallbacks: DrawElementsInstancedBaseVertexBaseInstanceEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DrawElementsInstancedBaseVertexBaseInstance(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const c_void, instancecount: types::GLsizei, basevertex: types::GLint, baseinstance: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, *const c_void, types::GLsizei, types::GLint, types::GLuint) -> ()>(storage::DrawElementsInstancedBaseVertexBaseInstance.f)(mode, count, type_, indices, instancecount, basevertex, baseinstance)
}
/// Fallbacks: GetInteger64vAPPLE
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetInteger64v(pname: types::GLenum, data: *mut types::GLint64) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLint64) -> ()>(storage::GetInteger64v.f)(pname, data)
}
/// Fallbacks: VertexAttribI2uiEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribI2ui(index: types::GLuint, x: types::GLuint, y: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::VertexAttribI2ui.f)(index, x, y)
}
/// Fallbacks: ProgramUniform1iEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform1i(program: types::GLuint, location: types::GLint, v0: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint) -> ()>(storage::ProgramUniform1i.f)(program, location, v0)
}
/// Fallbacks: GetVertexAttribivARB, GetVertexAttribivNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetVertexAttribiv(index: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetVertexAttribiv.f)(index, pname, params)
}
/// Fallbacks: ProgramUniform4iEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform4i(program: types::GLuint, location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint, v3: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(storage::ProgramUniform4i.f)(program, location, v0, v1, v2, v3)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexArrayAttribBinding(vaobj: types::GLuint, attribindex: types::GLuint, bindingindex: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::VertexArrayAttribBinding.f)(vaobj, attribindex, bindingindex)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetFloatv(pname: types::GLenum, data: *mut types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLfloat) -> ()>(storage::GetFloatv.f)(pname, data)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DispatchComputeIndirect(indirect: types::GLintptr) -> () {
    mem::transmute::<_, extern "system" fn(types::GLintptr) -> ()>(storage::DispatchComputeIndirect.f)(indirect)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Enable(cap: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::Enable.f)(cap)
}
/// Fallbacks: BindBufferRangeEXT, BindBufferRangeNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindBufferRange(target: types::GLenum, index: types::GLuint, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLuint, types::GLintptr, types::GLsizeiptr) -> ()>(storage::BindBufferRange.f)(target, index, buffer, offset, size)
}
/// Fallbacks: ShaderSourceARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ShaderSource(shader: types::GLuint, count: types::GLsizei, string: *const *const types::GLchar, length: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const *const types::GLchar, *const types::GLint) -> ()>(storage::ShaderSource.f)(shader, count, string, length)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexArrayAttribIFormat(vaobj: types::GLuint, attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, relativeoffset: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLint, types::GLenum, types::GLuint) -> ()>(storage::VertexArrayAttribIFormat.f)(vaobj, attribindex, size, type_, relativeoffset)
}
/// Fallbacks: VertexAttribI4ubvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribI4ubv(index: types::GLuint, v: *const types::GLubyte) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLubyte) -> ()>(storage::VertexAttribI4ubv.f)(index, v)
}
/// Fallbacks: VertexAttrib1sARB, VertexAttrib1sNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib1s(index: types::GLuint, x: types::GLshort) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLshort) -> ()>(storage::VertexAttrib1s.f)(index, x)
}
/// Fallbacks: VertexAttribI2ivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribI2iv(index: types::GLuint, v: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(storage::VertexAttribI2iv.f)(index, v)
}
/// Fallbacks: GetObjectPtrLabelKHR
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetObjectPtrLabel(ptr: *const c_void, bufSize: types::GLsizei, length: *mut types::GLsizei, label: *mut types::GLchar) -> () {
    mem::transmute::<_, extern "system" fn(*const c_void, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(storage::GetObjectPtrLabel.f)(ptr, bufSize, length, label)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform2d(location: types::GLint, x: types::GLdouble, y: types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLdouble, types::GLdouble) -> ()>(storage::Uniform2d.f)(location, x, y)
}
/// Fallbacks: MultiDrawArraysIndirectAMD, MultiDrawArraysIndirectEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn MultiDrawArraysIndirect(mode: types::GLenum, indirect: *const c_void, drawcount: types::GLsizei, stride: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *const c_void, types::GLsizei, types::GLsizei) -> ()>(storage::MultiDrawArraysIndirect.f)(mode, indirect, drawcount, stride)
}
/// Fallbacks: DrawArraysInstancedANGLE, DrawArraysInstancedARB, DrawArraysInstancedEXT, DrawArraysInstancedNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DrawArraysInstanced(mode: types::GLenum, first: types::GLint, count: types::GLsizei, instancecount: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::DrawArraysInstanced.f)(mode, first, count, instancecount)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetVertexArrayIndexed64iv(vaobj: types::GLuint, index: types::GLuint, pname: types::GLenum, param: *mut types::GLint64) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLenum, *mut types::GLint64) -> ()>(storage::GetVertexArrayIndexed64iv.f)(vaobj, index, pname, param)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetQueryIndexediv(target: types::GLenum, index: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetQueryIndexediv.f)(target, index, pname, params)
}
/// Fallbacks: GetFragDataLocationEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetFragDataLocation(program: types::GLuint, name: *const types::GLchar) -> types::GLint {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLint>(storage::GetFragDataLocation.f)(program, name)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DispatchCompute(num_groups_x: types::GLuint, num_groups_y: types::GLuint, num_groups_z: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::DispatchCompute.f)(num_groups_x, num_groups_y, num_groups_z)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CopyTextureSubImage2D(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::CopyTextureSubImage2D.f)(texture, level, xoffset, yoffset, x, y, width, height)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ClearTexImage(texture: types::GLuint, level: types::GLint, format: types::GLenum, type_: types::GLenum, data: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLenum, *const c_void) -> ()>(storage::ClearTexImage.f)(texture, level, format, type_, data)
}
/// Fallbacks: VertexAttribI4uiEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribI4ui(index: types::GLuint, x: types::GLuint, y: types::GLuint, z: types::GLuint, w: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::VertexAttribI4ui.f)(index, x, y, z, w)
}
/// Fallbacks: VertexAttrib4NsvARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib4Nsv(index: types::GLuint, v: *const types::GLshort) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLshort) -> ()>(storage::VertexAttrib4Nsv.f)(index, v)
}
/// Fallbacks: VertexAttribI3iEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribI3i(index: types::GLuint, x: types::GLint, y: types::GLint, z: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint) -> ()>(storage::VertexAttribI3i.f)(index, x, y, z)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribP4uiv(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLboolean, *const types::GLuint) -> ()>(storage::VertexAttribP4uiv.f)(index, type_, normalized, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribP2uiv(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLboolean, *const types::GLuint) -> ()>(storage::VertexAttribP2uiv.f)(index, type_, normalized, value)
}
/// Fallbacks: ProgramUniform2uiEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform2ui(program: types::GLuint, location: types::GLint, v0: types::GLuint, v1: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLuint, types::GLuint) -> ()>(storage::ProgramUniform2ui.f)(program, location, v0, v1)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Viewport(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::Viewport.f)(x, y, width, height)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetError() -> types::GLenum {
    mem::transmute::<_, extern "system" fn() -> types::GLenum>(storage::GetError.f)()
}
/// Fallbacks: DrawBuffersARB, DrawBuffersATI, DrawBuffersEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DrawBuffers(n: types::GLsizei, bufs: *const types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLenum) -> ()>(storage::DrawBuffers.f)(n, bufs)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetTextureLevelParameterfv(texture: types::GLuint, level: types::GLint, pname: types::GLenum, params: *mut types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, *mut types::GLfloat) -> ()>(storage::GetTextureLevelParameterfv.f)(texture, level, pname, params)
}
/// Fallbacks: NamedBufferStorageEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn NamedBufferStorage(buffer: types::GLuint, size: types::GLsizeiptr, data: *const c_void, flags: types::GLbitfield) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizeiptr, *const c_void, types::GLbitfield) -> ()>(storage::NamedBufferStorage.f)(buffer, size, data, flags)
}
/// Fallbacks: DrawRangeElementsBaseVertexEXT, DrawRangeElementsBaseVertexOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DrawRangeElementsBaseVertex(mode: types::GLenum, start: types::GLuint, end: types::GLuint, count: types::GLsizei, type_: types::GLenum, indices: *const c_void, basevertex: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLuint, types::GLsizei, types::GLenum, *const c_void, types::GLint) -> ()>(storage::DrawRangeElementsBaseVertex.f)(mode, start, end, count, type_, indices, basevertex)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniformMatrix2dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(storage::ProgramUniformMatrix2dv.f)(program, location, count, transpose, value)
}
/// Fallbacks: GetVertexAttribdvARB, GetVertexAttribdvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetVertexAttribdv(index: types::GLuint, pname: types::GLenum, params: *mut types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLdouble) -> ()>(storage::GetVertexAttribdv.f)(index, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetnUniformdv(program: types::GLuint, location: types::GLint, bufSize: types::GLsizei, params: *mut types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *mut types::GLdouble) -> ()>(storage::GetnUniformdv.f)(program, location, bufSize, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ClearBufferuiv(buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, *const types::GLuint) -> ()>(storage::ClearBufferuiv.f)(buffer, drawbuffer, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn IsEnabled(cap: types::GLenum) -> types::GLboolean {
    mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLboolean>(storage::IsEnabled.f)(cap)
}
/// Fallbacks: DrawTransformFeedbackNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DrawTransformFeedback(mode: types::GLenum, id: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::DrawTransformFeedback.f)(mode, id)
}
/// Fallbacks: VertexAttribL2dvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribL2dv(index: types::GLuint, v: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(storage::VertexAttribL2dv.f)(index, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DepthFunc(func: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::DepthFunc.f)(func)
}
/// Fallbacks: MultiDrawElementsEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn MultiDrawElements(mode: types::GLenum, count: *const types::GLsizei, type_: types::GLenum, indices: *const *const c_void, drawcount: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLsizei, types::GLenum, *const *const c_void, types::GLsizei) -> ()>(storage::MultiDrawElements.f)(mode, count, type_, indices, drawcount)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Flush() -> () {
    mem::transmute::<_, extern "system" fn() -> ()>(storage::Flush.f)()
}
/// Fallbacks: GetUniformfvARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetUniformfv(program: types::GLuint, location: types::GLint, params: *mut types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, *mut types::GLfloat) -> ()>(storage::GetUniformfv.f)(program, location, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetnPixelMapuiv(map: types::GLenum, bufSize: types::GLsizei, values: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *mut types::GLuint) -> ()>(storage::GetnPixelMapuiv.f)(map, bufSize, values)
}
/// Fallbacks: GetQueryObjecti64vEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetQueryObjecti64v(id: types::GLuint, pname: types::GLenum, params: *mut types::GLint64) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint64) -> ()>(storage::GetQueryObjecti64v.f)(id, pname, params)
}
/// Fallbacks: GenerateMipmapEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GenerateMipmap(target: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::GenerateMipmap.f)(target)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DrawTransformFeedbackStream(mode: types::GLenum, id: types::GLuint, stream: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLuint) -> ()>(storage::DrawTransformFeedbackStream.f)(mode, id, stream)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetTexLevelParameterfv(target: types::GLenum, level: types::GLint, pname: types::GLenum, params: *mut types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, *mut types::GLfloat) -> ()>(storage::GetTexLevelParameterfv.f)(target, level, pname, params)
}
/// Fallbacks: VertexAttrib4uivARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib4uiv(index: types::GLuint, v: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLuint) -> ()>(storage::VertexAttrib4uiv.f)(index, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UniformMatrix4dv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(storage::UniformMatrix4dv.f)(location, count, transpose, value)
}
/// Fallbacks: VertexAttrib4dARB, VertexAttrib4dNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib4d(index: types::GLuint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble, w: types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::VertexAttrib4d.f)(index, x, y, z, w)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DepthMask(flag: types::GLboolean) -> () {
    mem::transmute::<_, extern "system" fn(types::GLboolean) -> ()>(storage::DepthMask.f)(flag)
}
/// Fallbacks: VertexAttribL4dEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribL4d(index: types::GLuint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble, w: types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::VertexAttribL4d.f)(index, x, y, z, w)
}
/// Fallbacks: CopyTexSubImage1DEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CopyTexSubImage1D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei) -> ()>(storage::CopyTexSubImage1D.f)(target, level, xoffset, x, y, width)
}
/// Fallbacks: Uniform1uiEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform1ui(location: types::GLint, v0: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLuint) -> ()>(storage::Uniform1ui.f)(location, v0)
}
/// Fallbacks: VertexAttrib4NubvARB, VertexAttrib4ubvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib4Nubv(index: types::GLuint, v: *const types::GLubyte) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLubyte) -> ()>(storage::VertexAttrib4Nubv.f)(index, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UniformSubroutinesuiv(shadertype: types::GLenum, count: types::GLsizei, indices: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const types::GLuint) -> ()>(storage::UniformSubroutinesuiv.f)(shadertype, count, indices)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Scissor(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::Scissor.f)(x, y, width, height)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TextureStorage3DMultisample(texture: types::GLuint, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, fixedsamplelocations: types::GLboolean) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei, types::GLsizei, types::GLboolean) -> ()>(storage::TextureStorage3DMultisample.f)(texture, samples, internalformat, width, height, depth, fixedsamplelocations)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn StencilFuncSeparate(face: types::GLenum, func: types::GLenum, ref_: types::GLint, mask: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint, types::GLuint) -> ()>(storage::StencilFuncSeparate.f)(face, func, ref_, mask)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexCoordP3uiv(type_: types::GLenum, coords: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(storage::TexCoordP3uiv.f)(type_, coords)
}
/// Fallbacks: ValidateProgramARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ValidateProgram(program: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::ValidateProgram.f)(program)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn InvalidateSubFramebuffer(target: types::GLenum, numAttachments: types::GLsizei, attachments: *const types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::InvalidateSubFramebuffer.f)(target, numAttachments, attachments, x, y, width, height)
}
/// Fallbacks: VertexAttrib3fvARB, VertexAttrib3fvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib3fv(index: types::GLuint, v: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(storage::VertexAttrib3fv.f)(index, v)
}
/// Fallbacks: DeleteVertexArraysAPPLE, DeleteVertexArraysOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DeleteVertexArrays(n: types::GLsizei, arrays: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteVertexArrays.f)(n, arrays)
}
/// Fallbacks: VertexAttribI4uivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribI4uiv(index: types::GLuint, v: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLuint) -> ()>(storage::VertexAttribI4uiv.f)(index, v)
}
/// Fallbacks: VertexAttrib4svARB, VertexAttrib4svNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib4sv(index: types::GLuint, v: *const types::GLshort) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLshort) -> ()>(storage::VertexAttrib4sv.f)(index, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn SamplerParameterf(sampler: types::GLuint, pname: types::GLenum, param: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLfloat) -> ()>(storage::SamplerParameterf.f)(sampler, pname, param)
}
/// Fallbacks: VertexAttribI1ivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribI1iv(index: types::GLuint, v: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(storage::VertexAttribI1iv.f)(index, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexParameteriv(target: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLint) -> ()>(storage::TexParameteriv.f)(target, pname, params)
}
/// Fallbacks: Uniform4iARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform4i(location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint, v3: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(storage::Uniform4i.f)(location, v0, v1, v2, v3)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexCoordP1ui(type_: types::GLenum, coords: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::TexCoordP1ui.f)(type_, coords)
}
/// Fallbacks: IsFramebufferEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn IsFramebuffer(framebuffer: types::GLuint) -> types::GLboolean {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsFramebuffer.f)(framebuffer)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn IsTexture(texture: types::GLuint) -> types::GLboolean {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsTexture.f)(texture)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BlendFunc(sfactor: types::GLenum, dfactor: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(storage::BlendFunc.f)(sfactor, dfactor)
}
/// Fallbacks: ProgramUniform4uiEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform4ui(program: types::GLuint, location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint, v3: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLuint, types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::ProgramUniform4ui.f)(program, location, v0, v1, v2, v3)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UniformMatrix2dv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(storage::UniformMatrix2dv.f)(location, count, transpose, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexArrayElementBuffer(vaobj: types::GLuint, buffer: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(storage::VertexArrayElementBuffer.f)(vaobj, buffer)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GenProgramPipelines(n: types::GLsizei, pipelines: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenProgramPipelines.f)(n, pipelines)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn NamedFramebufferReadBuffer(framebuffer: types::GLuint, src: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum) -> ()>(storage::NamedFramebufferReadBuffer.f)(framebuffer, src)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DrawElements(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, *const c_void) -> ()>(storage::DrawElements.f)(mode, count, type_, indices)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TextureParameteriv(texture: types::GLuint, pname: types::GLenum, param: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLint) -> ()>(storage::TextureParameteriv.f)(texture, pname, param)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn StencilOp(fail: types::GLenum, zfail: types::GLenum, zpass: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum) -> ()>(storage::StencilOp.f)(fail, zfail, zpass)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindVertexBuffers(first: types::GLuint, count: types::GLsizei, buffers: *const types::GLuint, offsets: *const types::GLintptr, strides: *const types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLuint, *const types::GLintptr, *const types::GLsizei) -> ()>(storage::BindVertexBuffers.f)(first, count, buffers, offsets, strides)
}
/// Fallbacks: PopDebugGroupKHR
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn PopDebugGroup() -> () {
    mem::transmute::<_, extern "system" fn() -> ()>(storage::PopDebugGroup.f)()
}
/// Fallbacks: Uniform2uiEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform2ui(location: types::GLint, v0: types::GLuint, v1: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLuint, types::GLuint) -> ()>(storage::Uniform2ui.f)(location, v0, v1)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn SecondaryColorP3uiv(type_: types::GLenum, color: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(storage::SecondaryColorP3uiv.f)(type_, color)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindSampler(unit: types::GLuint, sampler: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(storage::BindSampler.f)(unit, sampler)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform1dv(location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(storage::Uniform1dv.f)(location, count, value)
}
/// Fallbacks: VertexAttrib3dARB, VertexAttrib3dNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib3d(index: types::GLuint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::VertexAttrib3d.f)(index, x, y, z)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetNamedBufferPointerv(buffer: types::GLuint, pname: types::GLenum, params: *const *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const *mut c_void) -> ()>(storage::GetNamedBufferPointerv.f)(buffer, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CreateSamplers(n: types::GLsizei, samplers: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::CreateSamplers.f)(n, samplers)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn EndQueryIndexed(target: types::GLenum, index: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::EndQueryIndexed.f)(target, index)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ClearBufferfv(buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, *const types::GLfloat) -> ()>(storage::ClearBufferfv.f)(buffer, drawbuffer, value)
}
/// Fallbacks: UniformMatrix4x2fvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UniformMatrix4x2fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::UniformMatrix4x2fv.f)(location, count, transpose, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn StencilMask(mask: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::StencilMask.f)(mask)
}
/// Fallbacks: UniformMatrix4fvARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UniformMatrix4fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::UniformMatrix4fv.f)(location, count, transpose, value)
}
/// Fallbacks: PolygonModeNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn PolygonMode(face: types::GLenum, mode: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(storage::PolygonMode.f)(face, mode)
}
/// Fallbacks: CompressedTexSubImage3DARB, CompressedTexSubImage3DOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CompressedTexSubImage3D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, types::GLenum, types::GLsizei, *const c_void) -> ()>(storage::CompressedTexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribP4ui(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLboolean, types::GLuint) -> ()>(storage::VertexAttribP4ui.f)(index, type_, normalized, value)
}
/// Fallbacks: VertexAttribIPointerEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribIPointer(index: types::GLuint, size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLsizei, *const c_void) -> ()>(storage::VertexAttribIPointer.f)(index, size, type_, stride, pointer)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn NamedFramebufferTextureLayer(framebuffer: types::GLuint, attachment: types::GLenum, texture: types::GLuint, level: types::GLint, layer: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, types::GLint, types::GLint) -> ()>(storage::NamedFramebufferTextureLayer.f)(framebuffer, attachment, texture, level, layer)
}
/// Fallbacks: DeleteFramebuffersEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DeleteFramebuffers(n: types::GLsizei, framebuffers: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteFramebuffers.f)(n, framebuffers)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Disable(cap: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::Disable.f)(cap)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetShaderInfoLog(shader: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, infoLog: *mut types::GLchar) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(storage::GetShaderInfoLog.f)(shader, bufSize, length, infoLog)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform3d(location: types::GLint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::Uniform3d.f)(location, x, y, z)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CopyTextureSubImage3D(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::CopyTextureSubImage3D.f)(texture, level, xoffset, yoffset, zoffset, x, y, width, height)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn InvalidateBufferData(buffer: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::InvalidateBufferData.f)(buffer)
}
/// Fallbacks: EndConditionalRenderNV, EndConditionalRenderNVX
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn EndConditionalRender() -> () {
    mem::transmute::<_, extern "system" fn() -> ()>(storage::EndConditionalRender.f)()
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ReleaseShaderCompiler() -> () {
    mem::transmute::<_, extern "system" fn() -> ()>(storage::ReleaseShaderCompiler.f)()
}
/// Fallbacks: NamedBufferSubDataEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn NamedBufferSubData(buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr, data: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLintptr, types::GLsizeiptr, *const c_void) -> ()>(storage::NamedBufferSubData.f)(buffer, offset, size, data)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetnPixelMapfv(map: types::GLenum, bufSize: types::GLsizei, values: *mut types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *mut types::GLfloat) -> ()>(storage::GetnPixelMapfv.f)(map, bufSize, values)
}
/// Fallbacks: UniformMatrix3x2fvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UniformMatrix3x2fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::UniformMatrix3x2fv.f)(location, count, transpose, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CopyNamedBufferSubData(readBuffer: types::GLuint, writeBuffer: types::GLuint, readOffset: types::GLintptr, writeOffset: types::GLintptr, size: types::GLsizeiptr) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLintptr, types::GLintptr, types::GLsizeiptr) -> ()>(storage::CopyNamedBufferSubData.f)(readBuffer, writeBuffer, readOffset, writeOffset, size)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniformMatrix4x2dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(storage::ProgramUniformMatrix4x2dv.f)(program, location, count, transpose, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetDoublev(pname: types::GLenum, data: *mut types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLdouble) -> ()>(storage::GetDoublev.f)(pname, data)
}
/// Fallbacks: DisableVertexAttribArrayARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DisableVertexAttribArray(index: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::DisableVertexAttribArray.f)(index)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindBuffersRange(target: types::GLenum, first: types::GLuint, count: types::GLsizei, buffers: *const types::GLuint, offsets: *const types::GLintptr, sizes: *const types::GLsizeiptr) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei, *const types::GLuint, *const types::GLintptr, *const types::GLsizeiptr) -> ()>(storage::BindBuffersRange.f)(target, first, count, buffers, offsets, sizes)
}
/// Fallbacks: ProgramUniform4uivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform4uiv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLuint) -> ()>(storage::ProgramUniform4uiv.f)(program, location, count, value)
}
/// Fallbacks: ActiveTextureARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ActiveTexture(texture: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::ActiveTexture.f)(texture)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetProgramiv(program: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetProgramiv.f)(program, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribIFormat(attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, relativeoffset: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLuint) -> ()>(storage::VertexAttribIFormat.f)(attribindex, size, type_, relativeoffset)
}
/// Fallbacks: CopyTexSubImage3DEXT, CopyTexSubImage3DOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CopyTexSubImage3D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::CopyTexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, x, y, width, height)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetActiveAtomicCounterBufferiv(program: types::GLuint, bufferIndex: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetActiveAtomicCounterBufferiv.f)(program, bufferIndex, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DrawElementsIndirect(mode: types::GLenum, type_: types::GLenum, indirect: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const c_void) -> ()>(storage::DrawElementsIndirect.f)(mode, type_, indirect)
}
/// Fallbacks: ViewportIndexedfNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ViewportIndexedf(index: types::GLuint, x: types::GLfloat, y: types::GLfloat, w: types::GLfloat, h: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::ViewportIndexedf.f)(index, x, y, w, h)
}
/// Fallbacks: VertexAttrib4ubvARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib4ubv(index: types::GLuint, v: *const types::GLubyte) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLubyte) -> ()>(storage::VertexAttrib4ubv.f)(index, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ClearBufferfi(buffer: types::GLenum, drawbuffer: types::GLint, depth: types::GLfloat, stencil: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLfloat, types::GLint) -> ()>(storage::ClearBufferfi.f)(buffer, drawbuffer, depth, stencil)
}
/// Fallbacks: VertexAttribI1uivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribI1uiv(index: types::GLuint, v: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLuint) -> ()>(storage::VertexAttribI1uiv.f)(index, v)
}
/// Fallbacks: AttachObjectARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn AttachShader(program: types::GLuint, shader: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(storage::AttachShader.f)(program, shader)
}
/// Fallbacks: VertexAttrib3svARB, VertexAttrib3svNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib3sv(index: types::GLuint, v: *const types::GLshort) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLshort) -> ()>(storage::VertexAttrib3sv.f)(index, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindTransformFeedback(target: types::GLenum, id: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::BindTransformFeedback.f)(target, id)
}
/// Fallbacks: ProgramUniform3iEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform3i(program: types::GLuint, location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(storage::ProgramUniform3i.f)(program, location, v0, v1, v2)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ClearBufferiv(buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, *const types::GLint) -> ()>(storage::ClearBufferiv.f)(buffer, drawbuffer, value)
}
/// Fallbacks: ProgramUniform3ivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform3iv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLint) -> ()>(storage::ProgramUniform3iv.f)(program, location, count, value)
}
/// Fallbacks: GetCompressedTexImageARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetCompressedTexImage(target: types::GLenum, level: types::GLint, img: *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, *mut c_void) -> ()>(storage::GetCompressedTexImage.f)(target, level, img)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetQueryBufferObjecti64v(id: types::GLuint, buffer: types::GLuint, pname: types::GLenum, offset: types::GLintptr) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLenum, types::GLintptr) -> ()>(storage::GetQueryBufferObjecti64v.f)(id, buffer, pname, offset)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform4dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(storage::ProgramUniform4dv.f)(program, location, count, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexArrayVertexBuffer(vaobj: types::GLuint, bindingindex: types::GLuint, buffer: types::GLuint, offset: types::GLintptr, stride: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint, types::GLintptr, types::GLsizei) -> ()>(storage::VertexArrayVertexBuffer.f)(vaobj, bindingindex, buffer, offset, stride)
}
/// Fallbacks: Uniform2fARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform2f(location: types::GLint, v0: types::GLfloat, v1: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat, types::GLfloat) -> ()>(storage::Uniform2f.f)(location, v0, v1)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetNamedRenderbufferParameteriv(renderbuffer: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetNamedRenderbufferParameteriv.f)(renderbuffer, pname, params)
}
/// Fallbacks: VertexAttrib2svARB, VertexAttrib2svNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib2sv(index: types::GLuint, v: *const types::GLshort) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLshort) -> ()>(storage::VertexAttrib2sv.f)(index, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetTextureSubImage(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, pixels: *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, types::GLsizei, *mut c_void) -> ()>(storage::GetTextureSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, bufSize, pixels)
}
/// Fallbacks: VertexAttribI3uiEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribI3ui(index: types::GLuint, x: types::GLuint, y: types::GLuint, z: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::VertexAttribI3ui.f)(index, x, y, z)
}
/// Fallbacks: GetQueryivARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetQueryiv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetQueryiv.f)(target, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn MemoryBarrierByRegion(barriers: types::GLbitfield) -> () {
    mem::transmute::<_, extern "system" fn(types::GLbitfield) -> ()>(storage::MemoryBarrierByRegion.f)(barriers)
}
/// Fallbacks: ProgramUniformMatrix3fvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniformMatrix3fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::ProgramUniformMatrix3fv.f)(program, location, count, transpose, value)
}
/// Fallbacks: VertexAttrib1svARB, VertexAttrib1svNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib1sv(index: types::GLuint, v: *const types::GLshort) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLshort) -> ()>(storage::VertexAttrib1sv.f)(index, v)
}
/// Fallbacks: BindTextureEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindTexture(target: types::GLenum, texture: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::BindTexture.f)(target, texture)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TextureBufferRange(texture: types::GLuint, internalformat: types::GLenum, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, types::GLintptr, types::GLsizeiptr) -> ()>(storage::TextureBufferRange.f)(texture, internalformat, buffer, offset, size)
}
/// Fallbacks: Uniform4fARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform4f(location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat, v3: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::Uniform4f.f)(location, v0, v1, v2, v3)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ClearDepth(depth: types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLdouble) -> ()>(storage::ClearDepth.f)(depth)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn FrontFace(mode: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::FrontFace.f)(mode)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetTextureParameterfv(texture: types::GLuint, pname: types::GLenum, params: *mut types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLfloat) -> ()>(storage::GetTextureParameterfv.f)(texture, pname, params)
}
/// Fallbacks: MemoryBarrierEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn MemoryBarrier(barriers: types::GLbitfield) -> () {
    mem::transmute::<_, extern "system" fn(types::GLbitfield) -> ()>(storage::MemoryBarrier.f)(barriers)
}
/// Fallbacks: ViewportArrayvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ViewportArrayv(first: types::GLuint, count: types::GLsizei, v: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLfloat) -> ()>(storage::ViewportArrayv.f)(first, count, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BeginQueryIndexed(target: types::GLenum, index: types::GLuint, id: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLuint) -> ()>(storage::BeginQueryIndexed.f)(target, index, id)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn PatchParameterfv(pname: types::GLenum, values: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfloat) -> ()>(storage::PatchParameterfv.f)(pname, values)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindTextures(first: types::GLuint, count: types::GLsizei, textures: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLuint) -> ()>(storage::BindTextures.f)(first, count, textures)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetProgramPipelineInfoLog(pipeline: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, infoLog: *mut types::GLchar) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(storage::GetProgramPipelineInfoLog.f)(pipeline, bufSize, length, infoLog)
}
/// Fallbacks: GetUniformuivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetUniformuiv(program: types::GLuint, location: types::GLint, params: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, *mut types::GLuint) -> ()>(storage::GetUniformuiv.f)(program, location, params)
}
/// Fallbacks: MultiDrawArraysEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn MultiDrawArrays(mode: types::GLenum, first: *const types::GLint, count: *const types::GLsizei, drawcount: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLint, *const types::GLsizei, types::GLsizei) -> ()>(storage::MultiDrawArrays.f)(mode, first, count, drawcount)
}
/// Fallbacks: ProgramUniform1uiEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform1ui(program: types::GLuint, location: types::GLint, v0: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLuint) -> ()>(storage::ProgramUniform1ui.f)(program, location, v0)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetStringi(name: types::GLenum, index: types::GLuint) -> *const types::GLubyte {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> *const types::GLubyte>(storage::GetStringi.f)(name, index)
}
/// Fallbacks: GetShaderSourceARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetShaderSource(shader: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, source: *mut types::GLchar) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(storage::GetShaderSource.f)(shader, bufSize, length, source)
}
/// Fallbacks: MapBufferRangeEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn MapBufferRange(target: types::GLenum, offset: types::GLintptr, length: types::GLsizeiptr, access: types::GLbitfield) -> *mut c_void {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLintptr, types::GLsizeiptr, types::GLbitfield) -> *mut c_void>(storage::MapBufferRange.f)(target, offset, length, access)
}
/// Fallbacks: VertexAttrib4NuivARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib4Nuiv(index: types::GLuint, v: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLuint) -> ()>(storage::VertexAttrib4Nuiv.f)(index, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ClearColor(red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::ClearColor.f)(red, green, blue, alpha)
}
/// Fallbacks: Uniform3uiEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform3ui(location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::Uniform3ui.f)(location, v0, v1, v2)
}
/// Fallbacks: CreateProgramObjectARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CreateProgram() -> types::GLuint {
    mem::transmute::<_, extern "system" fn() -> types::GLuint>(storage::CreateProgram.f)()
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn IsProgramPipeline(pipeline: types::GLuint) -> types::GLboolean {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsProgramPipeline.f)(pipeline)
}
/// Fallbacks: Uniform3fARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform3f(location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::Uniform3f.f)(location, v0, v1, v2)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CreateQueries(target: types::GLenum, n: types::GLsizei, ids: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *mut types::GLuint) -> ()>(storage::CreateQueries.f)(target, n, ids)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetNamedBufferParameteriv(buffer: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetNamedBufferParameteriv.f)(buffer, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetShaderiv(shader: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetShaderiv.f)(shader, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn PointSize(size: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(storage::PointSize.f)(size)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DrawTransformFeedbackInstanced(mode: types::GLenum, id: types::GLuint, instancecount: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei) -> ()>(storage::DrawTransformFeedbackInstanced.f)(mode, id, instancecount)
}
/// Fallbacks: IsVertexArrayAPPLE, IsVertexArrayOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn IsVertexArray(array: types::GLuint) -> types::GLboolean {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsVertexArray.f)(array)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetCompressedTextureSubImage(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, bufSize: types::GLsizei, pixels: *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, types::GLsizei, *mut c_void) -> ()>(storage::GetCompressedTextureSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, bufSize, pixels)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetnPixelMapusv(map: types::GLenum, bufSize: types::GLsizei, values: *mut types::GLushort) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *mut types::GLushort) -> ()>(storage::GetnPixelMapusv.f)(map, bufSize, values)
}
/// Fallbacks: BeginTransformFeedbackEXT, BeginTransformFeedbackNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BeginTransformFeedback(primitiveMode: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::BeginTransformFeedback.f)(primitiveMode)
}
/// Fallbacks: GetGraphicsResetStatusKHR
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetGraphicsResetStatus() -> types::GLenum {
    mem::transmute::<_, extern "system" fn() -> types::GLenum>(storage::GetGraphicsResetStatus.f)()
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Clear(mask: types::GLbitfield) -> () {
    mem::transmute::<_, extern "system" fn(types::GLbitfield) -> ()>(storage::Clear.f)(mask)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ColorP3ui(type_: types::GLenum, color: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::ColorP3ui.f)(type_, color)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CreateBuffers(n: types::GLsizei, buffers: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::CreateBuffers.f)(n, buffers)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexParameteri(target: types::GLenum, pname: types::GLenum, param: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint) -> ()>(storage::TexParameteri.f)(target, pname, param)
}
/// Fallbacks: Uniform2iARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform2i(location: types::GLint, v0: types::GLint, v1: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint) -> ()>(storage::Uniform2i.f)(location, v0, v1)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn IsShader(shader: types::GLuint) -> types::GLboolean {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsShader.f)(shader)
}
/// Fallbacks: GetBufferParameterivARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetBufferParameteriv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetBufferParameteriv.f)(target, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetCompressedTextureImage(texture: types::GLuint, level: types::GLint, bufSize: types::GLsizei, pixels: *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *mut c_void) -> ()>(storage::GetCompressedTextureImage.f)(texture, level, bufSize, pixels)
}
/// Fallbacks: Uniform1fARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform1f(location: types::GLint, v0: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat) -> ()>(storage::Uniform1f.f)(location, v0)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ClearNamedFramebufferuiv(framebuffer: types::GLuint, buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLint, *const types::GLuint) -> ()>(storage::ClearNamedFramebufferuiv.f)(framebuffer, buffer, drawbuffer, value)
}
/// Fallbacks: BlendEquationIndexedAMD, BlendEquationiARB, BlendEquationiEXT, BlendEquationiOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BlendEquationi(buf: types::GLuint, mode: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum) -> ()>(storage::BlendEquationi.f)(buf, mode)
}
/// Fallbacks: CopyBufferSubDataNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CopyBufferSubData(readTarget: types::GLenum, writeTarget: types::GLenum, readOffset: types::GLintptr, writeOffset: types::GLintptr, size: types::GLsizeiptr) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLintptr, types::GLintptr, types::GLsizeiptr) -> ()>(storage::CopyBufferSubData.f)(readTarget, writeTarget, readOffset, writeOffset, size)
}
/// Fallbacks: PointParameterivNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn PointParameteriv(pname: types::GLenum, params: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLint) -> ()>(storage::PointParameteriv.f)(pname, params)
}
/// Fallbacks: GetnUniformivKHR
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetnUniformiv(program: types::GLuint, location: types::GLint, bufSize: types::GLsizei, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *mut types::GLint) -> ()>(storage::GetnUniformiv.f)(program, location, bufSize, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetActiveUniformsiv(program: types::GLuint, uniformCount: types::GLsizei, uniformIndices: *const types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetActiveUniformsiv.f)(program, uniformCount, uniformIndices, pname, params)
}
/// Fallbacks: BindBufferARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindBuffer(target: types::GLenum, buffer: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::BindBuffer.f)(target, buffer)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DeleteProgram(program: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::DeleteProgram.f)(program)
}
/// Fallbacks: VertexAttrib2dvARB, VertexAttrib2dvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib2dv(index: types::GLuint, v: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(storage::VertexAttrib2dv.f)(index, v)
}
/// Fallbacks: ProgramUniformMatrix2x3fvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniformMatrix2x3fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::ProgramUniformMatrix2x3fv.f)(program, location, count, transpose, value)
}
/// Fallbacks: BindAttribLocationARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindAttribLocation(program: types::GLuint, index: types::GLuint, name: *const types::GLchar) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, *const types::GLchar) -> ()>(storage::BindAttribLocation.f)(program, index, name)
}
/// Fallbacks: ProvokingVertexEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProvokingVertex(mode: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::ProvokingVertex.f)(mode)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetTransformFeedbacki_v(xfb: types::GLuint, pname: types::GLenum, index: types::GLuint, param: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, *mut types::GLint) -> ()>(storage::GetTransformFeedbacki_v.f)(xfb, pname, index, param)
}
/// Fallbacks: ProgramUniform4fEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform4f(program: types::GLuint, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat, v3: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::ProgramUniform4f.f)(program, location, v0, v1, v2, v3)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CompressedTextureSubImage1D(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, width: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLsizei, types::GLenum, types::GLsizei, *const c_void) -> ()>(storage::CompressedTextureSubImage1D.f)(texture, level, xoffset, width, format, imageSize, data)
}
/// Fallbacks: TexStorage1DEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexStorage1D(target: types::GLenum, levels: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, types::GLsizei) -> ()>(storage::TexStorage1D.f)(target, levels, internalformat, width)
}
/// Fallbacks: VertexAttribI4usvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribI4usv(index: types::GLuint, v: *const types::GLushort) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLushort) -> ()>(storage::VertexAttribI4usv.f)(index, v)
}
/// Fallbacks: IsRenderbufferEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn IsRenderbuffer(renderbuffer: types::GLuint) -> types::GLboolean {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsRenderbuffer.f)(renderbuffer)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribP1ui(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLboolean, types::GLuint) -> ()>(storage::VertexAttribP1ui.f)(index, type_, normalized, value)
}
/// Fallbacks: Uniform3uivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform3uiv(location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLuint) -> ()>(storage::Uniform3uiv.f)(location, count, value)
}
/// Fallbacks: ProgramUniformMatrix4x3fvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniformMatrix4x3fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::ProgramUniformMatrix4x3fv.f)(program, location, count, transpose, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetUniformIndices(program: types::GLuint, uniformCount: types::GLsizei, uniformNames: *const *const types::GLchar, uniformIndices: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const *const types::GLchar, *mut types::GLuint) -> ()>(storage::GetUniformIndices.f)(program, uniformCount, uniformNames, uniformIndices)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GenSamplers(count: types::GLsizei, samplers: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenSamplers.f)(count, samplers)
}
/// Fallbacks: ProgramUniformMatrix4fvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniformMatrix4fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::ProgramUniformMatrix4fv.f)(program, location, count, transpose, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexArrayBindingDivisor(vaobj: types::GLuint, bindingindex: types::GLuint, divisor: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::VertexArrayBindingDivisor.f)(vaobj, bindingindex, divisor)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexP2uiv(type_: types::GLenum, value: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(storage::VertexP2uiv.f)(type_, value)
}
/// Fallbacks: VertexAttrib4sARB, VertexAttrib4sNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib4s(index: types::GLuint, x: types::GLshort, y: types::GLshort, z: types::GLshort, w: types::GLshort) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLshort, types::GLshort, types::GLshort, types::GLshort) -> ()>(storage::VertexAttrib4s.f)(index, x, y, z, w)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DeleteTextures(n: types::GLsizei, textures: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteTextures.f)(n, textures)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindImageTextures(first: types::GLuint, count: types::GLsizei, textures: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLuint) -> ()>(storage::BindImageTextures.f)(first, count, textures)
}
/// Fallbacks: WaitSyncAPPLE
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn WaitSync(sync: types::GLsync, flags: types::GLbitfield, timeout: types::GLuint64) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsync, types::GLbitfield, types::GLuint64) -> ()>(storage::WaitSync.f)(sync, flags, timeout)
}
/// Fallbacks: BindVertexArrayOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindVertexArray(array: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::BindVertexArray.f)(array)
}
/// Fallbacks: GetActiveAttribARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetActiveAttrib(program: types::GLuint, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, size: *mut types::GLint, type_: *mut types::GLenum, name: *mut types::GLchar) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLint, *mut types::GLenum, *mut types::GLchar) -> ()>(storage::GetActiveAttrib.f)(program, index, bufSize, length, size, type_, name)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TextureStorage2DMultisample(texture: types::GLuint, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, fixedsamplelocations: types::GLboolean) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei, types::GLboolean) -> ()>(storage::TextureStorage2DMultisample.f)(texture, samples, internalformat, width, height, fixedsamplelocations)
}
/// Fallbacks: DebugMessageInsertARB, DebugMessageInsertKHR
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DebugMessageInsert(source: types::GLenum, type_: types::GLenum, id: types::GLuint, severity: types::GLenum, length: types::GLsizei, buf: *const types::GLchar) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint, types::GLenum, types::GLsizei, *const types::GLchar) -> ()>(storage::DebugMessageInsert.f)(source, type_, id, severity, length, buf)
}
/// Fallbacks: DeleteTransformFeedbacksNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DeleteTransformFeedbacks(n: types::GLsizei, ids: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteTransformFeedbacks.f)(n, ids)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TextureSubImage1D(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, width: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLsizei, types::GLenum, types::GLenum, *const c_void) -> ()>(storage::TextureSubImage1D.f)(texture, level, xoffset, width, format, type_, pixels)
}
/// Fallbacks: VertexAttribL1dvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribL1dv(index: types::GLuint, v: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(storage::VertexAttribL1dv.f)(index, v)
}
/// Fallbacks: VertexAttrib1fvARB, VertexAttrib1fvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib1fv(index: types::GLuint, v: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(storage::VertexAttrib1fv.f)(index, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetBufferParameteri64v(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint64) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint64) -> ()>(storage::GetBufferParameteri64v.f)(target, pname, params)
}
/// Fallbacks: DeleteRenderbuffersEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DeleteRenderbuffers(n: types::GLsizei, renderbuffers: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteRenderbuffers.f)(n, renderbuffers)
}
/// Fallbacks: GetRenderbufferParameterivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetRenderbufferParameteriv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetRenderbufferParameteriv.f)(target, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TextureParameterfv(texture: types::GLuint, pname: types::GLenum, param: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLfloat) -> ()>(storage::TextureParameterfv.f)(texture, pname, param)
}
/// Fallbacks: TexBufferRangeEXT, TexBufferRangeOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexBufferRange(target: types::GLenum, internalformat: types::GLenum, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint, types::GLintptr, types::GLsizeiptr) -> ()>(storage::TexBufferRange.f)(target, internalformat, buffer, offset, size)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn NamedBufferData(buffer: types::GLuint, size: types::GLsizeiptr, data: *const c_void, usage: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizeiptr, *const c_void, types::GLenum) -> ()>(storage::NamedBufferData.f)(buffer, size, data, usage)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn PixelStorei(pname: types::GLenum, param: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(storage::PixelStorei.f)(pname, param)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetActiveSubroutineUniformName(program: types::GLuint, shadertype: types::GLenum, index: types::GLuint, bufsize: types::GLsizei, length: *mut types::GLsizei, name: *mut types::GLchar) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(storage::GetActiveSubroutineUniformName.f)(program, shadertype, index, bufsize, length, name)
}
/// Fallbacks: BlendEquationEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BlendEquation(mode: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::BlendEquation.f)(mode)
}
/// Fallbacks: BufferDataARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BufferData(target: types::GLenum, size: types::GLsizeiptr, data: *const c_void, usage: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizeiptr, *const c_void, types::GLenum) -> ()>(storage::BufferData.f)(target, size, data, usage)
}
/// Fallbacks: CompressedTexSubImage2DARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CompressedTexSubImage2D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLsizei, *const c_void) -> ()>(storage::CompressedTexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, imageSize, data)
}
/// Fallbacks: FramebufferTexture3DEXT, FramebufferTexture3DOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn FramebufferTexture3D(target: types::GLenum, attachment: types::GLenum, textarget: types::GLenum, texture: types::GLuint, level: types::GLint, zoffset: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLuint, types::GLint, types::GLint) -> ()>(storage::FramebufferTexture3D.f)(target, attachment, textarget, texture, level, zoffset)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniformMatrix4x3dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(storage::ProgramUniformMatrix4x3dv.f)(program, location, count, transpose, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetnCompressedTexImage(target: types::GLenum, lod: types::GLint, bufSize: types::GLsizei, pixels: *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLsizei, *mut c_void) -> ()>(storage::GetnCompressedTexImage.f)(target, lod, bufSize, pixels)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetProgramStageiv(program: types::GLuint, shadertype: types::GLenum, pname: types::GLenum, values: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetProgramStageiv.f)(program, shadertype, pname, values)
}
/// Fallbacks: ClampColorARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ClampColor(target: types::GLenum, clamp: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(storage::ClampColor.f)(target, clamp)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ValidateProgramPipeline(pipeline: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::ValidateProgramPipeline.f)(pipeline)
}
/// Fallbacks: GetVertexAttribfvARB, GetVertexAttribfvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetVertexAttribfv(index: types::GLuint, pname: types::GLenum, params: *mut types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLfloat) -> ()>(storage::GetVertexAttribfv.f)(index, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniformMatrix2x4dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(storage::ProgramUniformMatrix2x4dv.f)(program, location, count, transpose, value)
}
/// Fallbacks: UniformMatrix4x3fvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UniformMatrix4x3fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::UniformMatrix4x3fv.f)(location, count, transpose, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn MultiTexCoordP2uiv(texture: types::GLenum, type_: types::GLenum, coords: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLuint) -> ()>(storage::MultiTexCoordP2uiv.f)(texture, type_, coords)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DeleteShader(shader: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::DeleteShader.f)(shader)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn NamedFramebufferRenderbuffer(framebuffer: types::GLuint, attachment: types::GLenum, renderbuffertarget: types::GLenum, renderbuffer: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum, types::GLuint) -> ()>(storage::NamedFramebufferRenderbuffer.f)(framebuffer, attachment, renderbuffertarget, renderbuffer)
}
/// Fallbacks: GetAttribLocationARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetAttribLocation(program: types::GLuint, name: *const types::GLchar) -> types::GLint {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLint>(storage::GetAttribLocation.f)(program, name)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetInteger64i_v(target: types::GLenum, index: types::GLuint, data: *mut types::GLint64) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, *mut types::GLint64) -> ()>(storage::GetInteger64i_v.f)(target, index, data)
}
/// Fallbacks: CopyTexImage1DEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CopyTexImage1D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, border: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLint) -> ()>(storage::CopyTexImage1D.f)(target, level, internalformat, x, y, width, border)
}
/// Fallbacks: VertexAttrib2fARB, VertexAttrib2fNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib2f(index: types::GLuint, x: types::GLfloat, y: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat, types::GLfloat) -> ()>(storage::VertexAttrib2f.f)(index, x, y)
}
/// Fallbacks: VertexAttribI4ivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribI4iv(index: types::GLuint, v: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(storage::VertexAttribI4iv.f)(index, v)
}
/// Fallbacks: ClearDepthfOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ClearDepthf(d: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(storage::ClearDepthf.f)(d)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UniformMatrix2x3dv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(storage::UniformMatrix2x3dv.f)(location, count, transpose, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetTexLevelParameteriv(target: types::GLenum, level: types::GLint, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, *mut types::GLint) -> ()>(storage::GetTexLevelParameteriv.f)(target, level, pname, params)
}
/// Fallbacks: ReadnPixelsARB, ReadnPixelsEXT, ReadnPixelsKHR
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ReadnPixels(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, data: *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, types::GLsizei, *mut c_void) -> ()>(storage::ReadnPixels.f)(x, y, width, height, format, type_, bufSize, data)
}
/// Fallbacks: LinkProgramARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn LinkProgram(program: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::LinkProgram.f)(program)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn EnableVertexArrayAttrib(vaobj: types::GLuint, index: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(storage::EnableVertexArrayAttrib.f)(vaobj, index)
}
/// Fallbacks: VertexAttribLPointerEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribLPointer(index: types::GLuint, size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLsizei, *const c_void) -> ()>(storage::VertexAttribLPointer.f)(index, size, type_, stride, pointer)
}
/// Fallbacks: TextureViewEXT, TextureViewOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TextureView(texture: types::GLuint, target: types::GLenum, origtexture: types::GLuint, internalformat: types::GLenum, minlevel: types::GLuint, numlevels: types::GLuint, minlayer: types::GLuint, numlayers: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, types::GLenum, types::GLuint, types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::TextureView.f)(texture, target, origtexture, internalformat, minlevel, numlevels, minlayer, numlayers)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetActiveSubroutineUniformiv(program: types::GLuint, shadertype: types::GLenum, index: types::GLuint, pname: types::GLenum, values: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetActiveSubroutineUniformiv.f)(program, shadertype, index, pname, values)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetQueryBufferObjectui64v(id: types::GLuint, buffer: types::GLuint, pname: types::GLenum, offset: types::GLintptr) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLenum, types::GLintptr) -> ()>(storage::GetQueryBufferObjectui64v.f)(id, buffer, pname, offset)
}
/// Fallbacks: CompileShaderARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CompileShader(shader: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::CompileShader.f)(shader)
}
/// Fallbacks: Uniform2fvARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform2fv(location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(storage::Uniform2fv.f)(location, count, value)
}
/// Fallbacks: TexSubImage3DEXT, TexSubImage3DOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexSubImage3D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *const c_void) -> ()>(storage::TexSubImage3D.f)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexImage2DMultisample(target: types::GLenum, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, fixedsamplelocations: types::GLboolean) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei, types::GLboolean) -> ()>(storage::TexImage2DMultisample.f)(target, samples, internalformat, width, height, fixedsamplelocations)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform4d(location: types::GLint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble, w: types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::Uniform4d.f)(location, x, y, z, w)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetTransformFeedbacki64_v(xfb: types::GLuint, pname: types::GLenum, index: types::GLuint, param: *mut types::GLint64) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, *mut types::GLint64) -> ()>(storage::GetTransformFeedbacki64_v.f)(xfb, pname, index, param)
}
/// Fallbacks: ProgramUniformMatrix3x2fvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniformMatrix3x2fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::ProgramUniformMatrix3x2fv.f)(program, location, count, transpose, value)
}
/// Fallbacks: ProgramUniformMatrix2fvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniformMatrix2fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::ProgramUniformMatrix2fv.f)(program, location, count, transpose, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CreateVertexArrays(n: types::GLsizei, arrays: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::CreateVertexArrays.f)(n, arrays)
}
/// Fallbacks: BindBufferBaseEXT, BindBufferBaseNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindBufferBase(target: types::GLenum, index: types::GLuint, buffer: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLuint) -> ()>(storage::BindBufferBase.f)(target, index, buffer)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetSamplerParameteriv(sampler: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetSamplerParameteriv.f)(sampler, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ReadPixels(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *mut c_void) -> ()>(storage::ReadPixels.f)(x, y, width, height, format, type_, pixels)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribLFormat(attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, relativeoffset: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLuint) -> ()>(storage::VertexAttribLFormat.f)(attribindex, size, type_, relativeoffset)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetQueryBufferObjectuiv(id: types::GLuint, buffer: types::GLuint, pname: types::GLenum, offset: types::GLintptr) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLenum, types::GLintptr) -> ()>(storage::GetQueryBufferObjectuiv.f)(id, buffer, pname, offset)
}
/// Fallbacks: FramebufferTextureARB, FramebufferTextureEXT, FramebufferTextureOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn FramebufferTexture(target: types::GLenum, attachment: types::GLenum, texture: types::GLuint, level: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint, types::GLint) -> ()>(storage::FramebufferTexture.f)(target, attachment, texture, level)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexParameterf(target: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfloat) -> ()>(storage::TexParameterf.f)(target, pname, param)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn FramebufferParameteri(target: types::GLenum, pname: types::GLenum, param: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint) -> ()>(storage::FramebufferParameteri.f)(target, pname, param)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TextureParameterIiv(texture: types::GLuint, pname: types::GLenum, params: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLint) -> ()>(storage::TextureParameterIiv.f)(texture, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindBuffersBase(target: types::GLenum, first: types::GLuint, count: types::GLsizei, buffers: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei, *const types::GLuint) -> ()>(storage::BindBuffersBase.f)(target, first, count, buffers)
}
/// Fallbacks: TexStorage3DMultisampleOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexStorage3DMultisample(target: types::GLenum, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, fixedsamplelocations: types::GLboolean) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei, types::GLsizei, types::GLboolean) -> ()>(storage::TexStorage3DMultisample.f)(target, samples, internalformat, width, height, depth, fixedsamplelocations)
}
/// Fallbacks: VertexAttribI4iEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribI4i(index: types::GLuint, x: types::GLint, y: types::GLint, z: types::GLint, w: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(storage::VertexAttribI4i.f)(index, x, y, z, w)
}
/// Fallbacks: DrawRangeElementsEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DrawRangeElements(mode: types::GLenum, start: types::GLuint, end: types::GLuint, count: types::GLsizei, type_: types::GLenum, indices: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLuint, types::GLsizei, types::GLenum, *const c_void) -> ()>(storage::DrawRangeElements.f)(mode, start, end, count, type_, indices)
}
/// Fallbacks: TexImage3DEXT, TexImage3DOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexImage3D(target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, types::GLint, types::GLenum, types::GLenum, *const c_void) -> ()>(storage::TexImage3D.f)(target, level, internalformat, width, height, depth, border, format, type_, pixels)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TextureStorage2D(texture: types::GLuint, levels: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei) -> ()>(storage::TextureStorage2D.f)(texture, levels, internalformat, width, height)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TransformFeedbackBufferRange(xfb: types::GLuint, index: types::GLuint, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint, types::GLintptr, types::GLsizeiptr) -> ()>(storage::TransformFeedbackBufferRange.f)(xfb, index, buffer, offset, size)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexP4ui(type_: types::GLenum, value: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::VertexP4ui.f)(type_, value)
}
/// Fallbacks: BlendFuncSeparateEXT, BlendFuncSeparateINGR
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BlendFuncSeparate(sfactorRGB: types::GLenum, dfactorRGB: types::GLenum, sfactorAlpha: types::GLenum, dfactorAlpha: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLenum) -> ()>(storage::BlendFuncSeparate.f)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha)
}
/// Fallbacks: Uniform4fvARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform4fv(location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(storage::Uniform4fv.f)(location, count, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CreateShaderProgramv(type_: types::GLenum, count: types::GLsizei, strings: *const *const types::GLchar) -> types::GLuint {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const *const types::GLchar) -> types::GLuint>(storage::CreateShaderProgramv.f)(type_, count, strings)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindVertexBuffer(bindingindex: types::GLuint, buffer: types::GLuint, offset: types::GLintptr, stride: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLintptr, types::GLsizei) -> ()>(storage::BindVertexBuffer.f)(bindingindex, buffer, offset, stride)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexStorage2DMultisample(target: types::GLenum, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, fixedsamplelocations: types::GLboolean) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei, types::GLboolean) -> ()>(storage::TexStorage2DMultisample.f)(target, samples, internalformat, width, height, fixedsamplelocations)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ShaderStorageBlockBinding(program: types::GLuint, storageBlockIndex: types::GLuint, storageBlockBinding: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::ShaderStorageBlockBinding.f)(program, storageBlockIndex, storageBlockBinding)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn NamedRenderbufferStorageMultisample(renderbuffer: types::GLuint, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei) -> ()>(storage::NamedRenderbufferStorageMultisample.f)(renderbuffer, samples, internalformat, width, height)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetProgramResourceiv(program: types::GLuint, programInterface: types::GLenum, index: types::GLuint, propCount: types::GLsizei, props: *const types::GLenum, bufSize: types::GLsizei, length: *mut types::GLsizei, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, types::GLsizei, *const types::GLenum, types::GLsizei, *mut types::GLsizei, *mut types::GLint) -> ()>(storage::GetProgramResourceiv.f)(program, programInterface, index, propCount, props, bufSize, length, params)
}
/// Fallbacks: EnableVertexAttribArrayARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn EnableVertexAttribArray(index: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::EnableVertexAttribArray.f)(index)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexCoordP2ui(type_: types::GLenum, coords: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::TexCoordP2ui.f)(type_, coords)
}
/// Fallbacks: TexStorage2DEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexStorage2D(target: types::GLenum, levels: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei) -> ()>(storage::TexStorage2D.f)(target, levels, internalformat, width, height)
}
/// Fallbacks: VertexAttrib4NivARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib4Niv(index: types::GLuint, v: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(storage::VertexAttrib4Niv.f)(index, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexArrayVertexBuffers(vaobj: types::GLuint, first: types::GLuint, count: types::GLsizei, buffers: *const types::GLuint, offsets: *const types::GLintptr, strides: *const types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *const types::GLuint, *const types::GLintptr, *const types::GLsizei) -> ()>(storage::VertexArrayVertexBuffers.f)(vaobj, first, count, buffers, offsets, strides)
}
/// Fallbacks: ProgramUniform2ivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform2iv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLint) -> ()>(storage::ProgramUniform2iv.f)(program, location, count, value)
}
/// Fallbacks: UniformMatrix2fvARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UniformMatrix2fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::UniformMatrix2fv.f)(location, count, transpose, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetnMinmax(target: types::GLenum, reset: types::GLboolean, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, values: *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLboolean, types::GLenum, types::GLenum, types::GLsizei, *mut c_void) -> ()>(storage::GetnMinmax.f)(target, reset, format, type_, bufSize, values)
}
/// Fallbacks: UniformMatrix2x4fvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UniformMatrix2x4fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::UniformMatrix2x4fv.f)(location, count, transpose, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Finish() -> () {
    mem::transmute::<_, extern "system" fn() -> ()>(storage::Finish.f)()
}
/// Fallbacks: MultiDrawElementsIndirectAMD, MultiDrawElementsIndirectEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn MultiDrawElementsIndirect(mode: types::GLenum, type_: types::GLenum, indirect: *const c_void, drawcount: types::GLsizei, stride: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const c_void, types::GLsizei, types::GLsizei) -> ()>(storage::MultiDrawElementsIndirect.f)(mode, type_, indirect, drawcount, stride)
}
/// Fallbacks: DebugMessageCallbackARB, DebugMessageCallbackKHR
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DebugMessageCallback(callback: types::GLDEBUGPROC, userParam: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLDEBUGPROC, *const c_void) -> ()>(storage::DebugMessageCallback.f)(callback, userParam)
}
/// Fallbacks: GetnUniformfvKHR
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetnUniformfv(program: types::GLuint, location: types::GLint, bufSize: types::GLsizei, params: *mut types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *mut types::GLfloat) -> ()>(storage::GetnUniformfv.f)(program, location, bufSize, params)
}
/// Fallbacks: SamplerParameterIuivEXT, SamplerParameterIuivOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn SamplerParameterIuiv(sampler: types::GLuint, pname: types::GLenum, param: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLuint) -> ()>(storage::SamplerParameterIuiv.f)(sampler, pname, param)
}
/// Fallbacks: CopyTexImage2DEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CopyTexImage2D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, border: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLint) -> ()>(storage::CopyTexImage2D.f)(target, level, internalformat, x, y, width, height, border)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UniformMatrix2x4dv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(storage::UniformMatrix2x4dv.f)(location, count, transpose, value)
}
/// Fallbacks: FramebufferTexture2DEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn FramebufferTexture2D(target: types::GLenum, attachment: types::GLenum, textarget: types::GLenum, texture: types::GLuint, level: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLuint, types::GLint) -> ()>(storage::FramebufferTexture2D.f)(target, attachment, textarget, texture, level)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribFormat(attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, normalized: types::GLboolean, relativeoffset: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLboolean, types::GLuint) -> ()>(storage::VertexAttribFormat.f)(attribindex, size, type_, normalized, relativeoffset)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ClearNamedBufferData(buffer: types::GLuint, internalformat: types::GLenum, format: types::GLenum, type_: types::GLenum, data: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum, types::GLenum, *const c_void) -> ()>(storage::ClearNamedBufferData.f)(buffer, internalformat, format, type_, data)
}
/// Fallbacks: CheckFramebufferStatusEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CheckFramebufferStatus(target: types::GLenum) -> types::GLenum {
    mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLenum>(storage::CheckFramebufferStatus.f)(target)
}
/// Fallbacks: VertexAttribI2uivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribI2uiv(index: types::GLuint, v: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLuint) -> ()>(storage::VertexAttribI2uiv.f)(index, v)
}
/// Fallbacks: BufferStorageEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BufferStorage(target: types::GLenum, size: types::GLsizeiptr, data: *const c_void, flags: types::GLbitfield) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizeiptr, *const c_void, types::GLbitfield) -> ()>(storage::BufferStorage.f)(target, size, data, flags)
}
/// Fallbacks: PointParameterfARB, PointParameterfEXT, PointParameterfSGIS
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn PointParameterf(pname: types::GLenum, param: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(storage::PointParameterf.f)(pname, param)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetnColorTable(target: types::GLenum, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, table: *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLsizei, *mut c_void) -> ()>(storage::GetnColorTable.f)(target, format, type_, bufSize, table)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetnTexImage(target: types::GLenum, level: types::GLint, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, pixels: *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLenum, types::GLsizei, *mut c_void) -> ()>(storage::GetnTexImage.f)(target, level, format, type_, bufSize, pixels)
}
/// Fallbacks: DeleteQueriesARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DeleteQueries(n: types::GLsizei, ids: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteQueries.f)(n, ids)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CreateTransformFeedbacks(n: types::GLsizei, ids: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::CreateTransformFeedbacks.f)(n, ids)
}
/// Fallbacks: ProgramUniform3fvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform3fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(storage::ProgramUniform3fv.f)(program, location, count, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TransformFeedbackBufferBase(xfb: types::GLuint, index: types::GLuint, buffer: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::TransformFeedbackBufferBase.f)(xfb, index, buffer)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UnmapNamedBuffer(buffer: types::GLuint) -> types::GLboolean {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::UnmapNamedBuffer.f)(buffer)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetUniformdv(program: types::GLuint, location: types::GLint, params: *mut types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, *mut types::GLdouble) -> ()>(storage::GetUniformdv.f)(program, location, params)
}
/// Fallbacks: CompressedTexImage3DARB, CompressedTexImage3DOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CompressedTexImage3D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, border: types::GLint, imageSize: types::GLsizei, data: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLsizei, types::GLsizei, types::GLsizei, types::GLint, types::GLsizei, *const c_void) -> ()>(storage::CompressedTexImage3D.f)(target, level, internalformat, width, height, depth, border, imageSize, data)
}
/// Fallbacks: DrawElementsInstancedANGLE, DrawElementsInstancedARB, DrawElementsInstancedEXT, DrawElementsInstancedNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DrawElementsInstanced(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const c_void, instancecount: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, *const c_void, types::GLsizei) -> ()>(storage::DrawElementsInstanced.f)(mode, count, type_, indices, instancecount)
}
/// Fallbacks: GenQueriesARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GenQueries(n: types::GLsizei, ids: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenQueries.f)(n, ids)
}
/// Fallbacks: CopyTexSubImage2DEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CopyTexSubImage2D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::CopyTexSubImage2D.f)(target, level, xoffset, yoffset, x, y, width, height)
}
/// Fallbacks: DrawArraysInstancedBaseInstanceEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DrawArraysInstancedBaseInstance(mode: types::GLenum, first: types::GLint, count: types::GLsizei, instancecount: types::GLsizei, baseinstance: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLsizei, types::GLsizei, types::GLuint) -> ()>(storage::DrawArraysInstancedBaseInstance.f)(mode, first, count, instancecount, baseinstance)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexCoordP4ui(type_: types::GLenum, coords: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::TexCoordP4ui.f)(type_, coords)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribP2ui(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLboolean, types::GLuint) -> ()>(storage::VertexAttribP2ui.f)(index, type_, normalized, value)
}
/// Fallbacks: VertexAttrib4dvARB, VertexAttrib4dvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib4dv(index: types::GLuint, v: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(storage::VertexAttrib4dv.f)(index, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ColorP4uiv(type_: types::GLenum, color: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(storage::ColorP4uiv.f)(type_, color)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetActiveSubroutineName(program: types::GLuint, shadertype: types::GLenum, index: types::GLuint, bufsize: types::GLsizei, length: *mut types::GLsizei, name: *mut types::GLchar) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(storage::GetActiveSubroutineName.f)(program, shadertype, index, bufsize, length, name)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexCoordP4uiv(type_: types::GLenum, coords: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(storage::TexCoordP4uiv.f)(type_, coords)
}
/// Fallbacks: ProgramUniform3fEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform3f(program: types::GLuint, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::ProgramUniform3f.f)(program, location, v0, v1, v2)
}
/// Fallbacks: ProgramUniform1ivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform1iv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLint) -> ()>(storage::ProgramUniform1iv.f)(program, location, count, value)
}
/// Fallbacks: VertexAttrib1fARB, VertexAttrib1fNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib1f(index: types::GLuint, x: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat) -> ()>(storage::VertexAttrib1f.f)(index, x)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform1d(location: types::GLint, x: types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLdouble) -> ()>(storage::Uniform1d.f)(location, x)
}
/// Fallbacks: Uniform2ivARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform2iv(location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> ()>(storage::Uniform2iv.f)(location, count, value)
}
/// Fallbacks: CompressedTexImage2DARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CompressedTexImage2D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, border: types::GLint, imageSize: types::GLsizei, data: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLsizei, types::GLsizei, types::GLint, types::GLsizei, *const c_void) -> ()>(storage::CompressedTexImage2D.f)(target, level, internalformat, width, height, border, imageSize, data)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DrawBuffer(buf: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::DrawBuffer.f)(buf)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ClearNamedFramebufferiv(framebuffer: types::GLuint, buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLint, *const types::GLint) -> ()>(storage::ClearNamedFramebufferiv.f)(framebuffer, buffer, drawbuffer, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Hint(target: types::GLenum, mode: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(storage::Hint.f)(target, mode)
}
/// Fallbacks: DeleteBuffersARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DeleteBuffers(n: types::GLsizei, buffers: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteBuffers.f)(n, buffers)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexArrayAttribFormat(vaobj: types::GLuint, attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, normalized: types::GLboolean, relativeoffset: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLint, types::GLenum, types::GLboolean, types::GLuint) -> ()>(storage::VertexArrayAttribFormat.f)(vaobj, attribindex, size, type_, normalized, relativeoffset)
}
/// Fallbacks: GenTransformFeedbacksNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GenTransformFeedbacks(n: types::GLsizei, ids: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenTransformFeedbacks.f)(n, ids)
}
/// Fallbacks: IsBufferARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn IsBuffer(buffer: types::GLuint) -> types::GLboolean {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsBuffer.f)(buffer)
}
/// Fallbacks: DrawElementsInstancedBaseVertexEXT, DrawElementsInstancedBaseVertexOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DrawElementsInstancedBaseVertex(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const c_void, instancecount: types::GLsizei, basevertex: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, *const c_void, types::GLsizei, types::GLint) -> ()>(storage::DrawElementsInstancedBaseVertex.f)(mode, count, type_, indices, instancecount, basevertex)
}
/// Fallbacks: Uniform3iARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform3i(location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(storage::Uniform3i.f)(location, v0, v1, v2)
}
/// Fallbacks: GetProgramBinaryOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetProgramBinary(program: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, binaryFormat: *mut types::GLenum, binary: *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLenum, *mut c_void) -> ()>(storage::GetProgramBinary.f)(program, bufSize, length, binaryFormat, binary)
}
/// Fallbacks: GetVertexAttribPointervARB, GetVertexAttribPointervNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetVertexAttribPointerv(index: types::GLuint, pname: types::GLenum, pointer: *const *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const *mut c_void) -> ()>(storage::GetVertexAttribPointerv.f)(index, pname, pointer)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetActiveUniformBlockiv(program: types::GLuint, uniformBlockIndex: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetActiveUniformBlockiv.f)(program, uniformBlockIndex, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform3dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(storage::ProgramUniform3dv.f)(program, location, count, value)
}
/// Fallbacks: TexStorage3DEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexStorage3D(target: types::GLenum, levels: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei, types::GLsizei) -> ()>(storage::TexStorage3D.f)(target, levels, internalformat, width, height, depth)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetQueryBufferObjectiv(id: types::GLuint, buffer: types::GLuint, pname: types::GLenum, offset: types::GLintptr) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLenum, types::GLintptr) -> ()>(storage::GetQueryBufferObjectiv.f)(id, buffer, pname, offset)
}
/// Fallbacks: DepthRangefOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DepthRangef(n: types::GLfloat, f: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(storage::DepthRangef.f)(n, f)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DeleteProgramPipelines(n: types::GLsizei, pipelines: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteProgramPipelines.f)(n, pipelines)
}
/// Fallbacks: VertexAttrib4NusvARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib4Nusv(index: types::GLuint, v: *const types::GLushort) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLushort) -> ()>(storage::VertexAttrib4Nusv.f)(index, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ClearTexSubImage(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, type_: types::GLenum, data: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *const c_void) -> ()>(storage::ClearTexSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, data)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn MultiTexCoordP3ui(texture: types::GLenum, type_: types::GLenum, coords: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint) -> ()>(storage::MultiTexCoordP3ui.f)(texture, type_, coords)
}
/// Fallbacks: ProgramUniform2fEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform2f(program: types::GLuint, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLfloat, types::GLfloat) -> ()>(storage::ProgramUniform2f.f)(program, location, v0, v1)
}
/// Fallbacks: IsQueryARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn IsQuery(id: types::GLuint) -> types::GLboolean {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsQuery.f)(id)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetnSeparableFilter(target: types::GLenum, format: types::GLenum, type_: types::GLenum, rowBufSize: types::GLsizei, row: *mut c_void, columnBufSize: types::GLsizei, column: *mut c_void, span: *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLsizei, *mut c_void, types::GLsizei, *mut c_void, *mut c_void) -> ()>(storage::GetnSeparableFilter.f)(target, format, type_, rowBufSize, row, columnBufSize, column, span)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetProgramInfoLog(program: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, infoLog: *mut types::GLchar) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(storage::GetProgramInfoLog.f)(program, bufSize, length, infoLog)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindRenderbuffer(target: types::GLenum, renderbuffer: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::BindRenderbuffer.f)(target, renderbuffer)
}
/// Fallbacks: RenderbufferStorageEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn RenderbufferStorage(target: types::GLenum, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLsizei, types::GLsizei) -> ()>(storage::RenderbufferStorage.f)(target, internalformat, width, height)
}
/// Fallbacks: DebugMessageControlARB, DebugMessageControlKHR
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DebugMessageControl(source: types::GLenum, type_: types::GLenum, severity: types::GLenum, count: types::GLsizei, ids: *const types::GLuint, enabled: types::GLboolean) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLsizei, *const types::GLuint, types::GLboolean) -> ()>(storage::DebugMessageControl.f)(source, type_, severity, count, ids, enabled)
}
/// Fallbacks: GetnUniformuivKHR
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetnUniformuiv(program: types::GLuint, location: types::GLint, bufSize: types::GLsizei, params: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *mut types::GLuint) -> ()>(storage::GetnUniformuiv.f)(program, location, bufSize, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn PolygonOffset(factor: types::GLfloat, units: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(storage::PolygonOffset.f)(factor, units)
}
/// Fallbacks: MultiDrawElementsBaseVertexEXT, MultiDrawElementsBaseVertexOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn MultiDrawElementsBaseVertex(mode: types::GLenum, count: *const types::GLsizei, type_: types::GLenum, indices: *const *const c_void, drawcount: types::GLsizei, basevertex: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLsizei, types::GLenum, *const *const c_void, types::GLsizei, *const types::GLint) -> ()>(storage::MultiDrawElementsBaseVertex.f)(mode, count, type_, indices, drawcount, basevertex)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn NamedFramebufferDrawBuffer(framebuffer: types::GLuint, buf: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum) -> ()>(storage::NamedFramebufferDrawBuffer.f)(framebuffer, buf)
}
/// Fallbacks: VertexAttrib2dARB, VertexAttrib2dNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib2d(index: types::GLuint, x: types::GLdouble, y: types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble) -> ()>(storage::VertexAttrib2d.f)(index, x, y)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CreateTextures(target: types::GLenum, n: types::GLsizei, textures: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *mut types::GLuint) -> ()>(storage::CreateTextures.f)(target, n, textures)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetUniformSubroutineuiv(shadertype: types::GLenum, location: types::GLint, params: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, *mut types::GLuint) -> ()>(storage::GetUniformSubroutineuiv.f)(shadertype, location, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ClearNamedFramebufferfv(framebuffer: types::GLuint, buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLint, *const types::GLfloat) -> ()>(storage::ClearNamedFramebufferfv.f)(framebuffer, buffer, drawbuffer, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CreateRenderbuffers(n: types::GLsizei, renderbuffers: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::CreateRenderbuffers.f)(n, renderbuffers)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn IsSampler(sampler: types::GLuint) -> types::GLboolean {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsSampler.f)(sampler)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn MultiTexCoordP4uiv(texture: types::GLenum, type_: types::GLenum, coords: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLuint) -> ()>(storage::MultiTexCoordP4uiv.f)(texture, type_, coords)
}
/// Fallbacks: GetSyncivAPPLE
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetSynciv(sync: types::GLsync, pname: types::GLenum, bufSize: types::GLsizei, length: *mut types::GLsizei, values: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsync, types::GLenum, types::GLsizei, *mut types::GLsizei, *mut types::GLint) -> ()>(storage::GetSynciv.f)(sync, pname, bufSize, length, values)
}
/// Fallbacks: UnmapBufferARB, UnmapBufferOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UnmapBuffer(target: types::GLenum) -> types::GLboolean {
    mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLboolean>(storage::UnmapBuffer.f)(target)
}
/// Fallbacks: GetBufferPointervARB, GetBufferPointervOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetBufferPointerv(target: types::GLenum, pname: types::GLenum, params: *const *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const *mut c_void) -> ()>(storage::GetBufferPointerv.f)(target, pname, params)
}
/// Fallbacks: GenVertexArraysAPPLE, GenVertexArraysOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GenVertexArrays(n: types::GLsizei, arrays: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenVertexArrays.f)(n, arrays)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn SampleMaski(maskNumber: types::GLuint, mask: types::GLbitfield) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLbitfield) -> ()>(storage::SampleMaski.f)(maskNumber, mask)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ClearStencil(s: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint) -> ()>(storage::ClearStencil.f)(s)
}
/// Fallbacks: BlendFuncSeparateIndexedAMD, BlendFuncSeparateiARB, BlendFuncSeparateiEXT, BlendFuncSeparateiOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BlendFuncSeparatei(buf: types::GLuint, srcRGB: types::GLenum, dstRGB: types::GLenum, srcAlpha: types::GLenum, dstAlpha: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum, types::GLenum, types::GLenum) -> ()>(storage::BlendFuncSeparatei.f)(buf, srcRGB, dstRGB, srcAlpha, dstAlpha)
}
/// Fallbacks: VertexAttrib4NubARB, VertexAttrib4ubNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib4Nub(index: types::GLuint, x: types::GLubyte, y: types::GLubyte, z: types::GLubyte, w: types::GLubyte) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLubyte, types::GLubyte, types::GLubyte, types::GLubyte) -> ()>(storage::VertexAttrib4Nub.f)(index, x, y, z, w)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ShaderBinary(count: types::GLsizei, shaders: *const types::GLuint, binaryformat: types::GLenum, binary: *const c_void, length: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint, types::GLenum, *const c_void, types::GLsizei) -> ()>(storage::ShaderBinary.f)(count, shaders, binaryformat, binary, length)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TextureSubImage3D(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *const c_void) -> ()>(storage::TextureSubImage3D.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels)
}
/// Fallbacks: GetUniformivARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetUniformiv(program: types::GLuint, location: types::GLint, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, *mut types::GLint) -> ()>(storage::GetUniformiv.f)(program, location, params)
}
/// Fallbacks: Uniform1uivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform1uiv(location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLuint) -> ()>(storage::Uniform1uiv.f)(location, count, value)
}
/// Fallbacks: VertexAttribI4svEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribI4sv(index: types::GLuint, v: *const types::GLshort) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLshort) -> ()>(storage::VertexAttribI4sv.f)(index, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BlitNamedFramebuffer(readFramebuffer: types::GLuint, drawFramebuffer: types::GLuint, srcX0: types::GLint, srcY0: types::GLint, srcX1: types::GLint, srcY1: types::GLint, dstX0: types::GLint, dstY0: types::GLint, dstX1: types::GLint, dstY1: types::GLint, mask: types::GLbitfield, filter: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLbitfield, types::GLenum) -> ()>(storage::BlitNamedFramebuffer.f)(readFramebuffer, drawFramebuffer, srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetAttachedShaders(program: types::GLuint, maxCount: types::GLsizei, count: *mut types::GLsizei, shaders: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLuint) -> ()>(storage::GetAttachedShaders.f)(program, maxCount, count, shaders)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn InvalidateBufferSubData(buffer: types::GLuint, offset: types::GLintptr, length: types::GLsizeiptr) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLintptr, types::GLsizeiptr) -> ()>(storage::InvalidateBufferSubData.f)(buffer, offset, length)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn InvalidateFramebuffer(target: types::GLenum, numAttachments: types::GLsizei, attachments: *const types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, *const types::GLenum) -> ()>(storage::InvalidateFramebuffer.f)(target, numAttachments, attachments)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TextureStorage1D(texture: types::GLuint, levels: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, types::GLenum, types::GLsizei) -> ()>(storage::TextureStorage1D.f)(texture, levels, internalformat, width)
}
/// Fallbacks: FramebufferTexture1DEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn FramebufferTexture1D(target: types::GLenum, attachment: types::GLenum, textarget: types::GLenum, texture: types::GLuint, level: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLuint, types::GLint) -> ()>(storage::FramebufferTexture1D.f)(target, attachment, textarget, texture, level)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetnMapiv(target: types::GLenum, query: types::GLenum, bufSize: types::GLsizei, v: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLsizei, *mut types::GLint) -> ()>(storage::GetnMapiv.f)(target, query, bufSize, v)
}
/// Fallbacks: GetQueryObjectuivARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetQueryObjectuiv(id: types::GLuint, pname: types::GLenum, params: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLuint) -> ()>(storage::GetQueryObjectuiv.f)(id, pname, params)
}
/// Fallbacks: DetachObjectARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DetachShader(program: types::GLuint, shader: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(storage::DetachShader.f)(program, shader)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetActiveUniformBlockName(program: types::GLuint, uniformBlockIndex: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, uniformBlockName: *mut types::GLchar) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(storage::GetActiveUniformBlockName.f)(program, uniformBlockIndex, bufSize, length, uniformBlockName)
}
/// Fallbacks: IsSyncAPPLE
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn IsSync(sync: types::GLsync) -> types::GLboolean {
    mem::transmute::<_, extern "system" fn(types::GLsync) -> types::GLboolean>(storage::IsSync.f)(sync)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetBooleanv(pname: types::GLenum, data: *mut types::GLboolean) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLboolean) -> ()>(storage::GetBooleanv.f)(pname, data)
}
/// Fallbacks: QueryCounterEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn QueryCounter(id: types::GLuint, target: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum) -> ()>(storage::QueryCounter.f)(id, target)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn InvalidateNamedFramebufferData(framebuffer: types::GLuint, numAttachments: types::GLsizei, attachments: *const types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLenum) -> ()>(storage::InvalidateNamedFramebufferData.f)(framebuffer, numAttachments, attachments)
}
/// Fallbacks: TexSubImage1DEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexSubImage1D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, width: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLenum, types::GLenum, *const c_void) -> ()>(storage::TexSubImage1D.f)(target, level, xoffset, width, format, type_, pixels)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CopyTextureSubImage1D(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei) -> ()>(storage::CopyTextureSubImage1D.f)(texture, level, xoffset, x, y, width)
}
/// Fallbacks: GetIntegerIndexedvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetIntegeri_v(target: types::GLenum, index: types::GLuint, data: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, *mut types::GLint) -> ()>(storage::GetIntegeri_v.f)(target, index, data)
}
/// Fallbacks: Uniform3fvARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform3fv(location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(storage::Uniform3fv.f)(location, count, value)
}
/// Fallbacks: VertexAttrib1dvARB, VertexAttrib1dvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib1dv(index: types::GLuint, v: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(storage::VertexAttrib1dv.f)(index, v)
}
/// Fallbacks: DisableIndexedEXT, DisableiEXT, DisableiNV, DisableiOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Disablei(target: types::GLenum, index: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::Disablei.f)(target, index)
}
/// Fallbacks: ViewportIndexedfvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ViewportIndexedfv(index: types::GLuint, v: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(storage::ViewportIndexedfv.f)(index, v)
}
/// Fallbacks: PatchParameteriEXT, PatchParameteriOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn PatchParameteri(pname: types::GLenum, value: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(storage::PatchParameteri.f)(pname, value)
}
/// Fallbacks: VertexAttribI2iEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribI2i(index: types::GLuint, x: types::GLint, y: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint) -> ()>(storage::VertexAttribI2i.f)(index, x, y)
}
/// Fallbacks: Uniform1iARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform1i(location: types::GLint, v0: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLint) -> ()>(storage::Uniform1i.f)(location, v0)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UniformMatrix3x4dv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(storage::UniformMatrix3x4dv.f)(location, count, transpose, value)
}
/// Fallbacks: VertexAttribL4dvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribL4dv(index: types::GLuint, v: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(storage::VertexAttribL4dv.f)(index, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn SamplerParameterfv(sampler: types::GLuint, pname: types::GLenum, param: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLfloat) -> ()>(storage::SamplerParameterfv.f)(sampler, pname, param)
}
/// Fallbacks: VertexAttrib3dvARB, VertexAttrib3dvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib3dv(index: types::GLuint, v: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(storage::VertexAttrib3dv.f)(index, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ColorMask(red: types::GLboolean, green: types::GLboolean, blue: types::GLboolean, alpha: types::GLboolean) -> () {
    mem::transmute::<_, extern "system" fn(types::GLboolean, types::GLboolean, types::GLboolean, types::GLboolean) -> ()>(storage::ColorMask.f)(red, green, blue, alpha)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetUniformBlockIndex(program: types::GLuint, uniformBlockName: *const types::GLchar) -> types::GLuint {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLuint>(storage::GetUniformBlockIndex.f)(program, uniformBlockName)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TextureParameterf(texture: types::GLuint, pname: types::GLenum, param: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLfloat) -> ()>(storage::TextureParameterf.f)(texture, pname, param)
}
/// Fallbacks: GetMultisamplefvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetMultisamplefv(pname: types::GLenum, index: types::GLuint, val: *mut types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, *mut types::GLfloat) -> ()>(storage::GetMultisamplefv.f)(pname, index, val)
}
/// Fallbacks: ProgramParameteriARB, ProgramParameteriEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramParameteri(program: types::GLuint, pname: types::GLenum, value: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLint) -> ()>(storage::ProgramParameteri.f)(program, pname, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn MapNamedBuffer(buffer: types::GLuint, access: types::GLenum) -> *mut c_void {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum) -> *mut c_void>(storage::MapNamedBuffer.f)(buffer, access)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TextureBuffer(texture: types::GLuint, internalformat: types::GLenum, buffer: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint) -> ()>(storage::TextureBuffer.f)(texture, internalformat, buffer)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn NormalP3uiv(type_: types::GLenum, coords: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(storage::NormalP3uiv.f)(type_, coords)
}
/// Fallbacks: BlendFuncIndexedAMD, BlendFunciARB, BlendFunciEXT, BlendFunciOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BlendFunci(buf: types::GLuint, src: types::GLenum, dst: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum) -> ()>(storage::BlendFunci.f)(buf, src, dst)
}
/// Fallbacks: VertexAttrib2sARB, VertexAttrib2sNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib2s(index: types::GLuint, x: types::GLshort, y: types::GLshort) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLshort, types::GLshort) -> ()>(storage::VertexAttrib2s.f)(index, x, y)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribP3ui(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLboolean, types::GLuint) -> ()>(storage::VertexAttribP3ui.f)(index, type_, normalized, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetNamedFramebufferAttachmentParameteriv(framebuffer: types::GLuint, attachment: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetNamedFramebufferAttachmentParameteriv.f)(framebuffer, attachment, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn NamedRenderbufferStorage(renderbuffer: types::GLuint, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLsizei, types::GLsizei) -> ()>(storage::NamedRenderbufferStorage.f)(renderbuffer, internalformat, width, height)
}
/// Fallbacks: ProgramUniform1fvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform1fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(storage::ProgramUniform1fv.f)(program, location, count, value)
}
/// Fallbacks: BlendEquationSeparateEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BlendEquationSeparate(modeRGB: types::GLenum, modeAlpha: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(storage::BlendEquationSeparate.f)(modeRGB, modeAlpha)
}
/// Fallbacks: TexBufferARB, TexBufferEXT, TexBufferOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexBuffer(target: types::GLenum, internalformat: types::GLenum, buffer: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint) -> ()>(storage::TexBuffer.f)(target, internalformat, buffer)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexImage1D(target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLint, types::GLenum, types::GLenum, *const c_void) -> ()>(storage::TexImage1D.f)(target, level, internalformat, width, border, format, type_, pixels)
}
/// Fallbacks: TexParameterIuivEXT, TexParameterIuivOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexParameterIuiv(target: types::GLenum, pname: types::GLenum, params: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLuint) -> ()>(storage::TexParameterIuiv.f)(target, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexP2ui(type_: types::GLenum, value: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::VertexP2ui.f)(type_, value)
}
/// Fallbacks: GenRenderbuffersEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GenRenderbuffers(n: types::GLsizei, renderbuffers: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenRenderbuffers.f)(n, renderbuffers)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexBindingDivisor(bindingindex: types::GLuint, divisor: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(storage::VertexBindingDivisor.f)(bindingindex, divisor)
}
/// Fallbacks: ProgramUniform2iEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform2i(program: types::GLuint, location: types::GLint, v0: types::GLint, v1: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint) -> ()>(storage::ProgramUniform2i.f)(program, location, v0, v1)
}
/// Fallbacks: EnableIndexedEXT, EnableiEXT, EnableiNV, EnableiOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Enablei(target: types::GLenum, index: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::Enablei.f)(target, index)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetnMapfv(target: types::GLenum, query: types::GLenum, bufSize: types::GLsizei, v: *mut types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLsizei, *mut types::GLfloat) -> ()>(storage::GetnMapfv.f)(target, query, bufSize, v)
}
/// Fallbacks: IsEnabledIndexedEXT, IsEnablediEXT, IsEnablediNV, IsEnablediOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn IsEnabledi(target: types::GLenum, index: types::GLuint) -> types::GLboolean {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> types::GLboolean>(storage::IsEnabledi.f)(target, index)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CompressedTextureSubImage3D(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei, types::GLenum, types::GLsizei, *const c_void) -> ()>(storage::CompressedTextureSubImage3D.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetShaderPrecisionFormat(shadertype: types::GLenum, precisiontype: types::GLenum, range: *mut types::GLint, precision: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint, *mut types::GLint) -> ()>(storage::GetShaderPrecisionFormat.f)(shadertype, precisiontype, range, precision)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetTextureImage(texture: types::GLuint, level: types::GLint, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, pixels: *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLenum, types::GLsizei, *mut c_void) -> ()>(storage::GetTextureImage.f)(texture, level, format, type_, bufSize, pixels)
}
/// Fallbacks: UniformMatrix3x4fvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UniformMatrix3x4fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::UniformMatrix3x4fv.f)(location, count, transpose, value)
}
/// Fallbacks: Uniform2uivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform2uiv(location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLuint) -> ()>(storage::Uniform2uiv.f)(location, count, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetInternalformati64v(target: types::GLenum, internalformat: types::GLenum, pname: types::GLenum, bufSize: types::GLsizei, params: *mut types::GLint64) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLsizei, *mut types::GLint64) -> ()>(storage::GetInternalformati64v.f)(target, internalformat, pname, bufSize, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform2dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(storage::ProgramUniform2dv.f)(program, location, count, value)
}
/// Fallbacks: VertexAttrib3sARB, VertexAttrib3sNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib3s(index: types::GLuint, x: types::GLshort, y: types::GLshort, z: types::GLshort) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLshort, types::GLshort, types::GLshort) -> ()>(storage::VertexAttrib3s.f)(index, x, y, z)
}
/// Fallbacks: FlushMappedBufferRangeAPPLE, FlushMappedBufferRangeEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn FlushMappedBufferRange(target: types::GLenum, offset: types::GLintptr, length: types::GLsizeiptr) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLintptr, types::GLsizeiptr) -> ()>(storage::FlushMappedBufferRange.f)(target, offset, length)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn InvalidateTexImage(texture: types::GLuint, level: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint) -> ()>(storage::InvalidateTexImage.f)(texture, level)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetProgramInterfaceiv(program: types::GLuint, programInterface: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetProgramInterfaceiv.f)(program, programInterface, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CullFace(mode: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::CullFace.f)(mode)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetFramebufferParameteriv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetFramebufferParameteriv.f)(target, pname, params)
}
/// Fallbacks: CreateShaderObjectARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CreateShader(type_: types::GLenum) -> types::GLuint {
    mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLuint>(storage::CreateShader.f)(type_)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniformMatrix3dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(storage::ProgramUniformMatrix3dv.f)(program, location, count, transpose, value)
}
/// Fallbacks: PointParameterfvARB, PointParameterfvEXT, PointParameterfvSGIS
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn PointParameterfv(pname: types::GLenum, params: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLfloat) -> ()>(storage::PointParameterfv.f)(pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DrawArraysIndirect(mode: types::GLenum, indirect: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *const c_void) -> ()>(storage::DrawArraysIndirect.f)(mode, indirect)
}
/// Fallbacks: UseProgramObjectARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UseProgram(program: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::UseProgram.f)(program)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniformMatrix3x2dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(storage::ProgramUniformMatrix3x2dv.f)(program, location, count, transpose, value)
}
/// Fallbacks: SampleCoverageARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn SampleCoverage(value: types::GLfloat, invert: types::GLboolean) -> () {
    mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLboolean) -> ()>(storage::SampleCoverage.f)(value, invert)
}
/// Fallbacks: Uniform3ivARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform3iv(location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> ()>(storage::Uniform3iv.f)(location, count, value)
}
/// Fallbacks: VertexAttribI3ivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribI3iv(index: types::GLuint, v: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(storage::VertexAttribI3iv.f)(index, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform1dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(storage::ProgramUniform1dv.f)(program, location, count, value)
}
/// Fallbacks: BlendEquationSeparateIndexedAMD, BlendEquationSeparateiARB, BlendEquationSeparateiEXT, BlendEquationSeparateiOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BlendEquationSeparatei(buf: types::GLuint, modeRGB: types::GLenum, modeAlpha: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLenum) -> ()>(storage::BlendEquationSeparatei.f)(buf, modeRGB, modeAlpha)
}
/// Fallbacks: GetFloatIndexedvEXT, GetFloati_vEXT, GetFloati_vNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetFloati_v(target: types::GLenum, index: types::GLuint, data: *mut types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, *mut types::GLfloat) -> ()>(storage::GetFloati_v.f)(target, index, data)
}
/// Fallbacks: ProgramUniform4ivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform4iv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLint) -> ()>(storage::ProgramUniform4iv.f)(program, location, count, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn SecondaryColorP3ui(type_: types::GLenum, color: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::SecondaryColorP3ui.f)(type_, color)
}
/// Fallbacks: VertexAttribI1uiEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribI1ui(index: types::GLuint, x: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(storage::VertexAttribI1ui.f)(index, x)
}
/// Fallbacks: Uniform1ivARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform1iv(location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> ()>(storage::Uniform1iv.f)(location, count, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetVertexArrayiv(vaobj: types::GLuint, pname: types::GLenum, param: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetVertexArrayiv.f)(vaobj, pname, param)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn IsProgram(program: types::GLuint) -> types::GLboolean {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsProgram.f)(program)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindTextureUnit(unit: types::GLuint, texture: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(storage::BindTextureUnit.f)(unit, texture)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetnPolygonStipple(bufSize: types::GLsizei, pattern: *mut types::GLubyte) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLubyte) -> ()>(storage::GetnPolygonStipple.f)(bufSize, pattern)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetIntegerv(pname: types::GLenum, data: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLint) -> ()>(storage::GetIntegerv.f)(pname, data)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn NamedFramebufferParameteri(framebuffer: types::GLuint, pname: types::GLenum, param: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLint) -> ()>(storage::NamedFramebufferParameteri.f)(framebuffer, pname, param)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexP3uiv(type_: types::GLenum, value: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(storage::VertexP3uiv.f)(type_, value)
}
/// Fallbacks: VertexAttrib4usvARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib4usv(index: types::GLuint, v: *const types::GLushort) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLushort) -> ()>(storage::VertexAttrib4usv.f)(index, v)
}
/// Fallbacks: UniformMatrix2x3fvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UniformMatrix2x3fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::UniformMatrix2x3fv.f)(location, count, transpose, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetnMapdv(target: types::GLenum, query: types::GLenum, bufSize: types::GLsizei, v: *mut types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLsizei, *mut types::GLdouble) -> ()>(storage::GetnMapdv.f)(target, query, bufSize, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexCoordP1uiv(type_: types::GLenum, coords: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(storage::TexCoordP1uiv.f)(type_, coords)
}
/// Fallbacks: Uniform1fvARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform1fv(location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(storage::Uniform1fv.f)(location, count, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetNamedBufferSubData(buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr, data: *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLintptr, types::GLsizeiptr, *mut c_void) -> ()>(storage::GetNamedBufferSubData.f)(buffer, offset, size, data)
}
/// Fallbacks: TransformFeedbackVaryingsEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TransformFeedbackVaryings(program: types::GLuint, count: types::GLsizei, varyings: *const *const types::GLchar, bufferMode: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const *const types::GLchar, types::GLenum) -> ()>(storage::TransformFeedbackVaryings.f)(program, count, varyings, bufferMode)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn InvalidateNamedFramebufferSubData(framebuffer: types::GLuint, numAttachments: types::GLsizei, attachments: *const types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::InvalidateNamedFramebufferSubData.f)(framebuffer, numAttachments, attachments, x, y, width, height)
}
/// Fallbacks: PointParameteriNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn PointParameteri(pname: types::GLenum, param: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(storage::PointParameteri.f)(pname, param)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetTexParameterfv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(storage::GetTexParameterfv.f)(target, pname, params)
}
/// Fallbacks: IsTransformFeedbackNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn IsTransformFeedback(id: types::GLuint) -> types::GLboolean {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(storage::IsTransformFeedback.f)(id)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TextureStorage3D(texture: types::GLuint, levels: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei, types::GLsizei) -> ()>(storage::TextureStorage3D.f)(texture, levels, internalformat, width, height, depth)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ClearNamedBufferSubData(buffer: types::GLuint, internalformat: types::GLenum, offset: types::GLintptr, size: types::GLsizeiptr, format: types::GLenum, type_: types::GLenum, data: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLintptr, types::GLsizeiptr, types::GLenum, types::GLenum, *const c_void) -> ()>(storage::ClearNamedBufferSubData.f)(buffer, internalformat, offset, size, format, type_, data)
}
/// Fallbacks: GetBufferSubDataARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetBufferSubData(target: types::GLenum, offset: types::GLintptr, size: types::GLsizeiptr, data: *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLintptr, types::GLsizeiptr, *mut c_void) -> ()>(storage::GetBufferSubData.f)(target, offset, size, data)
}
/// Fallbacks: VertexAttrib4fvARB, VertexAttrib4fvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib4fv(index: types::GLuint, v: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(storage::VertexAttrib4fv.f)(index, v)
}
/// Fallbacks: GetVertexAttribIivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetVertexAttribIiv(index: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetVertexAttribIiv.f)(index, pname, params)
}
/// Fallbacks: GetDebugMessageLogARB, GetDebugMessageLogKHR
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetDebugMessageLog(count: types::GLuint, bufSize: types::GLsizei, sources: *mut types::GLenum, types: *mut types::GLenum, ids: *mut types::GLuint, severities: *mut types::GLenum, lengths: *mut types::GLsizei, messageLog: *mut types::GLchar) -> types::GLuint {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLenum, *mut types::GLenum, *mut types::GLuint, *mut types::GLenum, *mut types::GLsizei, *mut types::GLchar) -> types::GLuint>(storage::GetDebugMessageLog.f)(count, bufSize, sources, types, ids, severities, lengths, messageLog)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UniformBlockBinding(program: types::GLuint, uniformBlockIndex: types::GLuint, uniformBlockBinding: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::UniformBlockBinding.f)(program, uniformBlockIndex, uniformBlockBinding)
}
/// Fallbacks: MapBufferARB, MapBufferOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn MapBuffer(target: types::GLenum, access: types::GLenum) -> *mut c_void {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> *mut c_void>(storage::MapBuffer.f)(target, access)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn NamedFramebufferDrawBuffers(framebuffer: types::GLuint, n: types::GLsizei, bufs: *const types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLenum) -> ()>(storage::NamedFramebufferDrawBuffers.f)(framebuffer, n, bufs)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribP1uiv(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLboolean, *const types::GLuint) -> ()>(storage::VertexAttribP1uiv.f)(index, type_, normalized, value)
}
/// Fallbacks: ClientWaitSyncAPPLE
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ClientWaitSync(sync: types::GLsync, flags: types::GLbitfield, timeout: types::GLuint64) -> types::GLenum {
    mem::transmute::<_, extern "system" fn(types::GLsync, types::GLbitfield, types::GLuint64) -> types::GLenum>(storage::ClientWaitSync.f)(sync, flags, timeout)
}
/// Fallbacks: GetSamplerParameterIuivEXT, GetSamplerParameterIuivOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetSamplerParameterIuiv(sampler: types::GLuint, pname: types::GLenum, params: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLuint) -> ()>(storage::GetSamplerParameterIuiv.f)(sampler, pname, params)
}
/// Fallbacks: ProgramUniformMatrix4x2fvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniformMatrix4x2fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::ProgramUniformMatrix4x2fv.f)(program, location, count, transpose, value)
}
/// Fallbacks: VertexAttribI4bvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribI4bv(index: types::GLuint, v: *const types::GLbyte) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLbyte) -> ()>(storage::VertexAttribI4bv.f)(index, v)
}
/// Fallbacks: GenFramebuffersEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GenFramebuffers(n: types::GLsizei, framebuffers: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenFramebuffers.f)(n, framebuffers)
}
/// Fallbacks: GetVertexAttribIuivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetVertexAttribIuiv(index: types::GLuint, pname: types::GLenum, params: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLuint) -> ()>(storage::GetVertexAttribIuiv.f)(index, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniformMatrix2x3dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(storage::ProgramUniformMatrix2x3dv.f)(program, location, count, transpose, value)
}
/// Fallbacks: BufferSubDataARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BufferSubData(target: types::GLenum, offset: types::GLintptr, size: types::GLsizeiptr, data: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLintptr, types::GLsizeiptr, *const c_void) -> ()>(storage::BufferSubData.f)(target, offset, size, data)
}
/// Fallbacks: VertexAttrib3fARB, VertexAttrib3fNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib3f(index: types::GLuint, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::VertexAttrib3f.f)(index, x, y, z)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexImage3DMultisample(target: types::GLenum, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, fixedsamplelocations: types::GLboolean) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei, types::GLsizei, types::GLboolean) -> ()>(storage::TexImage3DMultisample.f)(target, samples, internalformat, width, height, depth, fixedsamplelocations)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetTexParameteriv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetTexParameteriv.f)(target, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetnConvolutionFilter(target: types::GLenum, format: types::GLenum, type_: types::GLenum, bufSize: types::GLsizei, image: *mut c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLsizei, *mut c_void) -> ()>(storage::GetnConvolutionFilter.f)(target, format, type_, bufSize, image)
}
/// Fallbacks: VertexAttrib4bvARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib4bv(index: types::GLuint, v: *const types::GLbyte) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLbyte) -> ()>(storage::VertexAttrib4bv.f)(index, v)
}
/// Fallbacks: GetDoubleIndexedvEXT, GetDoublei_vEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetDoublei_v(target: types::GLenum, index: types::GLuint, data: *mut types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, *mut types::GLdouble) -> ()>(storage::GetDoublei_v.f)(target, index, data)
}
/// Fallbacks: DeleteSyncAPPLE
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DeleteSync(sync: types::GLsync) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsync) -> ()>(storage::DeleteSync.f)(sync)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn FlushMappedNamedBufferRange(buffer: types::GLuint, offset: types::GLintptr, length: types::GLsizeiptr) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLintptr, types::GLsizeiptr) -> ()>(storage::FlushMappedNamedBufferRange.f)(buffer, offset, length)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetActiveUniformName(program: types::GLuint, uniformIndex: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, uniformName: *mut types::GLchar) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(storage::GetActiveUniformName.f)(program, uniformIndex, bufSize, length, uniformName)
}
/// Fallbacks: ProgramUniform1uivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform1uiv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLuint) -> ()>(storage::ProgramUniform1uiv.f)(program, location, count, value)
}
/// Fallbacks: ProgramBinaryOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramBinary(program: types::GLuint, binaryFormat: types::GLenum, binary: *const c_void, length: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const c_void, types::GLsizei) -> ()>(storage::ProgramBinary.f)(program, binaryFormat, binary, length)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GenerateTextureMipmap(texture: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::GenerateTextureMipmap.f)(texture)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DepthRangeArrayv(first: types::GLuint, count: types::GLsizei, v: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLdouble) -> ()>(storage::DepthRangeArrayv.f)(first, count, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform2d(program: types::GLuint, location: types::GLint, v0: types::GLdouble, v1: types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLdouble, types::GLdouble) -> ()>(storage::ProgramUniform2d.f)(program, location, v0, v1)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CheckNamedFramebufferStatus(framebuffer: types::GLuint, target: types::GLenum) -> types::GLenum {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum) -> types::GLenum>(storage::CheckNamedFramebufferStatus.f)(framebuffer, target)
}
/// Fallbacks: ResumeTransformFeedbackNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ResumeTransformFeedback() -> () {
    mem::transmute::<_, extern "system" fn() -> ()>(storage::ResumeTransformFeedback.f)()
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribBinding(attribindex: types::GLuint, bindingindex: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(storage::VertexAttribBinding.f)(attribindex, bindingindex)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn PixelStoref(pname: types::GLenum, param: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLfloat) -> ()>(storage::PixelStoref.f)(pname, param)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn MultiTexCoordP1ui(texture: types::GLenum, type_: types::GLenum, coords: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint) -> ()>(storage::MultiTexCoordP1ui.f)(texture, type_, coords)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetSamplerParameterfv(sampler: types::GLuint, pname: types::GLenum, params: *mut types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLfloat) -> ()>(storage::GetSamplerParameterfv.f)(sampler, pname, params)
}
/// Fallbacks: GetTexParameterIuivEXT, GetTexParameterIuivOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetTexParameterIuiv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLuint) -> ()>(storage::GetTexParameterIuiv.f)(target, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ClipControl(origin: types::GLenum, depth: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(storage::ClipControl.f)(origin, depth)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetSubroutineIndex(program: types::GLuint, shadertype: types::GLenum, name: *const types::GLchar) -> types::GLuint {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLchar) -> types::GLuint>(storage::GetSubroutineIndex.f)(program, shadertype, name)
}
/// Fallbacks: GenBuffersARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GenBuffers(n: types::GLsizei, buffers: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenBuffers.f)(n, buffers)
}
/// Fallbacks: GetSamplerParameterIivEXT, GetSamplerParameterIivOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetSamplerParameterIiv(sampler: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetSamplerParameterIiv.f)(sampler, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform3dv(location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(storage::Uniform3dv.f)(location, count, value)
}
/// Fallbacks: ProgramUniformMatrix3x4fvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniformMatrix3x4fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::ProgramUniformMatrix3x4fv.f)(program, location, count, transpose, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn LineWidth(width: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(storage::LineWidth.f)(width)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexArrayAttribLFormat(vaobj: types::GLuint, attribindex: types::GLuint, size: types::GLint, type_: types::GLenum, relativeoffset: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLint, types::GLenum, types::GLuint) -> ()>(storage::VertexArrayAttribLFormat.f)(vaobj, attribindex, size, type_, relativeoffset)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DepthRangeIndexed(index: types::GLuint, n: types::GLdouble, f: types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble, types::GLdouble) -> ()>(storage::DepthRangeIndexed.f)(index, n, f)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniformMatrix3x4dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(storage::ProgramUniformMatrix3x4dv.f)(program, location, count, transpose, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetTextureParameteriv(texture: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetTextureParameteriv.f)(texture, pname, params)
}
/// Fallbacks: DrawElementsInstancedBaseInstanceEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DrawElementsInstancedBaseInstance(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const c_void, instancecount: types::GLsizei, baseinstance: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, *const c_void, types::GLsizei, types::GLuint) -> ()>(storage::DrawElementsInstancedBaseInstance.f)(mode, count, type_, indices, instancecount, baseinstance)
}
/// Fallbacks: GetVertexAttribLdvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetVertexAttribLdv(index: types::GLuint, pname: types::GLenum, params: *mut types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLdouble) -> ()>(storage::GetVertexAttribLdv.f)(index, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexP3ui(type_: types::GLenum, value: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::VertexP3ui.f)(type_, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ClearNamedFramebufferfi(framebuffer: types::GLuint, buffer: types::GLenum, depth: types::GLfloat, stencil: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLfloat, types::GLint) -> ()>(storage::ClearNamedFramebufferfi.f)(framebuffer, buffer, depth, stencil)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DrawTransformFeedbackStreamInstanced(mode: types::GLenum, id: types::GLuint, stream: types::GLuint, instancecount: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLuint, types::GLsizei) -> ()>(storage::DrawTransformFeedbackStreamInstanced.f)(mode, id, stream, instancecount)
}
/// Fallbacks: ProgramUniform3uiEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform3ui(program: types::GLuint, location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::ProgramUniform3ui.f)(program, location, v0, v1, v2)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetTextureLevelParameteriv(texture: types::GLuint, level: types::GLint, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, *mut types::GLint) -> ()>(storage::GetTextureLevelParameteriv.f)(texture, level, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform2dv(location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(storage::Uniform2dv.f)(location, count, value)
}
/// Fallbacks: GetQueryObjectui64vEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetQueryObjectui64v(id: types::GLuint, pname: types::GLenum, params: *mut types::GLuint64) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLuint64) -> ()>(storage::GetQueryObjectui64v.f)(id, pname, params)
}
/// Fallbacks: ProgramUniform2fvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform2fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(storage::ProgramUniform2fv.f)(program, location, count, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn MultiTexCoordP1uiv(texture: types::GLenum, type_: types::GLenum, coords: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLuint) -> ()>(storage::MultiTexCoordP1uiv.f)(texture, type_, coords)
}
/// Fallbacks: RenderbufferStorageMultisampleEXT, RenderbufferStorageMultisampleNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn RenderbufferStorageMultisample(target: types::GLenum, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, types::GLsizei, types::GLsizei) -> ()>(storage::RenderbufferStorageMultisample.f)(target, samples, internalformat, width, height)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ColorP3uiv(type_: types::GLenum, color: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(storage::ColorP3uiv.f)(type_, color)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn MultiTexCoordP2ui(texture: types::GLenum, type_: types::GLenum, coords: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint) -> ()>(storage::MultiTexCoordP2ui.f)(texture, type_, coords)
}
/// Fallbacks: BindFragDataLocationEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindFragDataLocation(program: types::GLuint, color: types::GLuint, name: *const types::GLchar) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, *const types::GLchar) -> ()>(storage::BindFragDataLocation.f)(program, color, name)
}
/// Fallbacks: Uniform4uivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform4uiv(location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLuint) -> ()>(storage::Uniform4uiv.f)(location, count, value)
}
/// Fallbacks: GetFramebufferAttachmentParameterivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetFramebufferAttachmentParameteriv(target: types::GLenum, attachment: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetFramebufferAttachmentParameteriv.f)(target, attachment, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetVertexArrayIndexediv(vaobj: types::GLuint, index: types::GLuint, pname: types::GLenum, param: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetVertexArrayIndexediv.f)(vaobj, index, pname, param)
}
/// Fallbacks: TexParameterIivEXT, TexParameterIivOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexParameterIiv(target: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLint) -> ()>(storage::TexParameterIiv.f)(target, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetNamedBufferParameteri64v(buffer: types::GLuint, pname: types::GLenum, params: *mut types::GLint64) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint64) -> ()>(storage::GetNamedBufferParameteri64v.f)(buffer, pname, params)
}
/// Fallbacks: UniformMatrix3fvARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UniformMatrix3fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::UniformMatrix3fv.f)(location, count, transpose, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ClearBufferData(target: types::GLenum, internalformat: types::GLenum, format: types::GLenum, type_: types::GLenum, data: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLenum, *const c_void) -> ()>(storage::ClearBufferData.f)(target, internalformat, format, type_, data)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexP4uiv(type_: types::GLenum, value: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(storage::VertexP4uiv.f)(type_, value)
}
/// Fallbacks: CopyImageSubDataEXT, CopyImageSubDataOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CopyImageSubData(srcName: types::GLuint, srcTarget: types::GLenum, srcLevel: types::GLint, srcX: types::GLint, srcY: types::GLint, srcZ: types::GLint, dstName: types::GLuint, dstTarget: types::GLenum, dstLevel: types::GLint, dstX: types::GLint, dstY: types::GLint, dstZ: types::GLint, srcWidth: types::GLsizei, srcHeight: types::GLsizei, srcDepth: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLuint, types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei) -> ()>(storage::CopyImageSubData.f)(srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstName, dstTarget, dstLevel, dstX, dstY, dstZ, srcWidth, srcHeight, srcDepth)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform4dv(location: types::GLint, count: types::GLsizei, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLdouble) -> ()>(storage::Uniform4dv.f)(location, count, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GenTextures(n: types::GLsizei, textures: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::GenTextures.f)(n, textures)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexCoordP2uiv(type_: types::GLenum, coords: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, *const types::GLuint) -> ()>(storage::TexCoordP2uiv.f)(type_, coords)
}
/// Fallbacks: VertexAttribL3dvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribL3dv(index: types::GLuint, v: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLdouble) -> ()>(storage::VertexAttribL3dv.f)(index, v)
}
/// Fallbacks: CompressedTexImage1DARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CompressedTexImage1D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, width: types::GLsizei, border: types::GLint, imageSize: types::GLsizei, data: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLsizei, types::GLint, types::GLsizei, *const c_void) -> ()>(storage::CompressedTexImage1D.f)(target, level, internalformat, width, border, imageSize, data)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetTextureParameterIuiv(texture: types::GLuint, pname: types::GLenum, params: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLuint) -> ()>(storage::GetTextureParameterIuiv.f)(texture, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn InvalidateTexSubImage(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLsizei) -> ()>(storage::InvalidateTexSubImage.f)(texture, level, xoffset, yoffset, zoffset, width, height, depth)
}
/// Fallbacks: FenceSyncAPPLE
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn FenceSync(condition: types::GLenum, flags: types::GLbitfield) -> types::GLsync {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLbitfield) -> types::GLsync>(storage::FenceSync.f)(condition, flags)
}
/// Fallbacks: VertexAttribL1dEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribL1d(index: types::GLuint, x: types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble) -> ()>(storage::VertexAttribL1d.f)(index, x)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UniformMatrix4x2dv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(storage::UniformMatrix4x2dv.f)(location, count, transpose, value)
}
/// Fallbacks: PauseTransformFeedbackNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn PauseTransformFeedback() -> () {
    mem::transmute::<_, extern "system" fn() -> ()>(storage::PauseTransformFeedback.f)()
}
/// Fallbacks: VertexAttrib4ivARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib4iv(index: types::GLuint, v: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLint) -> ()>(storage::VertexAttrib4iv.f)(index, v)
}
/// Fallbacks: FramebufferTextureLayerARB, FramebufferTextureLayerEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn FramebufferTextureLayer(target: types::GLenum, attachment: types::GLenum, texture: types::GLuint, level: types::GLint, layer: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint, types::GLint, types::GLint) -> ()>(storage::FramebufferTextureLayer.f)(target, attachment, texture, level, layer)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TextureSubImage2D(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *const c_void) -> ()>(storage::TextureSubImage2D.f)(texture, level, xoffset, yoffset, width, height, format, type_, pixels)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ColorP4ui(type_: types::GLenum, color: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::ColorP4ui.f)(type_, color)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexParameterfv(target: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfloat) -> ()>(storage::TexParameterfv.f)(target, pname, params)
}
/// Fallbacks: PushDebugGroupKHR
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn PushDebugGroup(source: types::GLenum, id: types::GLuint, length: types::GLsizei, message: *const types::GLchar) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei, *const types::GLchar) -> ()>(storage::PushDebugGroup.f)(source, id, length, message)
}
/// Fallbacks: MinSampleShadingARB, MinSampleShadingOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn MinSampleShading(value: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(storage::MinSampleShading.f)(value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindFragDataLocationIndexed(program: types::GLuint, colorNumber: types::GLuint, index: types::GLuint, name: *const types::GLchar) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLuint, *const types::GLchar) -> ()>(storage::BindFragDataLocationIndexed.f)(program, colorNumber, index, name)
}
/// Fallbacks: ScissorIndexedNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ScissorIndexed(index: types::GLuint, left: types::GLint, bottom: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(storage::ScissorIndexed.f)(index, left, bottom, width, height)
}
/// Fallbacks: VertexAttrib1dARB, VertexAttrib1dNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib1d(index: types::GLuint, x: types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLdouble) -> ()>(storage::VertexAttrib1d.f)(index, x)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn LogicOp(opcode: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::LogicOp.f)(opcode)
}
/// Fallbacks: GetBooleanIndexedvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetBooleani_v(target: types::GLenum, index: types::GLuint, data: *mut types::GLboolean) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, *mut types::GLboolean) -> ()>(storage::GetBooleani_v.f)(target, index, data)
}
/// Fallbacks: GetActiveUniformARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetActiveUniform(program: types::GLuint, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, size: *mut types::GLint, type_: *mut types::GLenum, name: *mut types::GLchar) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLint, *mut types::GLenum, *mut types::GLchar) -> ()>(storage::GetActiveUniform.f)(program, index, bufSize, length, size, type_, name)
}
/// Fallbacks: VertexAttrib2fvARB, VertexAttrib2fvNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib2fv(index: types::GLuint, v: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(storage::VertexAttrib2fv.f)(index, v)
}
/// Fallbacks: Uniform4uiEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform4ui(location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint, v3: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLuint, types::GLuint, types::GLuint, types::GLuint) -> ()>(storage::Uniform4ui.f)(location, v0, v1, v2, v3)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform3d(program: types::GLuint, location: types::GLint, v0: types::GLdouble, v1: types::GLdouble, v2: types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::ProgramUniform3d.f)(program, location, v0, v1, v2)
}
/// Fallbacks: VertexAttribI1iEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribI1i(index: types::GLuint, x: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint) -> ()>(storage::VertexAttribI1i.f)(index, x)
}
/// Fallbacks: VertexAttribPointerARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribPointer(index: types::GLuint, size: types::GLint, type_: types::GLenum, normalized: types::GLboolean, stride: types::GLsizei, pointer: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLboolean, types::GLsizei, *const c_void) -> ()>(storage::VertexAttribPointer.f)(index, size, type_, normalized, stride, pointer)
}
/// Fallbacks: GetUniformLocationARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetUniformLocation(program: types::GLuint, name: *const types::GLchar) -> types::GLint {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLint>(storage::GetUniformLocation.f)(program, name)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CreateFramebuffers(n: types::GLsizei, framebuffers: *mut types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(storage::CreateFramebuffers.f)(n, framebuffers)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindSamplers(first: types::GLuint, count: types::GLsizei, samplers: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const types::GLuint) -> ()>(storage::BindSamplers.f)(first, count, samplers)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetProgramResourceIndex(program: types::GLuint, programInterface: types::GLenum, name: *const types::GLchar) -> types::GLuint {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLchar) -> types::GLuint>(storage::GetProgramResourceIndex.f)(program, programInterface, name)
}
/// Fallbacks: GetTexParameterIivEXT, GetTexParameterIivOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetTexParameterIiv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(storage::GetTexParameterIiv.f)(target, pname, params)
}
/// Fallbacks: GetQueryObjectivARB, GetQueryObjectivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetQueryObjectiv(id: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetQueryObjectiv.f)(id, pname, params)
}
/// Fallbacks: VertexAttrib4NbvARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib4Nbv(index: types::GLuint, v: *const types::GLbyte) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLbyte) -> ()>(storage::VertexAttrib4Nbv.f)(index, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetString(name: types::GLenum) -> *const types::GLubyte {
    mem::transmute::<_, extern "system" fn(types::GLenum) -> *const types::GLubyte>(storage::GetString.f)(name)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn MultiTexCoordP4ui(texture: types::GLenum, type_: types::GLenum, coords: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint) -> ()>(storage::MultiTexCoordP4ui.f)(texture, type_, coords)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniformMatrix4dv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(storage::ProgramUniformMatrix4dv.f)(program, location, count, transpose, value)
}
/// Fallbacks: ColorMaskIndexedEXT, ColorMaskiEXT, ColorMaskiOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ColorMaski(index: types::GLuint, r: types::GLboolean, g: types::GLboolean, b: types::GLboolean, a: types::GLboolean) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLboolean, types::GLboolean, types::GLboolean, types::GLboolean) -> ()>(storage::ColorMaski.f)(index, r, g, b, a)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BindFramebuffer(target: types::GLenum, framebuffer: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::BindFramebuffer.f)(target, framebuffer)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetSubroutineUniformLocation(program: types::GLuint, shadertype: types::GLenum, name: *const types::GLchar) -> types::GLint {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLchar) -> types::GLint>(storage::GetSubroutineUniformLocation.f)(program, shadertype, name)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn NamedFramebufferTexture(framebuffer: types::GLuint, attachment: types::GLenum, texture: types::GLuint, level: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, types::GLint) -> ()>(storage::NamedFramebufferTexture.f)(framebuffer, attachment, texture, level)
}
/// Fallbacks: SamplerParameterIivEXT, SamplerParameterIivOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn SamplerParameterIiv(sampler: types::GLuint, pname: types::GLenum, param: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLint) -> ()>(storage::SamplerParameterIiv.f)(sampler, pname, param)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexCoordP3ui(type_: types::GLenum, coords: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::TexCoordP3ui.f)(type_, coords)
}
/// Fallbacks: FramebufferRenderbufferEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn FramebufferRenderbuffer(target: types::GLenum, attachment: types::GLenum, renderbuffertarget: types::GLenum, renderbuffer: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLuint) -> ()>(storage::FramebufferRenderbuffer.f)(target, attachment, renderbuffertarget, renderbuffer)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetProgramResourceName(program: types::GLuint, programInterface: types::GLenum, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, name: *mut types::GLchar) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(storage::GetProgramResourceName.f)(program, programInterface, index, bufSize, length, name)
}
/// Fallbacks: ProgramUniform3uivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform3uiv(program: types::GLuint, location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, *const types::GLuint) -> ()>(storage::ProgramUniform3uiv.f)(program, location, count, value)
}
/// Fallbacks: CompressedTexSubImage1DARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CompressedTexSubImage1D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, width: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLenum, types::GLsizei, *const c_void) -> ()>(storage::CompressedTexSubImage1D.f)(target, level, xoffset, width, format, imageSize, data)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TextureParameterIuiv(texture: types::GLuint, pname: types::GLenum, params: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const types::GLuint) -> ()>(storage::TextureParameterIuiv.f)(texture, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UniformMatrix3x2dv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(storage::UniformMatrix3x2dv.f)(location, count, transpose, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetTextureParameterIiv(texture: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetTextureParameterIiv.f)(texture, pname, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn PrimitiveRestartIndex(index: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(storage::PrimitiveRestartIndex.f)(index)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn StencilMaskSeparate(face: types::GLenum, mask: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::StencilMaskSeparate.f)(face, mask)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform4d(program: types::GLuint, location: types::GLint, v0: types::GLdouble, v1: types::GLdouble, v2: types::GLdouble, v3: types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLdouble, types::GLdouble, types::GLdouble, types::GLdouble) -> ()>(storage::ProgramUniform4d.f)(program, location, v0, v1, v2, v3)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DepthRange(near: types::GLdouble, far: types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLdouble, types::GLdouble) -> ()>(storage::DepthRange.f)(near, far)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn StencilFunc(func: types::GLenum, ref_: types::GLint, mask: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLuint) -> ()>(storage::StencilFunc.f)(func, ref_, mask)
}
/// Fallbacks: DrawElementsBaseVertexEXT, DrawElementsBaseVertexOES
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DrawElementsBaseVertex(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const c_void, basevertex: types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, *const c_void, types::GLint) -> ()>(storage::DrawElementsBaseVertex.f)(mode, count, type_, indices, basevertex)
}
/// Fallbacks: Uniform4ivARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn Uniform4iv(location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> ()>(storage::Uniform4iv.f)(location, count, value)
}
/// Fallbacks: ProgramUniform1fEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform1f(program: types::GLuint, location: types::GLint, v0: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLfloat) -> ()>(storage::ProgramUniform1f.f)(program, location, v0)
}
/// Fallbacks: VertexAttribI3uivEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribI3uiv(index: types::GLuint, v: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLuint) -> ()>(storage::VertexAttribI3uiv.f)(index, v)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn CompressedTextureSubImage2D(texture: types::GLuint, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLsizei, *const c_void) -> ()>(storage::CompressedTextureSubImage2D.f)(texture, level, xoffset, yoffset, width, height, format, imageSize, data)
}
/// Fallbacks: BlitFramebufferEXT, BlitFramebufferNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BlitFramebuffer(srcX0: types::GLint, srcY0: types::GLint, srcX1: types::GLint, srcY1: types::GLint, dstX0: types::GLint, dstY0: types::GLint, dstX1: types::GLint, dstY1: types::GLint, mask: types::GLbitfield, filter: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLbitfield, types::GLenum) -> ()>(storage::BlitFramebuffer.f)(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter)
}
/// Fallbacks: BeginQueryARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BeginQuery(target: types::GLenum, id: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(storage::BeginQuery.f)(target, id)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn UniformMatrix3dv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLdouble) -> ()>(storage::UniformMatrix3dv.f)(location, count, transpose, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DisableVertexArrayAttrib(vaobj: types::GLuint, index: types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(storage::DisableVertexArrayAttrib.f)(vaobj, index)
}
/// Fallbacks: VertexAttrib4fARB, VertexAttrib4fNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttrib4f(index: types::GLuint, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat, w: types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(storage::VertexAttrib4f.f)(index, x, y, z, w)
}
/// Fallbacks: ObjectLabelKHR
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ObjectLabel(identifier: types::GLenum, name: types::GLuint, length: types::GLsizei, label: *const types::GLchar) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei, *const types::GLchar) -> ()>(storage::ObjectLabel.f)(identifier, name, length, label)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn MultiTexCoordP3uiv(texture: types::GLenum, type_: types::GLenum, coords: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLuint) -> ()>(storage::MultiTexCoordP3uiv.f)(texture, type_, coords)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetNamedFramebufferParameteriv(framebuffer: types::GLuint, pname: types::GLenum, param: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetNamedFramebufferParameteriv.f)(framebuffer, pname, param)
}
/// Fallbacks: EndQueryARB
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn EndQuery(target: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(storage::EndQuery.f)(target)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniform1d(program: types::GLuint, location: types::GLint, v0: types::GLdouble) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLdouble) -> ()>(storage::ProgramUniform1d.f)(program, location, v0)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn VertexAttribP3uiv(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, types::GLboolean, *const types::GLuint) -> ()>(storage::VertexAttribP3uiv.f)(index, type_, normalized, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetInternalformativ(target: types::GLenum, internalformat: types::GLenum, pname: types::GLenum, bufSize: types::GLsizei, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLsizei, *mut types::GLint) -> ()>(storage::GetInternalformativ.f)(target, internalformat, pname, bufSize, params)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ClearBufferSubData(target: types::GLenum, internalformat: types::GLenum, offset: types::GLintptr, size: types::GLsizeiptr, format: types::GLenum, type_: types::GLenum, data: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLintptr, types::GLsizeiptr, types::GLenum, types::GLenum, *const c_void) -> ()>(storage::ClearBufferSubData.f)(target, internalformat, offset, size, format, type_, data)
}
/// Fallbacks: BeginConditionalRenderNV
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn BeginConditionalRender(id: types::GLuint, mode: types::GLenum) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum) -> ()>(storage::BeginConditionalRender.f)(id, mode)
}
/// Fallbacks: DrawArraysEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DrawArrays(mode: types::GLenum, first: types::GLint, count: types::GLsizei) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLsizei) -> ()>(storage::DrawArrays.f)(mode, first, count)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn TexImage2D(target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, height: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const c_void) -> () {
    mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLint, types::GLenum, types::GLenum, *const c_void) -> ()>(storage::TexImage2D.f)(target, level, internalformat, width, height, border, format, type_, pixels)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn DeleteSamplers(count: types::GLsizei, samplers: *const types::GLuint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(storage::DeleteSamplers.f)(count, samplers)
}
/// Fallbacks: ProgramUniformMatrix2x4fvEXT
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn ProgramUniformMatrix2x4fv(program: types::GLuint, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(storage::ProgramUniformMatrix2x4fv.f)(program, location, count, transpose, value)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetTransformFeedbackiv(xfb: types::GLuint, pname: types::GLenum, param: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetTransformFeedbackiv.f)(xfb, pname, param)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetFragDataIndex(program: types::GLuint, name: *const types::GLchar) -> types::GLint {
    mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLint>(storage::GetFragDataIndex.f)(program, name)
}
#[allow(non_snake_case, unused_variables, dead_code)]
#[inline]
pub unsafe fn GetProgramPipelineiv(pipeline: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () {
    mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(storage::GetProgramPipelineiv.f)(pipeline, pname, params)
}

#[allow(missing_copy_implementations)]
pub struct FnPtr {
    /// The function pointer that will be used when calling the function.
    f: *const c_void,
    /// True if the pointer points to a real function, false if points to a `panic!` fn.
    is_loaded: bool,
}

impl FnPtr {
    /// Creates a `FnPtr` from a load attempt.
    pub fn new(ptr: *const c_void) -> FnPtr {
        if ptr.is_null() {
            FnPtr { f: missing_fn_panic as *const c_void, is_loaded: false }
        } else {
            FnPtr { f: ptr, is_loaded: true }
        }
    }
}

mod storage {
    #![allow(non_snake_case)]

    use c_types::*;

    use super::FnPtr;
    pub static mut EndTransformFeedback: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetProgramResourceLocationIndex: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetProgramResourceLocation: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribL3d: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ObjectPtrLabel: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ActiveShaderProgram: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindProgramPipeline: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CreateProgramPipelines: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut NormalP3ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UseProgramStages: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribL2d: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetnHistogram: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ScissorArrayv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribDivisor: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetTexImage: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut SamplerParameteri: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TextureBarrier: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TextureParameteri: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetObjectLabel: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ReadBuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut StencilOpSeparate: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexSubImage2D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetTransformFeedbackVarying: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut MapNamedBufferRange: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut SamplerParameteriv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform4fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UniformMatrix4x3dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ScissorIndexedv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindImageTexture: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BlendColor: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetPointerv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform2uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DrawElementsInstancedBaseVertexBaseInstance: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetInteger64v: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribI2ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform1i: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetVertexAttribiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform4i: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexArrayAttribBinding: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetFloatv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DispatchComputeIndirect: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Enable: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindBufferRange: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ShaderSource: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexArrayAttribIFormat: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribI4ubv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib1s: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribI2iv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetObjectPtrLabel: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform2d: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut MultiDrawArraysIndirect: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DrawArraysInstanced: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetVertexArrayIndexed64iv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetQueryIndexediv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetFragDataLocation: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DispatchCompute: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CopyTextureSubImage2D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ClearTexImage: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribI4ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib4Nsv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribI3i: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribP4uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribP2uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform2ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Viewport: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetError: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DrawBuffers: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetTextureLevelParameterfv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut NamedBufferStorage: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DrawRangeElementsBaseVertex: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniformMatrix2dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetVertexAttribdv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetnUniformdv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ClearBufferuiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut IsEnabled: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DrawTransformFeedback: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribL2dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DepthFunc: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut MultiDrawElements: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Flush: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetUniformfv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetnPixelMapuiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetQueryObjecti64v: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GenerateMipmap: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DrawTransformFeedbackStream: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetTexLevelParameterfv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib4uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UniformMatrix4dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib4d: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DepthMask: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribL4d: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CopyTexSubImage1D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform1ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib4Nubv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UniformSubroutinesuiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Scissor: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TextureStorage3DMultisample: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut StencilFuncSeparate: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexCoordP3uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ValidateProgram: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut InvalidateSubFramebuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib3fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DeleteVertexArrays: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribI4uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib4sv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut SamplerParameterf: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribI1iv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexParameteriv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform4i: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexCoordP1ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut IsFramebuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut IsTexture: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BlendFunc: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform4ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UniformMatrix2dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexArrayElementBuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GenProgramPipelines: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut NamedFramebufferReadBuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DrawElements: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TextureParameteriv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut StencilOp: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindVertexBuffers: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut PopDebugGroup: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform2ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut SecondaryColorP3uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindSampler: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform1dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib3d: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetNamedBufferPointerv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CreateSamplers: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut EndQueryIndexed: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ClearBufferfv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UniformMatrix4x2fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut StencilMask: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UniformMatrix4fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut PolygonMode: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CompressedTexSubImage3D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribP4ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribIPointer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut NamedFramebufferTextureLayer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DeleteFramebuffers: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Disable: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetShaderInfoLog: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform3d: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CopyTextureSubImage3D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut InvalidateBufferData: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut EndConditionalRender: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ReleaseShaderCompiler: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut NamedBufferSubData: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetnPixelMapfv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UniformMatrix3x2fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CopyNamedBufferSubData: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniformMatrix4x2dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetDoublev: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DisableVertexAttribArray: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindBuffersRange: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform4uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ActiveTexture: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetProgramiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribIFormat: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CopyTexSubImage3D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetActiveAtomicCounterBufferiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DrawElementsIndirect: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ViewportIndexedf: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib4ubv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ClearBufferfi: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribI1uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut AttachShader: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib3sv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindTransformFeedback: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform3i: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ClearBufferiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform3iv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetCompressedTexImage: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetQueryBufferObjecti64v: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform4dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexArrayVertexBuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform2f: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetNamedRenderbufferParameteriv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib2sv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetTextureSubImage: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribI3ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetQueryiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut MemoryBarrierByRegion: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniformMatrix3fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib1sv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindTexture: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TextureBufferRange: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform4f: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ClearDepth: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut FrontFace: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetTextureParameterfv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut MemoryBarrier: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ViewportArrayv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BeginQueryIndexed: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut PatchParameterfv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindTextures: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetProgramPipelineInfoLog: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetUniformuiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut MultiDrawArrays: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform1ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetStringi: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetShaderSource: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut MapBufferRange: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib4Nuiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ClearColor: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform3ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CreateProgram: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut IsProgramPipeline: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform3f: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CreateQueries: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetNamedBufferParameteriv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetShaderiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut PointSize: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DrawTransformFeedbackInstanced: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut IsVertexArray: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetCompressedTextureSubImage: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetnPixelMapusv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BeginTransformFeedback: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetGraphicsResetStatus: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Clear: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ColorP3ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CreateBuffers: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexParameteri: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform2i: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut IsShader: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetBufferParameteriv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetCompressedTextureImage: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform1f: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ClearNamedFramebufferuiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BlendEquationi: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CopyBufferSubData: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut PointParameteriv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetnUniformiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetActiveUniformsiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindBuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DeleteProgram: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib2dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniformMatrix2x3fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindAttribLocation: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProvokingVertex: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetTransformFeedbacki_v: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform4f: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CompressedTextureSubImage1D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexStorage1D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribI4usv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut IsRenderbuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribP1ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform3uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniformMatrix4x3fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetUniformIndices: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GenSamplers: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniformMatrix4fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexArrayBindingDivisor: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexP2uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib4s: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DeleteTextures: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindImageTextures: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut WaitSync: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindVertexArray: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetActiveAttrib: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TextureStorage2DMultisample: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DebugMessageInsert: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DeleteTransformFeedbacks: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TextureSubImage1D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribL1dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib1fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetBufferParameteri64v: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DeleteRenderbuffers: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetRenderbufferParameteriv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TextureParameterfv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexBufferRange: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut NamedBufferData: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut PixelStorei: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetActiveSubroutineUniformName: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BlendEquation: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BufferData: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CompressedTexSubImage2D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut FramebufferTexture3D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniformMatrix4x3dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetnCompressedTexImage: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetProgramStageiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ClampColor: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ValidateProgramPipeline: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetVertexAttribfv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniformMatrix2x4dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UniformMatrix4x3fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut MultiTexCoordP2uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DeleteShader: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut NamedFramebufferRenderbuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetAttribLocation: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetInteger64i_v: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CopyTexImage1D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib2f: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribI4iv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ClearDepthf: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UniformMatrix2x3dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetTexLevelParameteriv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ReadnPixels: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut LinkProgram: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut EnableVertexArrayAttrib: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribLPointer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TextureView: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetActiveSubroutineUniformiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetQueryBufferObjectui64v: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CompileShader: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform2fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexSubImage3D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexImage2DMultisample: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform4d: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetTransformFeedbacki64_v: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniformMatrix3x2fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniformMatrix2fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CreateVertexArrays: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindBufferBase: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetSamplerParameteriv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ReadPixels: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribLFormat: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetQueryBufferObjectuiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut FramebufferTexture: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexParameterf: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut FramebufferParameteri: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TextureParameterIiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindBuffersBase: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexStorage3DMultisample: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribI4i: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DrawRangeElements: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexImage3D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TextureStorage2D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TransformFeedbackBufferRange: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexP4ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BlendFuncSeparate: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform4fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CreateShaderProgramv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindVertexBuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexStorage2DMultisample: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ShaderStorageBlockBinding: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut NamedRenderbufferStorageMultisample: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetProgramResourceiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut EnableVertexAttribArray: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexCoordP2ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexStorage2D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib4Niv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexArrayVertexBuffers: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform2iv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UniformMatrix2fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetnMinmax: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UniformMatrix2x4fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Finish: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut MultiDrawElementsIndirect: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DebugMessageCallback: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetnUniformfv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut SamplerParameterIuiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CopyTexImage2D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UniformMatrix2x4dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut FramebufferTexture2D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribFormat: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ClearNamedBufferData: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CheckFramebufferStatus: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribI2uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BufferStorage: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut PointParameterf: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetnColorTable: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetnTexImage: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DeleteQueries: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CreateTransformFeedbacks: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform3fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TransformFeedbackBufferBase: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UnmapNamedBuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetUniformdv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CompressedTexImage3D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DrawElementsInstanced: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GenQueries: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CopyTexSubImage2D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DrawArraysInstancedBaseInstance: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexCoordP4ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribP2ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib4dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ColorP4uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetActiveSubroutineName: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexCoordP4uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform3f: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform1iv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib1f: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform1d: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform2iv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CompressedTexImage2D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DrawBuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ClearNamedFramebufferiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Hint: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DeleteBuffers: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexArrayAttribFormat: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GenTransformFeedbacks: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut IsBuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DrawElementsInstancedBaseVertex: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform3i: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetProgramBinary: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetVertexAttribPointerv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetActiveUniformBlockiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform3dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexStorage3D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetQueryBufferObjectiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DepthRangef: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DeleteProgramPipelines: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib4Nusv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ClearTexSubImage: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut MultiTexCoordP3ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform2f: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut IsQuery: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetnSeparableFilter: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetProgramInfoLog: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindRenderbuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut RenderbufferStorage: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DebugMessageControl: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetnUniformuiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut PolygonOffset: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut MultiDrawElementsBaseVertex: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut NamedFramebufferDrawBuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib2d: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CreateTextures: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetUniformSubroutineuiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ClearNamedFramebufferfv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CreateRenderbuffers: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut IsSampler: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut MultiTexCoordP4uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetSynciv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UnmapBuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetBufferPointerv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GenVertexArrays: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut SampleMaski: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ClearStencil: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BlendFuncSeparatei: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib4Nub: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ShaderBinary: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TextureSubImage3D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetUniformiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform1uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribI4sv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BlitNamedFramebuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetAttachedShaders: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut InvalidateBufferSubData: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut InvalidateFramebuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TextureStorage1D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut FramebufferTexture1D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetnMapiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetQueryObjectuiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DetachShader: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetActiveUniformBlockName: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut IsSync: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetBooleanv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut QueryCounter: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut InvalidateNamedFramebufferData: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexSubImage1D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CopyTextureSubImage1D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetIntegeri_v: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform3fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib1dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Disablei: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ViewportIndexedfv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut PatchParameteri: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribI2i: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform1i: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UniformMatrix3x4dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribL4dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut SamplerParameterfv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib3dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ColorMask: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetUniformBlockIndex: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TextureParameterf: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetMultisamplefv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramParameteri: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut MapNamedBuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TextureBuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut NormalP3uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BlendFunci: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib2s: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribP3ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetNamedFramebufferAttachmentParameteriv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut NamedRenderbufferStorage: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform1fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BlendEquationSeparate: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexBuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexImage1D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexParameterIuiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexP2ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GenRenderbuffers: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexBindingDivisor: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform2i: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Enablei: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetnMapfv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut IsEnabledi: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CompressedTextureSubImage3D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetShaderPrecisionFormat: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetTextureImage: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UniformMatrix3x4fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform2uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetInternalformati64v: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform2dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib3s: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut FlushMappedBufferRange: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut InvalidateTexImage: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetProgramInterfaceiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CullFace: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetFramebufferParameteriv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CreateShader: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniformMatrix3dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut PointParameterfv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DrawArraysIndirect: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UseProgram: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniformMatrix3x2dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut SampleCoverage: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform3iv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribI3iv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform1dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BlendEquationSeparatei: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetFloati_v: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform4iv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut SecondaryColorP3ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribI1ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform1iv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetVertexArrayiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut IsProgram: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindTextureUnit: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetnPolygonStipple: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetIntegerv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut NamedFramebufferParameteri: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexP3uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib4usv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UniformMatrix2x3fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetnMapdv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexCoordP1uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform1fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetNamedBufferSubData: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TransformFeedbackVaryings: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut InvalidateNamedFramebufferSubData: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut PointParameteri: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetTexParameterfv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut IsTransformFeedback: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TextureStorage3D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ClearNamedBufferSubData: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetBufferSubData: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib4fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetVertexAttribIiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetDebugMessageLog: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UniformBlockBinding: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut MapBuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut NamedFramebufferDrawBuffers: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribP1uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ClientWaitSync: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetSamplerParameterIuiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniformMatrix4x2fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribI4bv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GenFramebuffers: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetVertexAttribIuiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniformMatrix2x3dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BufferSubData: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib3f: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexImage3DMultisample: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetTexParameteriv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetnConvolutionFilter: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib4bv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetDoublei_v: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DeleteSync: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut FlushMappedNamedBufferRange: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetActiveUniformName: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform1uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramBinary: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GenerateTextureMipmap: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DepthRangeArrayv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform2d: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CheckNamedFramebufferStatus: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ResumeTransformFeedback: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribBinding: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut PixelStoref: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut MultiTexCoordP1ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetSamplerParameterfv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetTexParameterIuiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ClipControl: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetSubroutineIndex: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GenBuffers: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetSamplerParameterIiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform3dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniformMatrix3x4fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut LineWidth: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexArrayAttribLFormat: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DepthRangeIndexed: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniformMatrix3x4dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetTextureParameteriv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DrawElementsInstancedBaseInstance: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetVertexAttribLdv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexP3ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ClearNamedFramebufferfi: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DrawTransformFeedbackStreamInstanced: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform3ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetTextureLevelParameteriv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform2dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetQueryObjectui64v: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform2fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut MultiTexCoordP1uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut RenderbufferStorageMultisample: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ColorP3uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut MultiTexCoordP2ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindFragDataLocation: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform4uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetFramebufferAttachmentParameteriv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetVertexArrayIndexediv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexParameterIiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetNamedBufferParameteri64v: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UniformMatrix3fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ClearBufferData: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexP4uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CopyImageSubData: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform4dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GenTextures: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexCoordP2uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribL3dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CompressedTexImage1D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetTextureParameterIuiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut InvalidateTexSubImage: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut FenceSync: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribL1d: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UniformMatrix4x2dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut PauseTransformFeedback: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib4iv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut FramebufferTextureLayer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TextureSubImage2D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ColorP4ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexParameterfv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut PushDebugGroup: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut MinSampleShading: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindFragDataLocationIndexed: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ScissorIndexed: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib1d: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut LogicOp: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetBooleani_v: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetActiveUniform: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib2fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform4ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform3d: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribI1i: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribPointer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetUniformLocation: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CreateFramebuffers: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindSamplers: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetProgramResourceIndex: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetTexParameterIiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetQueryObjectiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib4Nbv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetString: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut MultiTexCoordP4ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniformMatrix4dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ColorMaski: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BindFramebuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetSubroutineUniformLocation: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut NamedFramebufferTexture: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut SamplerParameterIiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexCoordP3ui: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut FramebufferRenderbuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetProgramResourceName: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform3uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CompressedTexSubImage1D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TextureParameterIuiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UniformMatrix3x2dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetTextureParameterIiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut PrimitiveRestartIndex: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut StencilMaskSeparate: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform4d: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DepthRange: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut StencilFunc: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DrawElementsBaseVertex: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut Uniform4iv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform1f: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribI3uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut CompressedTextureSubImage2D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BlitFramebuffer: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BeginQuery: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut UniformMatrix3dv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DisableVertexArrayAttrib: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttrib4f: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ObjectLabel: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut MultiTexCoordP3uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetNamedFramebufferParameteriv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut EndQuery: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniform1d: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut VertexAttribP3uiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetInternalformativ: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ClearBufferSubData: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut BeginConditionalRender: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DrawArrays: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut TexImage2D: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut DeleteSamplers: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut ProgramUniformMatrix2x4fv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetTransformFeedbackiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetFragDataIndex: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
    pub static mut GetProgramPipelineiv: FnPtr = FnPtr { f: super::missing_fn_panic as *const c_void, is_loaded: false };
}

#[allow(non_snake_case)]
pub mod EndTransformFeedback {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::EndTransformFeedback.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::EndTransformFeedback = FnPtr::new(metaloadfn(&mut loadfn, b"glEndTransformFeedback\0", &[b"glEndTransformFeedbackEXT\0", b"glEndTransformFeedbackNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetProgramResourceLocationIndex {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetProgramResourceLocationIndex.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetProgramResourceLocationIndex = FnPtr::new(metaloadfn(&mut loadfn, b"glGetProgramResourceLocationIndex\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetProgramResourceLocation {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetProgramResourceLocation.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetProgramResourceLocation = FnPtr::new(metaloadfn(&mut loadfn, b"glGetProgramResourceLocation\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribL3d {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribL3d.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribL3d = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribL3d\0", &[b"glVertexAttribL3dEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ObjectPtrLabel {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ObjectPtrLabel.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ObjectPtrLabel = FnPtr::new(metaloadfn(&mut loadfn, b"glObjectPtrLabel\0", &[b"glObjectPtrLabelKHR\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ActiveShaderProgram {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ActiveShaderProgram.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ActiveShaderProgram = FnPtr::new(metaloadfn(&mut loadfn, b"glActiveShaderProgram\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BindProgramPipeline {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindProgramPipeline.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindProgramPipeline = FnPtr::new(metaloadfn(&mut loadfn, b"glBindProgramPipeline\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod CreateProgramPipelines {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CreateProgramPipelines.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CreateProgramPipelines = FnPtr::new(metaloadfn(&mut loadfn, b"glCreateProgramPipelines\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod NormalP3ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::NormalP3ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::NormalP3ui = FnPtr::new(metaloadfn(&mut loadfn, b"glNormalP3ui\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod UseProgramStages {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UseProgramStages.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UseProgramStages = FnPtr::new(metaloadfn(&mut loadfn, b"glUseProgramStages\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribL2d {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribL2d.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribL2d = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribL2d\0", &[b"glVertexAttribL2dEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetnHistogram {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetnHistogram.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetnHistogram = FnPtr::new(metaloadfn(&mut loadfn, b"glGetnHistogram\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ScissorArrayv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ScissorArrayv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ScissorArrayv = FnPtr::new(metaloadfn(&mut loadfn, b"glScissorArrayv\0", &[b"glScissorArrayvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribDivisor {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribDivisor.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribDivisor = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribDivisor\0", &[b"glVertexAttribDivisorANGLE\0", b"glVertexAttribDivisorARB\0", b"glVertexAttribDivisorEXT\0", b"glVertexAttribDivisorNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetTexImage {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetTexImage.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetTexImage = FnPtr::new(metaloadfn(&mut loadfn, b"glGetTexImage\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod SamplerParameteri {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::SamplerParameteri.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::SamplerParameteri = FnPtr::new(metaloadfn(&mut loadfn, b"glSamplerParameteri\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TextureBarrier {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TextureBarrier.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TextureBarrier = FnPtr::new(metaloadfn(&mut loadfn, b"glTextureBarrier\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TextureParameteri {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TextureParameteri.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TextureParameteri = FnPtr::new(metaloadfn(&mut loadfn, b"glTextureParameteri\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetObjectLabel {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetObjectLabel.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetObjectLabel = FnPtr::new(metaloadfn(&mut loadfn, b"glGetObjectLabel\0", &[b"glGetObjectLabelKHR\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ReadBuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ReadBuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ReadBuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glReadBuffer\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod StencilOpSeparate {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::StencilOpSeparate.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::StencilOpSeparate = FnPtr::new(metaloadfn(&mut loadfn, b"glStencilOpSeparate\0", &[b"glStencilOpSeparateATI\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TexSubImage2D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexSubImage2D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexSubImage2D = FnPtr::new(metaloadfn(&mut loadfn, b"glTexSubImage2D\0", &[b"glTexSubImage2DEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetTransformFeedbackVarying {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetTransformFeedbackVarying.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetTransformFeedbackVarying = FnPtr::new(metaloadfn(&mut loadfn, b"glGetTransformFeedbackVarying\0", &[b"glGetTransformFeedbackVaryingEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod MapNamedBufferRange {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::MapNamedBufferRange.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::MapNamedBufferRange = FnPtr::new(metaloadfn(&mut loadfn, b"glMapNamedBufferRange\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod SamplerParameteriv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::SamplerParameteriv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::SamplerParameteriv = FnPtr::new(metaloadfn(&mut loadfn, b"glSamplerParameteriv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform4fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform4fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform4fv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform4fv\0", &[b"glProgramUniform4fvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod UniformMatrix4x3dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UniformMatrix4x3dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UniformMatrix4x3dv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniformMatrix4x3dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ScissorIndexedv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ScissorIndexedv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ScissorIndexedv = FnPtr::new(metaloadfn(&mut loadfn, b"glScissorIndexedv\0", &[b"glScissorIndexedvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod BindImageTexture {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindImageTexture.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindImageTexture = FnPtr::new(metaloadfn(&mut loadfn, b"glBindImageTexture\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BlendColor {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BlendColor.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BlendColor = FnPtr::new(metaloadfn(&mut loadfn, b"glBlendColor\0", &[b"glBlendColorEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetPointerv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetPointerv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetPointerv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetPointerv\0", &[b"glGetPointervEXT\0", b"glGetPointervKHR\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform2uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform2uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform2uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform2uiv\0", &[b"glProgramUniform2uivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DrawElementsInstancedBaseVertexBaseInstance {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DrawElementsInstancedBaseVertexBaseInstance.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DrawElementsInstancedBaseVertexBaseInstance = FnPtr::new(metaloadfn(&mut loadfn, b"glDrawElementsInstancedBaseVertexBaseInstance\0", &[b"glDrawElementsInstancedBaseVertexBaseInstanceEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetInteger64v {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetInteger64v.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetInteger64v = FnPtr::new(metaloadfn(&mut loadfn, b"glGetInteger64v\0", &[b"glGetInteger64vAPPLE\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribI2ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribI2ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribI2ui = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribI2ui\0", &[b"glVertexAttribI2uiEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform1i {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform1i.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform1i = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform1i\0", &[b"glProgramUniform1iEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetVertexAttribiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetVertexAttribiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetVertexAttribiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetVertexAttribiv\0", &[b"glGetVertexAttribivARB\0", b"glGetVertexAttribivNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform4i {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform4i.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform4i = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform4i\0", &[b"glProgramUniform4iEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexArrayAttribBinding {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexArrayAttribBinding.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexArrayAttribBinding = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexArrayAttribBinding\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetFloatv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetFloatv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetFloatv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetFloatv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DispatchComputeIndirect {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DispatchComputeIndirect.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DispatchComputeIndirect = FnPtr::new(metaloadfn(&mut loadfn, b"glDispatchComputeIndirect\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod Enable {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Enable.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Enable = FnPtr::new(metaloadfn(&mut loadfn, b"glEnable\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BindBufferRange {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindBufferRange.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindBufferRange = FnPtr::new(metaloadfn(&mut loadfn, b"glBindBufferRange\0", &[b"glBindBufferRangeEXT\0", b"glBindBufferRangeNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ShaderSource {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ShaderSource.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ShaderSource = FnPtr::new(metaloadfn(&mut loadfn, b"glShaderSource\0", &[b"glShaderSourceARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexArrayAttribIFormat {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexArrayAttribIFormat.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexArrayAttribIFormat = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexArrayAttribIFormat\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribI4ubv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribI4ubv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribI4ubv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribI4ubv\0", &[b"glVertexAttribI4ubvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib1s {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib1s.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib1s = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib1s\0", &[b"glVertexAttrib1sARB\0", b"glVertexAttrib1sNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribI2iv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribI2iv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribI2iv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribI2iv\0", &[b"glVertexAttribI2ivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetObjectPtrLabel {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetObjectPtrLabel.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetObjectPtrLabel = FnPtr::new(metaloadfn(&mut loadfn, b"glGetObjectPtrLabel\0", &[b"glGetObjectPtrLabelKHR\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform2d {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform2d.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform2d = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform2d\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod MultiDrawArraysIndirect {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::MultiDrawArraysIndirect.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::MultiDrawArraysIndirect = FnPtr::new(metaloadfn(&mut loadfn, b"glMultiDrawArraysIndirect\0", &[b"glMultiDrawArraysIndirectAMD\0", b"glMultiDrawArraysIndirectEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DrawArraysInstanced {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DrawArraysInstanced.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DrawArraysInstanced = FnPtr::new(metaloadfn(&mut loadfn, b"glDrawArraysInstanced\0", &[b"glDrawArraysInstancedANGLE\0", b"glDrawArraysInstancedARB\0", b"glDrawArraysInstancedEXT\0", b"glDrawArraysInstancedNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetVertexArrayIndexed64iv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetVertexArrayIndexed64iv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetVertexArrayIndexed64iv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetVertexArrayIndexed64iv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetQueryIndexediv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetQueryIndexediv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetQueryIndexediv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetQueryIndexediv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetFragDataLocation {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetFragDataLocation.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetFragDataLocation = FnPtr::new(metaloadfn(&mut loadfn, b"glGetFragDataLocation\0", &[b"glGetFragDataLocationEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DispatchCompute {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DispatchCompute.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DispatchCompute = FnPtr::new(metaloadfn(&mut loadfn, b"glDispatchCompute\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod CopyTextureSubImage2D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CopyTextureSubImage2D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CopyTextureSubImage2D = FnPtr::new(metaloadfn(&mut loadfn, b"glCopyTextureSubImage2D\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ClearTexImage {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ClearTexImage.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ClearTexImage = FnPtr::new(metaloadfn(&mut loadfn, b"glClearTexImage\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribI4ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribI4ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribI4ui = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribI4ui\0", &[b"glVertexAttribI4uiEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib4Nsv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib4Nsv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib4Nsv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib4Nsv\0", &[b"glVertexAttrib4NsvARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribI3i {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribI3i.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribI3i = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribI3i\0", &[b"glVertexAttribI3iEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribP4uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribP4uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribP4uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribP4uiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribP2uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribP2uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribP2uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribP2uiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform2ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform2ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform2ui = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform2ui\0", &[b"glProgramUniform2uiEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Viewport {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Viewport.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Viewport = FnPtr::new(metaloadfn(&mut loadfn, b"glViewport\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetError {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetError.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetError = FnPtr::new(metaloadfn(&mut loadfn, b"glGetError\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DrawBuffers {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DrawBuffers.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DrawBuffers = FnPtr::new(metaloadfn(&mut loadfn, b"glDrawBuffers\0", &[b"glDrawBuffersARB\0", b"glDrawBuffersATI\0", b"glDrawBuffersEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetTextureLevelParameterfv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetTextureLevelParameterfv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetTextureLevelParameterfv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetTextureLevelParameterfv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod NamedBufferStorage {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::NamedBufferStorage.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::NamedBufferStorage = FnPtr::new(metaloadfn(&mut loadfn, b"glNamedBufferStorage\0", &[b"glNamedBufferStorageEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DrawRangeElementsBaseVertex {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DrawRangeElementsBaseVertex.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DrawRangeElementsBaseVertex = FnPtr::new(metaloadfn(&mut loadfn, b"glDrawRangeElementsBaseVertex\0", &[b"glDrawRangeElementsBaseVertexEXT\0", b"glDrawRangeElementsBaseVertexOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniformMatrix2dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniformMatrix2dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniformMatrix2dv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniformMatrix2dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetVertexAttribdv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetVertexAttribdv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetVertexAttribdv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetVertexAttribdv\0", &[b"glGetVertexAttribdvARB\0", b"glGetVertexAttribdvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetnUniformdv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetnUniformdv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetnUniformdv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetnUniformdv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ClearBufferuiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ClearBufferuiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ClearBufferuiv = FnPtr::new(metaloadfn(&mut loadfn, b"glClearBufferuiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod IsEnabled {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::IsEnabled.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::IsEnabled = FnPtr::new(metaloadfn(&mut loadfn, b"glIsEnabled\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DrawTransformFeedback {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DrawTransformFeedback.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DrawTransformFeedback = FnPtr::new(metaloadfn(&mut loadfn, b"glDrawTransformFeedback\0", &[b"glDrawTransformFeedbackNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribL2dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribL2dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribL2dv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribL2dv\0", &[b"glVertexAttribL2dvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DepthFunc {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DepthFunc.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DepthFunc = FnPtr::new(metaloadfn(&mut loadfn, b"glDepthFunc\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod MultiDrawElements {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::MultiDrawElements.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::MultiDrawElements = FnPtr::new(metaloadfn(&mut loadfn, b"glMultiDrawElements\0", &[b"glMultiDrawElementsEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Flush {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Flush.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Flush = FnPtr::new(metaloadfn(&mut loadfn, b"glFlush\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetUniformfv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetUniformfv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetUniformfv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetUniformfv\0", &[b"glGetUniformfvARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetnPixelMapuiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetnPixelMapuiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetnPixelMapuiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetnPixelMapuiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetQueryObjecti64v {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetQueryObjecti64v.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetQueryObjecti64v = FnPtr::new(metaloadfn(&mut loadfn, b"glGetQueryObjecti64v\0", &[b"glGetQueryObjecti64vEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GenerateMipmap {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GenerateMipmap.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GenerateMipmap = FnPtr::new(metaloadfn(&mut loadfn, b"glGenerateMipmap\0", &[b"glGenerateMipmapEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DrawTransformFeedbackStream {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DrawTransformFeedbackStream.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DrawTransformFeedbackStream = FnPtr::new(metaloadfn(&mut loadfn, b"glDrawTransformFeedbackStream\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetTexLevelParameterfv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetTexLevelParameterfv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetTexLevelParameterfv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetTexLevelParameterfv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib4uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib4uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib4uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib4uiv\0", &[b"glVertexAttrib4uivARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod UniformMatrix4dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UniformMatrix4dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UniformMatrix4dv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniformMatrix4dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib4d {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib4d.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib4d = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib4d\0", &[b"glVertexAttrib4dARB\0", b"glVertexAttrib4dNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DepthMask {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DepthMask.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DepthMask = FnPtr::new(metaloadfn(&mut loadfn, b"glDepthMask\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribL4d {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribL4d.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribL4d = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribL4d\0", &[b"glVertexAttribL4dEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod CopyTexSubImage1D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CopyTexSubImage1D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CopyTexSubImage1D = FnPtr::new(metaloadfn(&mut loadfn, b"glCopyTexSubImage1D\0", &[b"glCopyTexSubImage1DEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform1ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform1ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform1ui = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform1ui\0", &[b"glUniform1uiEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib4Nubv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib4Nubv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib4Nubv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib4Nubv\0", &[b"glVertexAttrib4NubvARB\0", b"glVertexAttrib4ubvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod UniformSubroutinesuiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UniformSubroutinesuiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UniformSubroutinesuiv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniformSubroutinesuiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod Scissor {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Scissor.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Scissor = FnPtr::new(metaloadfn(&mut loadfn, b"glScissor\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TextureStorage3DMultisample {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TextureStorage3DMultisample.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TextureStorage3DMultisample = FnPtr::new(metaloadfn(&mut loadfn, b"glTextureStorage3DMultisample\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod StencilFuncSeparate {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::StencilFuncSeparate.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::StencilFuncSeparate = FnPtr::new(metaloadfn(&mut loadfn, b"glStencilFuncSeparate\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TexCoordP3uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexCoordP3uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexCoordP3uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glTexCoordP3uiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ValidateProgram {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ValidateProgram.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ValidateProgram = FnPtr::new(metaloadfn(&mut loadfn, b"glValidateProgram\0", &[b"glValidateProgramARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod InvalidateSubFramebuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::InvalidateSubFramebuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::InvalidateSubFramebuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glInvalidateSubFramebuffer\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib3fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib3fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib3fv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib3fv\0", &[b"glVertexAttrib3fvARB\0", b"glVertexAttrib3fvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DeleteVertexArrays {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DeleteVertexArrays.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DeleteVertexArrays = FnPtr::new(metaloadfn(&mut loadfn, b"glDeleteVertexArrays\0", &[b"glDeleteVertexArraysAPPLE\0", b"glDeleteVertexArraysOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribI4uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribI4uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribI4uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribI4uiv\0", &[b"glVertexAttribI4uivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib4sv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib4sv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib4sv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib4sv\0", &[b"glVertexAttrib4svARB\0", b"glVertexAttrib4svNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod SamplerParameterf {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::SamplerParameterf.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::SamplerParameterf = FnPtr::new(metaloadfn(&mut loadfn, b"glSamplerParameterf\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribI1iv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribI1iv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribI1iv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribI1iv\0", &[b"glVertexAttribI1ivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TexParameteriv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexParameteriv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexParameteriv = FnPtr::new(metaloadfn(&mut loadfn, b"glTexParameteriv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform4i {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform4i.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform4i = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform4i\0", &[b"glUniform4iARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TexCoordP1ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexCoordP1ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexCoordP1ui = FnPtr::new(metaloadfn(&mut loadfn, b"glTexCoordP1ui\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod IsFramebuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::IsFramebuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::IsFramebuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glIsFramebuffer\0", &[b"glIsFramebufferEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod IsTexture {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::IsTexture.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::IsTexture = FnPtr::new(metaloadfn(&mut loadfn, b"glIsTexture\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BlendFunc {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BlendFunc.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BlendFunc = FnPtr::new(metaloadfn(&mut loadfn, b"glBlendFunc\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform4ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform4ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform4ui = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform4ui\0", &[b"glProgramUniform4uiEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod UniformMatrix2dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UniformMatrix2dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UniformMatrix2dv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniformMatrix2dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexArrayElementBuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexArrayElementBuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexArrayElementBuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexArrayElementBuffer\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GenProgramPipelines {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GenProgramPipelines.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GenProgramPipelines = FnPtr::new(metaloadfn(&mut loadfn, b"glGenProgramPipelines\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod NamedFramebufferReadBuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::NamedFramebufferReadBuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::NamedFramebufferReadBuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glNamedFramebufferReadBuffer\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DrawElements {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DrawElements.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DrawElements = FnPtr::new(metaloadfn(&mut loadfn, b"glDrawElements\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TextureParameteriv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TextureParameteriv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TextureParameteriv = FnPtr::new(metaloadfn(&mut loadfn, b"glTextureParameteriv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod StencilOp {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::StencilOp.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::StencilOp = FnPtr::new(metaloadfn(&mut loadfn, b"glStencilOp\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BindVertexBuffers {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindVertexBuffers.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindVertexBuffers = FnPtr::new(metaloadfn(&mut loadfn, b"glBindVertexBuffers\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod PopDebugGroup {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::PopDebugGroup.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::PopDebugGroup = FnPtr::new(metaloadfn(&mut loadfn, b"glPopDebugGroup\0", &[b"glPopDebugGroupKHR\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform2ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform2ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform2ui = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform2ui\0", &[b"glUniform2uiEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod SecondaryColorP3uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::SecondaryColorP3uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::SecondaryColorP3uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glSecondaryColorP3uiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BindSampler {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindSampler.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindSampler = FnPtr::new(metaloadfn(&mut loadfn, b"glBindSampler\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform1dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform1dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform1dv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform1dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib3d {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib3d.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib3d = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib3d\0", &[b"glVertexAttrib3dARB\0", b"glVertexAttrib3dNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetNamedBufferPointerv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetNamedBufferPointerv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetNamedBufferPointerv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetNamedBufferPointerv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod CreateSamplers {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CreateSamplers.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CreateSamplers = FnPtr::new(metaloadfn(&mut loadfn, b"glCreateSamplers\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod EndQueryIndexed {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::EndQueryIndexed.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::EndQueryIndexed = FnPtr::new(metaloadfn(&mut loadfn, b"glEndQueryIndexed\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ClearBufferfv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ClearBufferfv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ClearBufferfv = FnPtr::new(metaloadfn(&mut loadfn, b"glClearBufferfv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod UniformMatrix4x2fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UniformMatrix4x2fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UniformMatrix4x2fv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniformMatrix4x2fv\0", &[b"glUniformMatrix4x2fvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod StencilMask {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::StencilMask.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::StencilMask = FnPtr::new(metaloadfn(&mut loadfn, b"glStencilMask\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod UniformMatrix4fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UniformMatrix4fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UniformMatrix4fv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniformMatrix4fv\0", &[b"glUniformMatrix4fvARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod PolygonMode {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::PolygonMode.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::PolygonMode = FnPtr::new(metaloadfn(&mut loadfn, b"glPolygonMode\0", &[b"glPolygonModeNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod CompressedTexSubImage3D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CompressedTexSubImage3D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CompressedTexSubImage3D = FnPtr::new(metaloadfn(&mut loadfn, b"glCompressedTexSubImage3D\0", &[b"glCompressedTexSubImage3DARB\0", b"glCompressedTexSubImage3DOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribP4ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribP4ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribP4ui = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribP4ui\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribIPointer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribIPointer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribIPointer = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribIPointer\0", &[b"glVertexAttribIPointerEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod NamedFramebufferTextureLayer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::NamedFramebufferTextureLayer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::NamedFramebufferTextureLayer = FnPtr::new(metaloadfn(&mut loadfn, b"glNamedFramebufferTextureLayer\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DeleteFramebuffers {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DeleteFramebuffers.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DeleteFramebuffers = FnPtr::new(metaloadfn(&mut loadfn, b"glDeleteFramebuffers\0", &[b"glDeleteFramebuffersEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Disable {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Disable.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Disable = FnPtr::new(metaloadfn(&mut loadfn, b"glDisable\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetShaderInfoLog {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetShaderInfoLog.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetShaderInfoLog = FnPtr::new(metaloadfn(&mut loadfn, b"glGetShaderInfoLog\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform3d {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform3d.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform3d = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform3d\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod CopyTextureSubImage3D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CopyTextureSubImage3D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CopyTextureSubImage3D = FnPtr::new(metaloadfn(&mut loadfn, b"glCopyTextureSubImage3D\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod InvalidateBufferData {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::InvalidateBufferData.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::InvalidateBufferData = FnPtr::new(metaloadfn(&mut loadfn, b"glInvalidateBufferData\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod EndConditionalRender {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::EndConditionalRender.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::EndConditionalRender = FnPtr::new(metaloadfn(&mut loadfn, b"glEndConditionalRender\0", &[b"glEndConditionalRenderNV\0", b"glEndConditionalRenderNVX\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ReleaseShaderCompiler {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ReleaseShaderCompiler.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ReleaseShaderCompiler = FnPtr::new(metaloadfn(&mut loadfn, b"glReleaseShaderCompiler\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod NamedBufferSubData {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::NamedBufferSubData.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::NamedBufferSubData = FnPtr::new(metaloadfn(&mut loadfn, b"glNamedBufferSubData\0", &[b"glNamedBufferSubDataEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetnPixelMapfv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetnPixelMapfv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetnPixelMapfv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetnPixelMapfv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod UniformMatrix3x2fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UniformMatrix3x2fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UniformMatrix3x2fv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniformMatrix3x2fv\0", &[b"glUniformMatrix3x2fvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod CopyNamedBufferSubData {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CopyNamedBufferSubData.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CopyNamedBufferSubData = FnPtr::new(metaloadfn(&mut loadfn, b"glCopyNamedBufferSubData\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniformMatrix4x2dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniformMatrix4x2dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniformMatrix4x2dv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniformMatrix4x2dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetDoublev {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetDoublev.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetDoublev = FnPtr::new(metaloadfn(&mut loadfn, b"glGetDoublev\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DisableVertexAttribArray {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DisableVertexAttribArray.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DisableVertexAttribArray = FnPtr::new(metaloadfn(&mut loadfn, b"glDisableVertexAttribArray\0", &[b"glDisableVertexAttribArrayARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod BindBuffersRange {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindBuffersRange.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindBuffersRange = FnPtr::new(metaloadfn(&mut loadfn, b"glBindBuffersRange\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform4uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform4uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform4uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform4uiv\0", &[b"glProgramUniform4uivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ActiveTexture {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ActiveTexture.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ActiveTexture = FnPtr::new(metaloadfn(&mut loadfn, b"glActiveTexture\0", &[b"glActiveTextureARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetProgramiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetProgramiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetProgramiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetProgramiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribIFormat {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribIFormat.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribIFormat = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribIFormat\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod CopyTexSubImage3D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CopyTexSubImage3D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CopyTexSubImage3D = FnPtr::new(metaloadfn(&mut loadfn, b"glCopyTexSubImage3D\0", &[b"glCopyTexSubImage3DEXT\0", b"glCopyTexSubImage3DOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetActiveAtomicCounterBufferiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetActiveAtomicCounterBufferiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetActiveAtomicCounterBufferiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetActiveAtomicCounterBufferiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DrawElementsIndirect {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DrawElementsIndirect.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DrawElementsIndirect = FnPtr::new(metaloadfn(&mut loadfn, b"glDrawElementsIndirect\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ViewportIndexedf {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ViewportIndexedf.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ViewportIndexedf = FnPtr::new(metaloadfn(&mut loadfn, b"glViewportIndexedf\0", &[b"glViewportIndexedfNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib4ubv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib4ubv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib4ubv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib4ubv\0", &[b"glVertexAttrib4ubvARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ClearBufferfi {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ClearBufferfi.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ClearBufferfi = FnPtr::new(metaloadfn(&mut loadfn, b"glClearBufferfi\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribI1uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribI1uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribI1uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribI1uiv\0", &[b"glVertexAttribI1uivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod AttachShader {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::AttachShader.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::AttachShader = FnPtr::new(metaloadfn(&mut loadfn, b"glAttachShader\0", &[b"glAttachObjectARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib3sv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib3sv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib3sv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib3sv\0", &[b"glVertexAttrib3svARB\0", b"glVertexAttrib3svNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod BindTransformFeedback {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindTransformFeedback.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindTransformFeedback = FnPtr::new(metaloadfn(&mut loadfn, b"glBindTransformFeedback\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform3i {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform3i.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform3i = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform3i\0", &[b"glProgramUniform3iEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ClearBufferiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ClearBufferiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ClearBufferiv = FnPtr::new(metaloadfn(&mut loadfn, b"glClearBufferiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform3iv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform3iv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform3iv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform3iv\0", &[b"glProgramUniform3ivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetCompressedTexImage {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetCompressedTexImage.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetCompressedTexImage = FnPtr::new(metaloadfn(&mut loadfn, b"glGetCompressedTexImage\0", &[b"glGetCompressedTexImageARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetQueryBufferObjecti64v {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetQueryBufferObjecti64v.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetQueryBufferObjecti64v = FnPtr::new(metaloadfn(&mut loadfn, b"glGetQueryBufferObjecti64v\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform4dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform4dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform4dv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform4dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexArrayVertexBuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexArrayVertexBuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexArrayVertexBuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexArrayVertexBuffer\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform2f {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform2f.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform2f = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform2f\0", &[b"glUniform2fARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetNamedRenderbufferParameteriv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetNamedRenderbufferParameteriv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetNamedRenderbufferParameteriv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetNamedRenderbufferParameteriv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib2sv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib2sv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib2sv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib2sv\0", &[b"glVertexAttrib2svARB\0", b"glVertexAttrib2svNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetTextureSubImage {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetTextureSubImage.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetTextureSubImage = FnPtr::new(metaloadfn(&mut loadfn, b"glGetTextureSubImage\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribI3ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribI3ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribI3ui = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribI3ui\0", &[b"glVertexAttribI3uiEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetQueryiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetQueryiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetQueryiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetQueryiv\0", &[b"glGetQueryivARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod MemoryBarrierByRegion {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::MemoryBarrierByRegion.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::MemoryBarrierByRegion = FnPtr::new(metaloadfn(&mut loadfn, b"glMemoryBarrierByRegion\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniformMatrix3fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniformMatrix3fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniformMatrix3fv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniformMatrix3fv\0", &[b"glProgramUniformMatrix3fvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib1sv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib1sv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib1sv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib1sv\0", &[b"glVertexAttrib1svARB\0", b"glVertexAttrib1svNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod BindTexture {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindTexture.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindTexture = FnPtr::new(metaloadfn(&mut loadfn, b"glBindTexture\0", &[b"glBindTextureEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TextureBufferRange {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TextureBufferRange.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TextureBufferRange = FnPtr::new(metaloadfn(&mut loadfn, b"glTextureBufferRange\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform4f {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform4f.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform4f = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform4f\0", &[b"glUniform4fARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ClearDepth {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ClearDepth.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ClearDepth = FnPtr::new(metaloadfn(&mut loadfn, b"glClearDepth\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod FrontFace {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::FrontFace.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::FrontFace = FnPtr::new(metaloadfn(&mut loadfn, b"glFrontFace\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetTextureParameterfv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetTextureParameterfv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetTextureParameterfv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetTextureParameterfv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod MemoryBarrier {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::MemoryBarrier.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::MemoryBarrier = FnPtr::new(metaloadfn(&mut loadfn, b"glMemoryBarrier\0", &[b"glMemoryBarrierEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ViewportArrayv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ViewportArrayv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ViewportArrayv = FnPtr::new(metaloadfn(&mut loadfn, b"glViewportArrayv\0", &[b"glViewportArrayvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod BeginQueryIndexed {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BeginQueryIndexed.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BeginQueryIndexed = FnPtr::new(metaloadfn(&mut loadfn, b"glBeginQueryIndexed\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod PatchParameterfv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::PatchParameterfv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::PatchParameterfv = FnPtr::new(metaloadfn(&mut loadfn, b"glPatchParameterfv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BindTextures {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindTextures.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindTextures = FnPtr::new(metaloadfn(&mut loadfn, b"glBindTextures\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetProgramPipelineInfoLog {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetProgramPipelineInfoLog.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetProgramPipelineInfoLog = FnPtr::new(metaloadfn(&mut loadfn, b"glGetProgramPipelineInfoLog\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetUniformuiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetUniformuiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetUniformuiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetUniformuiv\0", &[b"glGetUniformuivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod MultiDrawArrays {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::MultiDrawArrays.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::MultiDrawArrays = FnPtr::new(metaloadfn(&mut loadfn, b"glMultiDrawArrays\0", &[b"glMultiDrawArraysEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform1ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform1ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform1ui = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform1ui\0", &[b"glProgramUniform1uiEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetStringi {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetStringi.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetStringi = FnPtr::new(metaloadfn(&mut loadfn, b"glGetStringi\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetShaderSource {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetShaderSource.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetShaderSource = FnPtr::new(metaloadfn(&mut loadfn, b"glGetShaderSource\0", &[b"glGetShaderSourceARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod MapBufferRange {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::MapBufferRange.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::MapBufferRange = FnPtr::new(metaloadfn(&mut loadfn, b"glMapBufferRange\0", &[b"glMapBufferRangeEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib4Nuiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib4Nuiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib4Nuiv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib4Nuiv\0", &[b"glVertexAttrib4NuivARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ClearColor {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ClearColor.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ClearColor = FnPtr::new(metaloadfn(&mut loadfn, b"glClearColor\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform3ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform3ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform3ui = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform3ui\0", &[b"glUniform3uiEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod CreateProgram {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CreateProgram.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CreateProgram = FnPtr::new(metaloadfn(&mut loadfn, b"glCreateProgram\0", &[b"glCreateProgramObjectARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod IsProgramPipeline {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::IsProgramPipeline.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::IsProgramPipeline = FnPtr::new(metaloadfn(&mut loadfn, b"glIsProgramPipeline\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform3f {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform3f.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform3f = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform3f\0", &[b"glUniform3fARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod CreateQueries {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CreateQueries.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CreateQueries = FnPtr::new(metaloadfn(&mut loadfn, b"glCreateQueries\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetNamedBufferParameteriv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetNamedBufferParameteriv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetNamedBufferParameteriv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetNamedBufferParameteriv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetShaderiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetShaderiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetShaderiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetShaderiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod PointSize {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::PointSize.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::PointSize = FnPtr::new(metaloadfn(&mut loadfn, b"glPointSize\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DrawTransformFeedbackInstanced {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DrawTransformFeedbackInstanced.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DrawTransformFeedbackInstanced = FnPtr::new(metaloadfn(&mut loadfn, b"glDrawTransformFeedbackInstanced\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod IsVertexArray {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::IsVertexArray.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::IsVertexArray = FnPtr::new(metaloadfn(&mut loadfn, b"glIsVertexArray\0", &[b"glIsVertexArrayAPPLE\0", b"glIsVertexArrayOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetCompressedTextureSubImage {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetCompressedTextureSubImage.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetCompressedTextureSubImage = FnPtr::new(metaloadfn(&mut loadfn, b"glGetCompressedTextureSubImage\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetnPixelMapusv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetnPixelMapusv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetnPixelMapusv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetnPixelMapusv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BeginTransformFeedback {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BeginTransformFeedback.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BeginTransformFeedback = FnPtr::new(metaloadfn(&mut loadfn, b"glBeginTransformFeedback\0", &[b"glBeginTransformFeedbackEXT\0", b"glBeginTransformFeedbackNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetGraphicsResetStatus {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetGraphicsResetStatus.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetGraphicsResetStatus = FnPtr::new(metaloadfn(&mut loadfn, b"glGetGraphicsResetStatus\0", &[b"glGetGraphicsResetStatusKHR\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Clear {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Clear.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Clear = FnPtr::new(metaloadfn(&mut loadfn, b"glClear\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ColorP3ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ColorP3ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ColorP3ui = FnPtr::new(metaloadfn(&mut loadfn, b"glColorP3ui\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod CreateBuffers {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CreateBuffers.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CreateBuffers = FnPtr::new(metaloadfn(&mut loadfn, b"glCreateBuffers\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TexParameteri {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexParameteri.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexParameteri = FnPtr::new(metaloadfn(&mut loadfn, b"glTexParameteri\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform2i {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform2i.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform2i = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform2i\0", &[b"glUniform2iARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod IsShader {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::IsShader.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::IsShader = FnPtr::new(metaloadfn(&mut loadfn, b"glIsShader\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetBufferParameteriv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetBufferParameteriv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetBufferParameteriv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetBufferParameteriv\0", &[b"glGetBufferParameterivARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetCompressedTextureImage {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetCompressedTextureImage.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetCompressedTextureImage = FnPtr::new(metaloadfn(&mut loadfn, b"glGetCompressedTextureImage\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform1f {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform1f.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform1f = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform1f\0", &[b"glUniform1fARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ClearNamedFramebufferuiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ClearNamedFramebufferuiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ClearNamedFramebufferuiv = FnPtr::new(metaloadfn(&mut loadfn, b"glClearNamedFramebufferuiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BlendEquationi {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BlendEquationi.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BlendEquationi = FnPtr::new(metaloadfn(&mut loadfn, b"glBlendEquationi\0", &[b"glBlendEquationIndexedAMD\0", b"glBlendEquationiARB\0", b"glBlendEquationiEXT\0", b"glBlendEquationiOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod CopyBufferSubData {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CopyBufferSubData.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CopyBufferSubData = FnPtr::new(metaloadfn(&mut loadfn, b"glCopyBufferSubData\0", &[b"glCopyBufferSubDataNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod PointParameteriv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::PointParameteriv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::PointParameteriv = FnPtr::new(metaloadfn(&mut loadfn, b"glPointParameteriv\0", &[b"glPointParameterivNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetnUniformiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetnUniformiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetnUniformiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetnUniformiv\0", &[b"glGetnUniformivKHR\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetActiveUniformsiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetActiveUniformsiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetActiveUniformsiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetActiveUniformsiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BindBuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindBuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindBuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glBindBuffer\0", &[b"glBindBufferARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DeleteProgram {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DeleteProgram.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DeleteProgram = FnPtr::new(metaloadfn(&mut loadfn, b"glDeleteProgram\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib2dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib2dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib2dv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib2dv\0", &[b"glVertexAttrib2dvARB\0", b"glVertexAttrib2dvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniformMatrix2x3fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniformMatrix2x3fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniformMatrix2x3fv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniformMatrix2x3fv\0", &[b"glProgramUniformMatrix2x3fvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod BindAttribLocation {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindAttribLocation.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindAttribLocation = FnPtr::new(metaloadfn(&mut loadfn, b"glBindAttribLocation\0", &[b"glBindAttribLocationARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProvokingVertex {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProvokingVertex.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProvokingVertex = FnPtr::new(metaloadfn(&mut loadfn, b"glProvokingVertex\0", &[b"glProvokingVertexEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetTransformFeedbacki_v {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetTransformFeedbacki_v.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetTransformFeedbacki_v = FnPtr::new(metaloadfn(&mut loadfn, b"glGetTransformFeedbacki_v\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform4f {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform4f.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform4f = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform4f\0", &[b"glProgramUniform4fEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod CompressedTextureSubImage1D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CompressedTextureSubImage1D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CompressedTextureSubImage1D = FnPtr::new(metaloadfn(&mut loadfn, b"glCompressedTextureSubImage1D\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TexStorage1D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexStorage1D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexStorage1D = FnPtr::new(metaloadfn(&mut loadfn, b"glTexStorage1D\0", &[b"glTexStorage1DEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribI4usv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribI4usv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribI4usv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribI4usv\0", &[b"glVertexAttribI4usvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod IsRenderbuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::IsRenderbuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::IsRenderbuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glIsRenderbuffer\0", &[b"glIsRenderbufferEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribP1ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribP1ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribP1ui = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribP1ui\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform3uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform3uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform3uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform3uiv\0", &[b"glUniform3uivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniformMatrix4x3fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniformMatrix4x3fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniformMatrix4x3fv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniformMatrix4x3fv\0", &[b"glProgramUniformMatrix4x3fvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetUniformIndices {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetUniformIndices.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetUniformIndices = FnPtr::new(metaloadfn(&mut loadfn, b"glGetUniformIndices\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GenSamplers {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GenSamplers.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GenSamplers = FnPtr::new(metaloadfn(&mut loadfn, b"glGenSamplers\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniformMatrix4fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniformMatrix4fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniformMatrix4fv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniformMatrix4fv\0", &[b"glProgramUniformMatrix4fvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexArrayBindingDivisor {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexArrayBindingDivisor.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexArrayBindingDivisor = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexArrayBindingDivisor\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexP2uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexP2uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexP2uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexP2uiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib4s {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib4s.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib4s = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib4s\0", &[b"glVertexAttrib4sARB\0", b"glVertexAttrib4sNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DeleteTextures {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DeleteTextures.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DeleteTextures = FnPtr::new(metaloadfn(&mut loadfn, b"glDeleteTextures\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BindImageTextures {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindImageTextures.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindImageTextures = FnPtr::new(metaloadfn(&mut loadfn, b"glBindImageTextures\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod WaitSync {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::WaitSync.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::WaitSync = FnPtr::new(metaloadfn(&mut loadfn, b"glWaitSync\0", &[b"glWaitSyncAPPLE\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod BindVertexArray {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindVertexArray.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindVertexArray = FnPtr::new(metaloadfn(&mut loadfn, b"glBindVertexArray\0", &[b"glBindVertexArrayOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetActiveAttrib {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetActiveAttrib.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetActiveAttrib = FnPtr::new(metaloadfn(&mut loadfn, b"glGetActiveAttrib\0", &[b"glGetActiveAttribARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TextureStorage2DMultisample {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TextureStorage2DMultisample.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TextureStorage2DMultisample = FnPtr::new(metaloadfn(&mut loadfn, b"glTextureStorage2DMultisample\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DebugMessageInsert {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DebugMessageInsert.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DebugMessageInsert = FnPtr::new(metaloadfn(&mut loadfn, b"glDebugMessageInsert\0", &[b"glDebugMessageInsertARB\0", b"glDebugMessageInsertKHR\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DeleteTransformFeedbacks {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DeleteTransformFeedbacks.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DeleteTransformFeedbacks = FnPtr::new(metaloadfn(&mut loadfn, b"glDeleteTransformFeedbacks\0", &[b"glDeleteTransformFeedbacksNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TextureSubImage1D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TextureSubImage1D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TextureSubImage1D = FnPtr::new(metaloadfn(&mut loadfn, b"glTextureSubImage1D\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribL1dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribL1dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribL1dv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribL1dv\0", &[b"glVertexAttribL1dvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib1fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib1fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib1fv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib1fv\0", &[b"glVertexAttrib1fvARB\0", b"glVertexAttrib1fvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetBufferParameteri64v {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetBufferParameteri64v.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetBufferParameteri64v = FnPtr::new(metaloadfn(&mut loadfn, b"glGetBufferParameteri64v\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DeleteRenderbuffers {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DeleteRenderbuffers.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DeleteRenderbuffers = FnPtr::new(metaloadfn(&mut loadfn, b"glDeleteRenderbuffers\0", &[b"glDeleteRenderbuffersEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetRenderbufferParameteriv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetRenderbufferParameteriv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetRenderbufferParameteriv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetRenderbufferParameteriv\0", &[b"glGetRenderbufferParameterivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TextureParameterfv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TextureParameterfv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TextureParameterfv = FnPtr::new(metaloadfn(&mut loadfn, b"glTextureParameterfv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TexBufferRange {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexBufferRange.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexBufferRange = FnPtr::new(metaloadfn(&mut loadfn, b"glTexBufferRange\0", &[b"glTexBufferRangeEXT\0", b"glTexBufferRangeOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod NamedBufferData {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::NamedBufferData.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::NamedBufferData = FnPtr::new(metaloadfn(&mut loadfn, b"glNamedBufferData\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod PixelStorei {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::PixelStorei.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::PixelStorei = FnPtr::new(metaloadfn(&mut loadfn, b"glPixelStorei\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetActiveSubroutineUniformName {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetActiveSubroutineUniformName.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetActiveSubroutineUniformName = FnPtr::new(metaloadfn(&mut loadfn, b"glGetActiveSubroutineUniformName\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BlendEquation {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BlendEquation.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BlendEquation = FnPtr::new(metaloadfn(&mut loadfn, b"glBlendEquation\0", &[b"glBlendEquationEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod BufferData {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BufferData.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BufferData = FnPtr::new(metaloadfn(&mut loadfn, b"glBufferData\0", &[b"glBufferDataARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod CompressedTexSubImage2D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CompressedTexSubImage2D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CompressedTexSubImage2D = FnPtr::new(metaloadfn(&mut loadfn, b"glCompressedTexSubImage2D\0", &[b"glCompressedTexSubImage2DARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod FramebufferTexture3D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::FramebufferTexture3D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::FramebufferTexture3D = FnPtr::new(metaloadfn(&mut loadfn, b"glFramebufferTexture3D\0", &[b"glFramebufferTexture3DEXT\0", b"glFramebufferTexture3DOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniformMatrix4x3dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniformMatrix4x3dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniformMatrix4x3dv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniformMatrix4x3dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetnCompressedTexImage {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetnCompressedTexImage.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetnCompressedTexImage = FnPtr::new(metaloadfn(&mut loadfn, b"glGetnCompressedTexImage\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetProgramStageiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetProgramStageiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetProgramStageiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetProgramStageiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ClampColor {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ClampColor.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ClampColor = FnPtr::new(metaloadfn(&mut loadfn, b"glClampColor\0", &[b"glClampColorARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ValidateProgramPipeline {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ValidateProgramPipeline.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ValidateProgramPipeline = FnPtr::new(metaloadfn(&mut loadfn, b"glValidateProgramPipeline\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetVertexAttribfv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetVertexAttribfv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetVertexAttribfv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetVertexAttribfv\0", &[b"glGetVertexAttribfvARB\0", b"glGetVertexAttribfvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniformMatrix2x4dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniformMatrix2x4dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniformMatrix2x4dv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniformMatrix2x4dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod UniformMatrix4x3fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UniformMatrix4x3fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UniformMatrix4x3fv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniformMatrix4x3fv\0", &[b"glUniformMatrix4x3fvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod MultiTexCoordP2uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::MultiTexCoordP2uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::MultiTexCoordP2uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glMultiTexCoordP2uiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DeleteShader {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DeleteShader.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DeleteShader = FnPtr::new(metaloadfn(&mut loadfn, b"glDeleteShader\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod NamedFramebufferRenderbuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::NamedFramebufferRenderbuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::NamedFramebufferRenderbuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glNamedFramebufferRenderbuffer\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetAttribLocation {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetAttribLocation.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetAttribLocation = FnPtr::new(metaloadfn(&mut loadfn, b"glGetAttribLocation\0", &[b"glGetAttribLocationARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetInteger64i_v {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetInteger64i_v.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetInteger64i_v = FnPtr::new(metaloadfn(&mut loadfn, b"glGetInteger64i_v\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod CopyTexImage1D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CopyTexImage1D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CopyTexImage1D = FnPtr::new(metaloadfn(&mut loadfn, b"glCopyTexImage1D\0", &[b"glCopyTexImage1DEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib2f {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib2f.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib2f = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib2f\0", &[b"glVertexAttrib2fARB\0", b"glVertexAttrib2fNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribI4iv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribI4iv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribI4iv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribI4iv\0", &[b"glVertexAttribI4ivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ClearDepthf {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ClearDepthf.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ClearDepthf = FnPtr::new(metaloadfn(&mut loadfn, b"glClearDepthf\0", &[b"glClearDepthfOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod UniformMatrix2x3dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UniformMatrix2x3dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UniformMatrix2x3dv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniformMatrix2x3dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetTexLevelParameteriv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetTexLevelParameteriv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetTexLevelParameteriv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetTexLevelParameteriv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ReadnPixels {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ReadnPixels.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ReadnPixels = FnPtr::new(metaloadfn(&mut loadfn, b"glReadnPixels\0", &[b"glReadnPixelsARB\0", b"glReadnPixelsEXT\0", b"glReadnPixelsKHR\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod LinkProgram {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::LinkProgram.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::LinkProgram = FnPtr::new(metaloadfn(&mut loadfn, b"glLinkProgram\0", &[b"glLinkProgramARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod EnableVertexArrayAttrib {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::EnableVertexArrayAttrib.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::EnableVertexArrayAttrib = FnPtr::new(metaloadfn(&mut loadfn, b"glEnableVertexArrayAttrib\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribLPointer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribLPointer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribLPointer = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribLPointer\0", &[b"glVertexAttribLPointerEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TextureView {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TextureView.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TextureView = FnPtr::new(metaloadfn(&mut loadfn, b"glTextureView\0", &[b"glTextureViewEXT\0", b"glTextureViewOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetActiveSubroutineUniformiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetActiveSubroutineUniformiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetActiveSubroutineUniformiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetActiveSubroutineUniformiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetQueryBufferObjectui64v {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetQueryBufferObjectui64v.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetQueryBufferObjectui64v = FnPtr::new(metaloadfn(&mut loadfn, b"glGetQueryBufferObjectui64v\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod CompileShader {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CompileShader.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CompileShader = FnPtr::new(metaloadfn(&mut loadfn, b"glCompileShader\0", &[b"glCompileShaderARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform2fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform2fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform2fv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform2fv\0", &[b"glUniform2fvARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TexSubImage3D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexSubImage3D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexSubImage3D = FnPtr::new(metaloadfn(&mut loadfn, b"glTexSubImage3D\0", &[b"glTexSubImage3DEXT\0", b"glTexSubImage3DOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TexImage2DMultisample {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexImage2DMultisample.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexImage2DMultisample = FnPtr::new(metaloadfn(&mut loadfn, b"glTexImage2DMultisample\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform4d {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform4d.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform4d = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform4d\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetTransformFeedbacki64_v {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetTransformFeedbacki64_v.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetTransformFeedbacki64_v = FnPtr::new(metaloadfn(&mut loadfn, b"glGetTransformFeedbacki64_v\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniformMatrix3x2fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniformMatrix3x2fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniformMatrix3x2fv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniformMatrix3x2fv\0", &[b"glProgramUniformMatrix3x2fvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniformMatrix2fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniformMatrix2fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniformMatrix2fv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniformMatrix2fv\0", &[b"glProgramUniformMatrix2fvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod CreateVertexArrays {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CreateVertexArrays.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CreateVertexArrays = FnPtr::new(metaloadfn(&mut loadfn, b"glCreateVertexArrays\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BindBufferBase {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindBufferBase.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindBufferBase = FnPtr::new(metaloadfn(&mut loadfn, b"glBindBufferBase\0", &[b"glBindBufferBaseEXT\0", b"glBindBufferBaseNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetSamplerParameteriv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetSamplerParameteriv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetSamplerParameteriv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetSamplerParameteriv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ReadPixels {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ReadPixels.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ReadPixels = FnPtr::new(metaloadfn(&mut loadfn, b"glReadPixels\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribLFormat {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribLFormat.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribLFormat = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribLFormat\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetQueryBufferObjectuiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetQueryBufferObjectuiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetQueryBufferObjectuiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetQueryBufferObjectuiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod FramebufferTexture {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::FramebufferTexture.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::FramebufferTexture = FnPtr::new(metaloadfn(&mut loadfn, b"glFramebufferTexture\0", &[b"glFramebufferTextureARB\0", b"glFramebufferTextureEXT\0", b"glFramebufferTextureOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TexParameterf {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexParameterf.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexParameterf = FnPtr::new(metaloadfn(&mut loadfn, b"glTexParameterf\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod FramebufferParameteri {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::FramebufferParameteri.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::FramebufferParameteri = FnPtr::new(metaloadfn(&mut loadfn, b"glFramebufferParameteri\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TextureParameterIiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TextureParameterIiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TextureParameterIiv = FnPtr::new(metaloadfn(&mut loadfn, b"glTextureParameterIiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BindBuffersBase {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindBuffersBase.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindBuffersBase = FnPtr::new(metaloadfn(&mut loadfn, b"glBindBuffersBase\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TexStorage3DMultisample {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexStorage3DMultisample.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexStorage3DMultisample = FnPtr::new(metaloadfn(&mut loadfn, b"glTexStorage3DMultisample\0", &[b"glTexStorage3DMultisampleOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribI4i {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribI4i.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribI4i = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribI4i\0", &[b"glVertexAttribI4iEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DrawRangeElements {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DrawRangeElements.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DrawRangeElements = FnPtr::new(metaloadfn(&mut loadfn, b"glDrawRangeElements\0", &[b"glDrawRangeElementsEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TexImage3D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexImage3D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexImage3D = FnPtr::new(metaloadfn(&mut loadfn, b"glTexImage3D\0", &[b"glTexImage3DEXT\0", b"glTexImage3DOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TextureStorage2D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TextureStorage2D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TextureStorage2D = FnPtr::new(metaloadfn(&mut loadfn, b"glTextureStorage2D\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TransformFeedbackBufferRange {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TransformFeedbackBufferRange.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TransformFeedbackBufferRange = FnPtr::new(metaloadfn(&mut loadfn, b"glTransformFeedbackBufferRange\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexP4ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexP4ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexP4ui = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexP4ui\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BlendFuncSeparate {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BlendFuncSeparate.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BlendFuncSeparate = FnPtr::new(metaloadfn(&mut loadfn, b"glBlendFuncSeparate\0", &[b"glBlendFuncSeparateEXT\0", b"glBlendFuncSeparateINGR\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform4fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform4fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform4fv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform4fv\0", &[b"glUniform4fvARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod CreateShaderProgramv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CreateShaderProgramv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CreateShaderProgramv = FnPtr::new(metaloadfn(&mut loadfn, b"glCreateShaderProgramv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BindVertexBuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindVertexBuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindVertexBuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glBindVertexBuffer\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TexStorage2DMultisample {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexStorage2DMultisample.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexStorage2DMultisample = FnPtr::new(metaloadfn(&mut loadfn, b"glTexStorage2DMultisample\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ShaderStorageBlockBinding {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ShaderStorageBlockBinding.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ShaderStorageBlockBinding = FnPtr::new(metaloadfn(&mut loadfn, b"glShaderStorageBlockBinding\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod NamedRenderbufferStorageMultisample {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::NamedRenderbufferStorageMultisample.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::NamedRenderbufferStorageMultisample = FnPtr::new(metaloadfn(&mut loadfn, b"glNamedRenderbufferStorageMultisample\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetProgramResourceiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetProgramResourceiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetProgramResourceiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetProgramResourceiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod EnableVertexAttribArray {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::EnableVertexAttribArray.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::EnableVertexAttribArray = FnPtr::new(metaloadfn(&mut loadfn, b"glEnableVertexAttribArray\0", &[b"glEnableVertexAttribArrayARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TexCoordP2ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexCoordP2ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexCoordP2ui = FnPtr::new(metaloadfn(&mut loadfn, b"glTexCoordP2ui\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TexStorage2D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexStorage2D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexStorage2D = FnPtr::new(metaloadfn(&mut loadfn, b"glTexStorage2D\0", &[b"glTexStorage2DEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib4Niv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib4Niv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib4Niv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib4Niv\0", &[b"glVertexAttrib4NivARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexArrayVertexBuffers {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexArrayVertexBuffers.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexArrayVertexBuffers = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexArrayVertexBuffers\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform2iv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform2iv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform2iv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform2iv\0", &[b"glProgramUniform2ivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod UniformMatrix2fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UniformMatrix2fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UniformMatrix2fv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniformMatrix2fv\0", &[b"glUniformMatrix2fvARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetnMinmax {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetnMinmax.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetnMinmax = FnPtr::new(metaloadfn(&mut loadfn, b"glGetnMinmax\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod UniformMatrix2x4fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UniformMatrix2x4fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UniformMatrix2x4fv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniformMatrix2x4fv\0", &[b"glUniformMatrix2x4fvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Finish {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Finish.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Finish = FnPtr::new(metaloadfn(&mut loadfn, b"glFinish\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod MultiDrawElementsIndirect {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::MultiDrawElementsIndirect.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::MultiDrawElementsIndirect = FnPtr::new(metaloadfn(&mut loadfn, b"glMultiDrawElementsIndirect\0", &[b"glMultiDrawElementsIndirectAMD\0", b"glMultiDrawElementsIndirectEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DebugMessageCallback {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DebugMessageCallback.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DebugMessageCallback = FnPtr::new(metaloadfn(&mut loadfn, b"glDebugMessageCallback\0", &[b"glDebugMessageCallbackARB\0", b"glDebugMessageCallbackKHR\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetnUniformfv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetnUniformfv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetnUniformfv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetnUniformfv\0", &[b"glGetnUniformfvKHR\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod SamplerParameterIuiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::SamplerParameterIuiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::SamplerParameterIuiv = FnPtr::new(metaloadfn(&mut loadfn, b"glSamplerParameterIuiv\0", &[b"glSamplerParameterIuivEXT\0", b"glSamplerParameterIuivOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod CopyTexImage2D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CopyTexImage2D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CopyTexImage2D = FnPtr::new(metaloadfn(&mut loadfn, b"glCopyTexImage2D\0", &[b"glCopyTexImage2DEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod UniformMatrix2x4dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UniformMatrix2x4dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UniformMatrix2x4dv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniformMatrix2x4dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod FramebufferTexture2D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::FramebufferTexture2D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::FramebufferTexture2D = FnPtr::new(metaloadfn(&mut loadfn, b"glFramebufferTexture2D\0", &[b"glFramebufferTexture2DEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribFormat {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribFormat.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribFormat = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribFormat\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ClearNamedBufferData {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ClearNamedBufferData.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ClearNamedBufferData = FnPtr::new(metaloadfn(&mut loadfn, b"glClearNamedBufferData\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod CheckFramebufferStatus {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CheckFramebufferStatus.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CheckFramebufferStatus = FnPtr::new(metaloadfn(&mut loadfn, b"glCheckFramebufferStatus\0", &[b"glCheckFramebufferStatusEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribI2uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribI2uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribI2uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribI2uiv\0", &[b"glVertexAttribI2uivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod BufferStorage {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BufferStorage.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BufferStorage = FnPtr::new(metaloadfn(&mut loadfn, b"glBufferStorage\0", &[b"glBufferStorageEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod PointParameterf {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::PointParameterf.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::PointParameterf = FnPtr::new(metaloadfn(&mut loadfn, b"glPointParameterf\0", &[b"glPointParameterfARB\0", b"glPointParameterfEXT\0", b"glPointParameterfSGIS\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetnColorTable {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetnColorTable.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetnColorTable = FnPtr::new(metaloadfn(&mut loadfn, b"glGetnColorTable\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetnTexImage {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetnTexImage.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetnTexImage = FnPtr::new(metaloadfn(&mut loadfn, b"glGetnTexImage\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DeleteQueries {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DeleteQueries.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DeleteQueries = FnPtr::new(metaloadfn(&mut loadfn, b"glDeleteQueries\0", &[b"glDeleteQueriesARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod CreateTransformFeedbacks {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CreateTransformFeedbacks.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CreateTransformFeedbacks = FnPtr::new(metaloadfn(&mut loadfn, b"glCreateTransformFeedbacks\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform3fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform3fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform3fv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform3fv\0", &[b"glProgramUniform3fvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TransformFeedbackBufferBase {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TransformFeedbackBufferBase.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TransformFeedbackBufferBase = FnPtr::new(metaloadfn(&mut loadfn, b"glTransformFeedbackBufferBase\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod UnmapNamedBuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UnmapNamedBuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UnmapNamedBuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glUnmapNamedBuffer\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetUniformdv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetUniformdv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetUniformdv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetUniformdv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod CompressedTexImage3D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CompressedTexImage3D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CompressedTexImage3D = FnPtr::new(metaloadfn(&mut loadfn, b"glCompressedTexImage3D\0", &[b"glCompressedTexImage3DARB\0", b"glCompressedTexImage3DOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DrawElementsInstanced {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DrawElementsInstanced.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DrawElementsInstanced = FnPtr::new(metaloadfn(&mut loadfn, b"glDrawElementsInstanced\0", &[b"glDrawElementsInstancedANGLE\0", b"glDrawElementsInstancedARB\0", b"glDrawElementsInstancedEXT\0", b"glDrawElementsInstancedNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GenQueries {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GenQueries.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GenQueries = FnPtr::new(metaloadfn(&mut loadfn, b"glGenQueries\0", &[b"glGenQueriesARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod CopyTexSubImage2D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CopyTexSubImage2D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CopyTexSubImage2D = FnPtr::new(metaloadfn(&mut loadfn, b"glCopyTexSubImage2D\0", &[b"glCopyTexSubImage2DEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DrawArraysInstancedBaseInstance {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DrawArraysInstancedBaseInstance.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DrawArraysInstancedBaseInstance = FnPtr::new(metaloadfn(&mut loadfn, b"glDrawArraysInstancedBaseInstance\0", &[b"glDrawArraysInstancedBaseInstanceEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TexCoordP4ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexCoordP4ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexCoordP4ui = FnPtr::new(metaloadfn(&mut loadfn, b"glTexCoordP4ui\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribP2ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribP2ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribP2ui = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribP2ui\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib4dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib4dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib4dv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib4dv\0", &[b"glVertexAttrib4dvARB\0", b"glVertexAttrib4dvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ColorP4uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ColorP4uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ColorP4uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glColorP4uiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetActiveSubroutineName {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetActiveSubroutineName.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetActiveSubroutineName = FnPtr::new(metaloadfn(&mut loadfn, b"glGetActiveSubroutineName\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TexCoordP4uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexCoordP4uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexCoordP4uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glTexCoordP4uiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform3f {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform3f.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform3f = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform3f\0", &[b"glProgramUniform3fEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform1iv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform1iv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform1iv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform1iv\0", &[b"glProgramUniform1ivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib1f {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib1f.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib1f = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib1f\0", &[b"glVertexAttrib1fARB\0", b"glVertexAttrib1fNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform1d {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform1d.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform1d = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform1d\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform2iv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform2iv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform2iv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform2iv\0", &[b"glUniform2ivARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod CompressedTexImage2D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CompressedTexImage2D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CompressedTexImage2D = FnPtr::new(metaloadfn(&mut loadfn, b"glCompressedTexImage2D\0", &[b"glCompressedTexImage2DARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DrawBuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DrawBuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DrawBuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glDrawBuffer\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ClearNamedFramebufferiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ClearNamedFramebufferiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ClearNamedFramebufferiv = FnPtr::new(metaloadfn(&mut loadfn, b"glClearNamedFramebufferiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod Hint {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Hint.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Hint = FnPtr::new(metaloadfn(&mut loadfn, b"glHint\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DeleteBuffers {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DeleteBuffers.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DeleteBuffers = FnPtr::new(metaloadfn(&mut loadfn, b"glDeleteBuffers\0", &[b"glDeleteBuffersARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexArrayAttribFormat {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexArrayAttribFormat.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexArrayAttribFormat = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexArrayAttribFormat\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GenTransformFeedbacks {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GenTransformFeedbacks.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GenTransformFeedbacks = FnPtr::new(metaloadfn(&mut loadfn, b"glGenTransformFeedbacks\0", &[b"glGenTransformFeedbacksNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod IsBuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::IsBuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::IsBuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glIsBuffer\0", &[b"glIsBufferARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DrawElementsInstancedBaseVertex {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DrawElementsInstancedBaseVertex.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DrawElementsInstancedBaseVertex = FnPtr::new(metaloadfn(&mut loadfn, b"glDrawElementsInstancedBaseVertex\0", &[b"glDrawElementsInstancedBaseVertexEXT\0", b"glDrawElementsInstancedBaseVertexOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform3i {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform3i.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform3i = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform3i\0", &[b"glUniform3iARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetProgramBinary {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetProgramBinary.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetProgramBinary = FnPtr::new(metaloadfn(&mut loadfn, b"glGetProgramBinary\0", &[b"glGetProgramBinaryOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetVertexAttribPointerv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetVertexAttribPointerv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetVertexAttribPointerv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetVertexAttribPointerv\0", &[b"glGetVertexAttribPointervARB\0", b"glGetVertexAttribPointervNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetActiveUniformBlockiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetActiveUniformBlockiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetActiveUniformBlockiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetActiveUniformBlockiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform3dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform3dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform3dv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform3dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TexStorage3D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexStorage3D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexStorage3D = FnPtr::new(metaloadfn(&mut loadfn, b"glTexStorage3D\0", &[b"glTexStorage3DEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetQueryBufferObjectiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetQueryBufferObjectiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetQueryBufferObjectiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetQueryBufferObjectiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DepthRangef {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DepthRangef.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DepthRangef = FnPtr::new(metaloadfn(&mut loadfn, b"glDepthRangef\0", &[b"glDepthRangefOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DeleteProgramPipelines {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DeleteProgramPipelines.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DeleteProgramPipelines = FnPtr::new(metaloadfn(&mut loadfn, b"glDeleteProgramPipelines\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib4Nusv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib4Nusv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib4Nusv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib4Nusv\0", &[b"glVertexAttrib4NusvARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ClearTexSubImage {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ClearTexSubImage.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ClearTexSubImage = FnPtr::new(metaloadfn(&mut loadfn, b"glClearTexSubImage\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod MultiTexCoordP3ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::MultiTexCoordP3ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::MultiTexCoordP3ui = FnPtr::new(metaloadfn(&mut loadfn, b"glMultiTexCoordP3ui\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform2f {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform2f.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform2f = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform2f\0", &[b"glProgramUniform2fEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod IsQuery {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::IsQuery.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::IsQuery = FnPtr::new(metaloadfn(&mut loadfn, b"glIsQuery\0", &[b"glIsQueryARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetnSeparableFilter {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetnSeparableFilter.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetnSeparableFilter = FnPtr::new(metaloadfn(&mut loadfn, b"glGetnSeparableFilter\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetProgramInfoLog {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetProgramInfoLog.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetProgramInfoLog = FnPtr::new(metaloadfn(&mut loadfn, b"glGetProgramInfoLog\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BindRenderbuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindRenderbuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindRenderbuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glBindRenderbuffer\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod RenderbufferStorage {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::RenderbufferStorage.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::RenderbufferStorage = FnPtr::new(metaloadfn(&mut loadfn, b"glRenderbufferStorage\0", &[b"glRenderbufferStorageEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DebugMessageControl {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DebugMessageControl.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DebugMessageControl = FnPtr::new(metaloadfn(&mut loadfn, b"glDebugMessageControl\0", &[b"glDebugMessageControlARB\0", b"glDebugMessageControlKHR\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetnUniformuiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetnUniformuiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetnUniformuiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetnUniformuiv\0", &[b"glGetnUniformuivKHR\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod PolygonOffset {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::PolygonOffset.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::PolygonOffset = FnPtr::new(metaloadfn(&mut loadfn, b"glPolygonOffset\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod MultiDrawElementsBaseVertex {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::MultiDrawElementsBaseVertex.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::MultiDrawElementsBaseVertex = FnPtr::new(metaloadfn(&mut loadfn, b"glMultiDrawElementsBaseVertex\0", &[b"glMultiDrawElementsBaseVertexEXT\0", b"glMultiDrawElementsBaseVertexOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod NamedFramebufferDrawBuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::NamedFramebufferDrawBuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::NamedFramebufferDrawBuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glNamedFramebufferDrawBuffer\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib2d {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib2d.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib2d = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib2d\0", &[b"glVertexAttrib2dARB\0", b"glVertexAttrib2dNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod CreateTextures {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CreateTextures.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CreateTextures = FnPtr::new(metaloadfn(&mut loadfn, b"glCreateTextures\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetUniformSubroutineuiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetUniformSubroutineuiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetUniformSubroutineuiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetUniformSubroutineuiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ClearNamedFramebufferfv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ClearNamedFramebufferfv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ClearNamedFramebufferfv = FnPtr::new(metaloadfn(&mut loadfn, b"glClearNamedFramebufferfv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod CreateRenderbuffers {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CreateRenderbuffers.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CreateRenderbuffers = FnPtr::new(metaloadfn(&mut loadfn, b"glCreateRenderbuffers\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod IsSampler {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::IsSampler.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::IsSampler = FnPtr::new(metaloadfn(&mut loadfn, b"glIsSampler\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod MultiTexCoordP4uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::MultiTexCoordP4uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::MultiTexCoordP4uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glMultiTexCoordP4uiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetSynciv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetSynciv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetSynciv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetSynciv\0", &[b"glGetSyncivAPPLE\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod UnmapBuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UnmapBuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UnmapBuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glUnmapBuffer\0", &[b"glUnmapBufferARB\0", b"glUnmapBufferOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetBufferPointerv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetBufferPointerv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetBufferPointerv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetBufferPointerv\0", &[b"glGetBufferPointervARB\0", b"glGetBufferPointervOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GenVertexArrays {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GenVertexArrays.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GenVertexArrays = FnPtr::new(metaloadfn(&mut loadfn, b"glGenVertexArrays\0", &[b"glGenVertexArraysAPPLE\0", b"glGenVertexArraysOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod SampleMaski {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::SampleMaski.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::SampleMaski = FnPtr::new(metaloadfn(&mut loadfn, b"glSampleMaski\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ClearStencil {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ClearStencil.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ClearStencil = FnPtr::new(metaloadfn(&mut loadfn, b"glClearStencil\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BlendFuncSeparatei {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BlendFuncSeparatei.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BlendFuncSeparatei = FnPtr::new(metaloadfn(&mut loadfn, b"glBlendFuncSeparatei\0", &[b"glBlendFuncSeparateIndexedAMD\0", b"glBlendFuncSeparateiARB\0", b"glBlendFuncSeparateiEXT\0", b"glBlendFuncSeparateiOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib4Nub {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib4Nub.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib4Nub = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib4Nub\0", &[b"glVertexAttrib4NubARB\0", b"glVertexAttrib4ubNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ShaderBinary {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ShaderBinary.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ShaderBinary = FnPtr::new(metaloadfn(&mut loadfn, b"glShaderBinary\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TextureSubImage3D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TextureSubImage3D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TextureSubImage3D = FnPtr::new(metaloadfn(&mut loadfn, b"glTextureSubImage3D\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetUniformiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetUniformiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetUniformiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetUniformiv\0", &[b"glGetUniformivARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform1uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform1uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform1uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform1uiv\0", &[b"glUniform1uivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribI4sv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribI4sv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribI4sv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribI4sv\0", &[b"glVertexAttribI4svEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod BlitNamedFramebuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BlitNamedFramebuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BlitNamedFramebuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glBlitNamedFramebuffer\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetAttachedShaders {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetAttachedShaders.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetAttachedShaders = FnPtr::new(metaloadfn(&mut loadfn, b"glGetAttachedShaders\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod InvalidateBufferSubData {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::InvalidateBufferSubData.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::InvalidateBufferSubData = FnPtr::new(metaloadfn(&mut loadfn, b"glInvalidateBufferSubData\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod InvalidateFramebuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::InvalidateFramebuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::InvalidateFramebuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glInvalidateFramebuffer\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TextureStorage1D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TextureStorage1D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TextureStorage1D = FnPtr::new(metaloadfn(&mut loadfn, b"glTextureStorage1D\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod FramebufferTexture1D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::FramebufferTexture1D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::FramebufferTexture1D = FnPtr::new(metaloadfn(&mut loadfn, b"glFramebufferTexture1D\0", &[b"glFramebufferTexture1DEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetnMapiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetnMapiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetnMapiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetnMapiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetQueryObjectuiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetQueryObjectuiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetQueryObjectuiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetQueryObjectuiv\0", &[b"glGetQueryObjectuivARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DetachShader {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DetachShader.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DetachShader = FnPtr::new(metaloadfn(&mut loadfn, b"glDetachShader\0", &[b"glDetachObjectARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetActiveUniformBlockName {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetActiveUniformBlockName.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetActiveUniformBlockName = FnPtr::new(metaloadfn(&mut loadfn, b"glGetActiveUniformBlockName\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod IsSync {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::IsSync.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::IsSync = FnPtr::new(metaloadfn(&mut loadfn, b"glIsSync\0", &[b"glIsSyncAPPLE\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetBooleanv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetBooleanv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetBooleanv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetBooleanv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod QueryCounter {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::QueryCounter.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::QueryCounter = FnPtr::new(metaloadfn(&mut loadfn, b"glQueryCounter\0", &[b"glQueryCounterEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod InvalidateNamedFramebufferData {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::InvalidateNamedFramebufferData.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::InvalidateNamedFramebufferData = FnPtr::new(metaloadfn(&mut loadfn, b"glInvalidateNamedFramebufferData\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TexSubImage1D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexSubImage1D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexSubImage1D = FnPtr::new(metaloadfn(&mut loadfn, b"glTexSubImage1D\0", &[b"glTexSubImage1DEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod CopyTextureSubImage1D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CopyTextureSubImage1D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CopyTextureSubImage1D = FnPtr::new(metaloadfn(&mut loadfn, b"glCopyTextureSubImage1D\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetIntegeri_v {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetIntegeri_v.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetIntegeri_v = FnPtr::new(metaloadfn(&mut loadfn, b"glGetIntegeri_v\0", &[b"glGetIntegerIndexedvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform3fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform3fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform3fv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform3fv\0", &[b"glUniform3fvARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib1dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib1dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib1dv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib1dv\0", &[b"glVertexAttrib1dvARB\0", b"glVertexAttrib1dvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Disablei {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Disablei.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Disablei = FnPtr::new(metaloadfn(&mut loadfn, b"glDisablei\0", &[b"glDisableIndexedEXT\0", b"glDisableiEXT\0", b"glDisableiNV\0", b"glDisableiOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ViewportIndexedfv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ViewportIndexedfv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ViewportIndexedfv = FnPtr::new(metaloadfn(&mut loadfn, b"glViewportIndexedfv\0", &[b"glViewportIndexedfvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod PatchParameteri {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::PatchParameteri.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::PatchParameteri = FnPtr::new(metaloadfn(&mut loadfn, b"glPatchParameteri\0", &[b"glPatchParameteriEXT\0", b"glPatchParameteriOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribI2i {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribI2i.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribI2i = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribI2i\0", &[b"glVertexAttribI2iEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform1i {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform1i.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform1i = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform1i\0", &[b"glUniform1iARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod UniformMatrix3x4dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UniformMatrix3x4dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UniformMatrix3x4dv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniformMatrix3x4dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribL4dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribL4dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribL4dv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribL4dv\0", &[b"glVertexAttribL4dvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod SamplerParameterfv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::SamplerParameterfv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::SamplerParameterfv = FnPtr::new(metaloadfn(&mut loadfn, b"glSamplerParameterfv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib3dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib3dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib3dv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib3dv\0", &[b"glVertexAttrib3dvARB\0", b"glVertexAttrib3dvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ColorMask {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ColorMask.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ColorMask = FnPtr::new(metaloadfn(&mut loadfn, b"glColorMask\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetUniformBlockIndex {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetUniformBlockIndex.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetUniformBlockIndex = FnPtr::new(metaloadfn(&mut loadfn, b"glGetUniformBlockIndex\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TextureParameterf {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TextureParameterf.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TextureParameterf = FnPtr::new(metaloadfn(&mut loadfn, b"glTextureParameterf\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetMultisamplefv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetMultisamplefv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetMultisamplefv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetMultisamplefv\0", &[b"glGetMultisamplefvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramParameteri {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramParameteri.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramParameteri = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramParameteri\0", &[b"glProgramParameteriARB\0", b"glProgramParameteriEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod MapNamedBuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::MapNamedBuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::MapNamedBuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glMapNamedBuffer\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TextureBuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TextureBuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TextureBuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glTextureBuffer\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod NormalP3uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::NormalP3uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::NormalP3uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glNormalP3uiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BlendFunci {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BlendFunci.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BlendFunci = FnPtr::new(metaloadfn(&mut loadfn, b"glBlendFunci\0", &[b"glBlendFuncIndexedAMD\0", b"glBlendFunciARB\0", b"glBlendFunciEXT\0", b"glBlendFunciOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib2s {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib2s.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib2s = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib2s\0", &[b"glVertexAttrib2sARB\0", b"glVertexAttrib2sNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribP3ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribP3ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribP3ui = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribP3ui\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetNamedFramebufferAttachmentParameteriv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetNamedFramebufferAttachmentParameteriv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetNamedFramebufferAttachmentParameteriv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetNamedFramebufferAttachmentParameteriv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod NamedRenderbufferStorage {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::NamedRenderbufferStorage.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::NamedRenderbufferStorage = FnPtr::new(metaloadfn(&mut loadfn, b"glNamedRenderbufferStorage\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform1fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform1fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform1fv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform1fv\0", &[b"glProgramUniform1fvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod BlendEquationSeparate {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BlendEquationSeparate.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BlendEquationSeparate = FnPtr::new(metaloadfn(&mut loadfn, b"glBlendEquationSeparate\0", &[b"glBlendEquationSeparateEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TexBuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexBuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexBuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glTexBuffer\0", &[b"glTexBufferARB\0", b"glTexBufferEXT\0", b"glTexBufferOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TexImage1D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexImage1D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexImage1D = FnPtr::new(metaloadfn(&mut loadfn, b"glTexImage1D\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TexParameterIuiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexParameterIuiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexParameterIuiv = FnPtr::new(metaloadfn(&mut loadfn, b"glTexParameterIuiv\0", &[b"glTexParameterIuivEXT\0", b"glTexParameterIuivOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexP2ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexP2ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexP2ui = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexP2ui\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GenRenderbuffers {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GenRenderbuffers.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GenRenderbuffers = FnPtr::new(metaloadfn(&mut loadfn, b"glGenRenderbuffers\0", &[b"glGenRenderbuffersEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexBindingDivisor {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexBindingDivisor.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexBindingDivisor = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexBindingDivisor\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform2i {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform2i.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform2i = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform2i\0", &[b"glProgramUniform2iEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Enablei {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Enablei.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Enablei = FnPtr::new(metaloadfn(&mut loadfn, b"glEnablei\0", &[b"glEnableIndexedEXT\0", b"glEnableiEXT\0", b"glEnableiNV\0", b"glEnableiOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetnMapfv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetnMapfv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetnMapfv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetnMapfv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod IsEnabledi {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::IsEnabledi.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::IsEnabledi = FnPtr::new(metaloadfn(&mut loadfn, b"glIsEnabledi\0", &[b"glIsEnabledIndexedEXT\0", b"glIsEnablediEXT\0", b"glIsEnablediNV\0", b"glIsEnablediOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod CompressedTextureSubImage3D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CompressedTextureSubImage3D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CompressedTextureSubImage3D = FnPtr::new(metaloadfn(&mut loadfn, b"glCompressedTextureSubImage3D\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetShaderPrecisionFormat {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetShaderPrecisionFormat.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetShaderPrecisionFormat = FnPtr::new(metaloadfn(&mut loadfn, b"glGetShaderPrecisionFormat\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetTextureImage {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetTextureImage.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetTextureImage = FnPtr::new(metaloadfn(&mut loadfn, b"glGetTextureImage\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod UniformMatrix3x4fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UniformMatrix3x4fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UniformMatrix3x4fv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniformMatrix3x4fv\0", &[b"glUniformMatrix3x4fvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform2uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform2uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform2uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform2uiv\0", &[b"glUniform2uivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetInternalformati64v {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetInternalformati64v.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetInternalformati64v = FnPtr::new(metaloadfn(&mut loadfn, b"glGetInternalformati64v\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform2dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform2dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform2dv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform2dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib3s {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib3s.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib3s = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib3s\0", &[b"glVertexAttrib3sARB\0", b"glVertexAttrib3sNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod FlushMappedBufferRange {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::FlushMappedBufferRange.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::FlushMappedBufferRange = FnPtr::new(metaloadfn(&mut loadfn, b"glFlushMappedBufferRange\0", &[b"glFlushMappedBufferRangeAPPLE\0", b"glFlushMappedBufferRangeEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod InvalidateTexImage {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::InvalidateTexImage.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::InvalidateTexImage = FnPtr::new(metaloadfn(&mut loadfn, b"glInvalidateTexImage\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetProgramInterfaceiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetProgramInterfaceiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetProgramInterfaceiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetProgramInterfaceiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod CullFace {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CullFace.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CullFace = FnPtr::new(metaloadfn(&mut loadfn, b"glCullFace\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetFramebufferParameteriv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetFramebufferParameteriv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetFramebufferParameteriv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetFramebufferParameteriv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod CreateShader {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CreateShader.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CreateShader = FnPtr::new(metaloadfn(&mut loadfn, b"glCreateShader\0", &[b"glCreateShaderObjectARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniformMatrix3dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniformMatrix3dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniformMatrix3dv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniformMatrix3dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod PointParameterfv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::PointParameterfv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::PointParameterfv = FnPtr::new(metaloadfn(&mut loadfn, b"glPointParameterfv\0", &[b"glPointParameterfvARB\0", b"glPointParameterfvEXT\0", b"glPointParameterfvSGIS\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DrawArraysIndirect {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DrawArraysIndirect.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DrawArraysIndirect = FnPtr::new(metaloadfn(&mut loadfn, b"glDrawArraysIndirect\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod UseProgram {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UseProgram.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UseProgram = FnPtr::new(metaloadfn(&mut loadfn, b"glUseProgram\0", &[b"glUseProgramObjectARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniformMatrix3x2dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniformMatrix3x2dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniformMatrix3x2dv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniformMatrix3x2dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod SampleCoverage {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::SampleCoverage.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::SampleCoverage = FnPtr::new(metaloadfn(&mut loadfn, b"glSampleCoverage\0", &[b"glSampleCoverageARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform3iv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform3iv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform3iv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform3iv\0", &[b"glUniform3ivARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribI3iv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribI3iv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribI3iv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribI3iv\0", &[b"glVertexAttribI3ivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform1dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform1dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform1dv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform1dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BlendEquationSeparatei {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BlendEquationSeparatei.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BlendEquationSeparatei = FnPtr::new(metaloadfn(&mut loadfn, b"glBlendEquationSeparatei\0", &[b"glBlendEquationSeparateIndexedAMD\0", b"glBlendEquationSeparateiARB\0", b"glBlendEquationSeparateiEXT\0", b"glBlendEquationSeparateiOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetFloati_v {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetFloati_v.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetFloati_v = FnPtr::new(metaloadfn(&mut loadfn, b"glGetFloati_v\0", &[b"glGetFloatIndexedvEXT\0", b"glGetFloati_vEXT\0", b"glGetFloati_vNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform4iv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform4iv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform4iv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform4iv\0", &[b"glProgramUniform4ivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod SecondaryColorP3ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::SecondaryColorP3ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::SecondaryColorP3ui = FnPtr::new(metaloadfn(&mut loadfn, b"glSecondaryColorP3ui\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribI1ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribI1ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribI1ui = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribI1ui\0", &[b"glVertexAttribI1uiEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform1iv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform1iv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform1iv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform1iv\0", &[b"glUniform1ivARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetVertexArrayiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetVertexArrayiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetVertexArrayiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetVertexArrayiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod IsProgram {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::IsProgram.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::IsProgram = FnPtr::new(metaloadfn(&mut loadfn, b"glIsProgram\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BindTextureUnit {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindTextureUnit.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindTextureUnit = FnPtr::new(metaloadfn(&mut loadfn, b"glBindTextureUnit\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetnPolygonStipple {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetnPolygonStipple.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetnPolygonStipple = FnPtr::new(metaloadfn(&mut loadfn, b"glGetnPolygonStipple\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetIntegerv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetIntegerv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetIntegerv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetIntegerv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod NamedFramebufferParameteri {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::NamedFramebufferParameteri.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::NamedFramebufferParameteri = FnPtr::new(metaloadfn(&mut loadfn, b"glNamedFramebufferParameteri\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexP3uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexP3uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexP3uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexP3uiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib4usv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib4usv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib4usv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib4usv\0", &[b"glVertexAttrib4usvARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod UniformMatrix2x3fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UniformMatrix2x3fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UniformMatrix2x3fv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniformMatrix2x3fv\0", &[b"glUniformMatrix2x3fvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetnMapdv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetnMapdv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetnMapdv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetnMapdv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TexCoordP1uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexCoordP1uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexCoordP1uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glTexCoordP1uiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform1fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform1fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform1fv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform1fv\0", &[b"glUniform1fvARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetNamedBufferSubData {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetNamedBufferSubData.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetNamedBufferSubData = FnPtr::new(metaloadfn(&mut loadfn, b"glGetNamedBufferSubData\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TransformFeedbackVaryings {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TransformFeedbackVaryings.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TransformFeedbackVaryings = FnPtr::new(metaloadfn(&mut loadfn, b"glTransformFeedbackVaryings\0", &[b"glTransformFeedbackVaryingsEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod InvalidateNamedFramebufferSubData {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::InvalidateNamedFramebufferSubData.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::InvalidateNamedFramebufferSubData = FnPtr::new(metaloadfn(&mut loadfn, b"glInvalidateNamedFramebufferSubData\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod PointParameteri {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::PointParameteri.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::PointParameteri = FnPtr::new(metaloadfn(&mut loadfn, b"glPointParameteri\0", &[b"glPointParameteriNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetTexParameterfv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetTexParameterfv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetTexParameterfv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetTexParameterfv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod IsTransformFeedback {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::IsTransformFeedback.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::IsTransformFeedback = FnPtr::new(metaloadfn(&mut loadfn, b"glIsTransformFeedback\0", &[b"glIsTransformFeedbackNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TextureStorage3D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TextureStorage3D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TextureStorage3D = FnPtr::new(metaloadfn(&mut loadfn, b"glTextureStorage3D\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ClearNamedBufferSubData {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ClearNamedBufferSubData.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ClearNamedBufferSubData = FnPtr::new(metaloadfn(&mut loadfn, b"glClearNamedBufferSubData\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetBufferSubData {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetBufferSubData.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetBufferSubData = FnPtr::new(metaloadfn(&mut loadfn, b"glGetBufferSubData\0", &[b"glGetBufferSubDataARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib4fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib4fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib4fv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib4fv\0", &[b"glVertexAttrib4fvARB\0", b"glVertexAttrib4fvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetVertexAttribIiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetVertexAttribIiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetVertexAttribIiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetVertexAttribIiv\0", &[b"glGetVertexAttribIivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetDebugMessageLog {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetDebugMessageLog.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetDebugMessageLog = FnPtr::new(metaloadfn(&mut loadfn, b"glGetDebugMessageLog\0", &[b"glGetDebugMessageLogARB\0", b"glGetDebugMessageLogKHR\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod UniformBlockBinding {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UniformBlockBinding.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UniformBlockBinding = FnPtr::new(metaloadfn(&mut loadfn, b"glUniformBlockBinding\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod MapBuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::MapBuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::MapBuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glMapBuffer\0", &[b"glMapBufferARB\0", b"glMapBufferOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod NamedFramebufferDrawBuffers {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::NamedFramebufferDrawBuffers.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::NamedFramebufferDrawBuffers = FnPtr::new(metaloadfn(&mut loadfn, b"glNamedFramebufferDrawBuffers\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribP1uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribP1uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribP1uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribP1uiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ClientWaitSync {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ClientWaitSync.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ClientWaitSync = FnPtr::new(metaloadfn(&mut loadfn, b"glClientWaitSync\0", &[b"glClientWaitSyncAPPLE\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetSamplerParameterIuiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetSamplerParameterIuiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetSamplerParameterIuiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetSamplerParameterIuiv\0", &[b"glGetSamplerParameterIuivEXT\0", b"glGetSamplerParameterIuivOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniformMatrix4x2fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniformMatrix4x2fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniformMatrix4x2fv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniformMatrix4x2fv\0", &[b"glProgramUniformMatrix4x2fvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribI4bv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribI4bv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribI4bv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribI4bv\0", &[b"glVertexAttribI4bvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GenFramebuffers {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GenFramebuffers.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GenFramebuffers = FnPtr::new(metaloadfn(&mut loadfn, b"glGenFramebuffers\0", &[b"glGenFramebuffersEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetVertexAttribIuiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetVertexAttribIuiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetVertexAttribIuiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetVertexAttribIuiv\0", &[b"glGetVertexAttribIuivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniformMatrix2x3dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniformMatrix2x3dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniformMatrix2x3dv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniformMatrix2x3dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BufferSubData {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BufferSubData.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BufferSubData = FnPtr::new(metaloadfn(&mut loadfn, b"glBufferSubData\0", &[b"glBufferSubDataARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib3f {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib3f.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib3f = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib3f\0", &[b"glVertexAttrib3fARB\0", b"glVertexAttrib3fNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TexImage3DMultisample {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexImage3DMultisample.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexImage3DMultisample = FnPtr::new(metaloadfn(&mut loadfn, b"glTexImage3DMultisample\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetTexParameteriv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetTexParameteriv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetTexParameteriv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetTexParameteriv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetnConvolutionFilter {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetnConvolutionFilter.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetnConvolutionFilter = FnPtr::new(metaloadfn(&mut loadfn, b"glGetnConvolutionFilter\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib4bv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib4bv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib4bv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib4bv\0", &[b"glVertexAttrib4bvARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetDoublei_v {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetDoublei_v.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetDoublei_v = FnPtr::new(metaloadfn(&mut loadfn, b"glGetDoublei_v\0", &[b"glGetDoubleIndexedvEXT\0", b"glGetDoublei_vEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DeleteSync {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DeleteSync.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DeleteSync = FnPtr::new(metaloadfn(&mut loadfn, b"glDeleteSync\0", &[b"glDeleteSyncAPPLE\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod FlushMappedNamedBufferRange {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::FlushMappedNamedBufferRange.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::FlushMappedNamedBufferRange = FnPtr::new(metaloadfn(&mut loadfn, b"glFlushMappedNamedBufferRange\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetActiveUniformName {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetActiveUniformName.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetActiveUniformName = FnPtr::new(metaloadfn(&mut loadfn, b"glGetActiveUniformName\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform1uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform1uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform1uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform1uiv\0", &[b"glProgramUniform1uivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramBinary {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramBinary.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramBinary = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramBinary\0", &[b"glProgramBinaryOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GenerateTextureMipmap {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GenerateTextureMipmap.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GenerateTextureMipmap = FnPtr::new(metaloadfn(&mut loadfn, b"glGenerateTextureMipmap\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DepthRangeArrayv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DepthRangeArrayv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DepthRangeArrayv = FnPtr::new(metaloadfn(&mut loadfn, b"glDepthRangeArrayv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform2d {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform2d.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform2d = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform2d\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod CheckNamedFramebufferStatus {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CheckNamedFramebufferStatus.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CheckNamedFramebufferStatus = FnPtr::new(metaloadfn(&mut loadfn, b"glCheckNamedFramebufferStatus\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ResumeTransformFeedback {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ResumeTransformFeedback.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ResumeTransformFeedback = FnPtr::new(metaloadfn(&mut loadfn, b"glResumeTransformFeedback\0", &[b"glResumeTransformFeedbackNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribBinding {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribBinding.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribBinding = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribBinding\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod PixelStoref {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::PixelStoref.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::PixelStoref = FnPtr::new(metaloadfn(&mut loadfn, b"glPixelStoref\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod MultiTexCoordP1ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::MultiTexCoordP1ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::MultiTexCoordP1ui = FnPtr::new(metaloadfn(&mut loadfn, b"glMultiTexCoordP1ui\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetSamplerParameterfv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetSamplerParameterfv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetSamplerParameterfv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetSamplerParameterfv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetTexParameterIuiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetTexParameterIuiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetTexParameterIuiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetTexParameterIuiv\0", &[b"glGetTexParameterIuivEXT\0", b"glGetTexParameterIuivOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ClipControl {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ClipControl.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ClipControl = FnPtr::new(metaloadfn(&mut loadfn, b"glClipControl\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetSubroutineIndex {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetSubroutineIndex.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetSubroutineIndex = FnPtr::new(metaloadfn(&mut loadfn, b"glGetSubroutineIndex\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GenBuffers {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GenBuffers.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GenBuffers = FnPtr::new(metaloadfn(&mut loadfn, b"glGenBuffers\0", &[b"glGenBuffersARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetSamplerParameterIiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetSamplerParameterIiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetSamplerParameterIiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetSamplerParameterIiv\0", &[b"glGetSamplerParameterIivEXT\0", b"glGetSamplerParameterIivOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform3dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform3dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform3dv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform3dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniformMatrix3x4fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniformMatrix3x4fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniformMatrix3x4fv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniformMatrix3x4fv\0", &[b"glProgramUniformMatrix3x4fvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod LineWidth {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::LineWidth.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::LineWidth = FnPtr::new(metaloadfn(&mut loadfn, b"glLineWidth\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexArrayAttribLFormat {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexArrayAttribLFormat.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexArrayAttribLFormat = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexArrayAttribLFormat\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DepthRangeIndexed {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DepthRangeIndexed.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DepthRangeIndexed = FnPtr::new(metaloadfn(&mut loadfn, b"glDepthRangeIndexed\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniformMatrix3x4dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniformMatrix3x4dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniformMatrix3x4dv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniformMatrix3x4dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetTextureParameteriv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetTextureParameteriv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetTextureParameteriv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetTextureParameteriv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DrawElementsInstancedBaseInstance {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DrawElementsInstancedBaseInstance.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DrawElementsInstancedBaseInstance = FnPtr::new(metaloadfn(&mut loadfn, b"glDrawElementsInstancedBaseInstance\0", &[b"glDrawElementsInstancedBaseInstanceEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetVertexAttribLdv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetVertexAttribLdv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetVertexAttribLdv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetVertexAttribLdv\0", &[b"glGetVertexAttribLdvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexP3ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexP3ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexP3ui = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexP3ui\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ClearNamedFramebufferfi {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ClearNamedFramebufferfi.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ClearNamedFramebufferfi = FnPtr::new(metaloadfn(&mut loadfn, b"glClearNamedFramebufferfi\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DrawTransformFeedbackStreamInstanced {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DrawTransformFeedbackStreamInstanced.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DrawTransformFeedbackStreamInstanced = FnPtr::new(metaloadfn(&mut loadfn, b"glDrawTransformFeedbackStreamInstanced\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform3ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform3ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform3ui = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform3ui\0", &[b"glProgramUniform3uiEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetTextureLevelParameteriv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetTextureLevelParameteriv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetTextureLevelParameteriv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetTextureLevelParameteriv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform2dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform2dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform2dv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform2dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetQueryObjectui64v {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetQueryObjectui64v.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetQueryObjectui64v = FnPtr::new(metaloadfn(&mut loadfn, b"glGetQueryObjectui64v\0", &[b"glGetQueryObjectui64vEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform2fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform2fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform2fv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform2fv\0", &[b"glProgramUniform2fvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod MultiTexCoordP1uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::MultiTexCoordP1uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::MultiTexCoordP1uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glMultiTexCoordP1uiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod RenderbufferStorageMultisample {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::RenderbufferStorageMultisample.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::RenderbufferStorageMultisample = FnPtr::new(metaloadfn(&mut loadfn, b"glRenderbufferStorageMultisample\0", &[b"glRenderbufferStorageMultisampleEXT\0", b"glRenderbufferStorageMultisampleNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ColorP3uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ColorP3uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ColorP3uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glColorP3uiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod MultiTexCoordP2ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::MultiTexCoordP2ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::MultiTexCoordP2ui = FnPtr::new(metaloadfn(&mut loadfn, b"glMultiTexCoordP2ui\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BindFragDataLocation {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindFragDataLocation.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindFragDataLocation = FnPtr::new(metaloadfn(&mut loadfn, b"glBindFragDataLocation\0", &[b"glBindFragDataLocationEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform4uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform4uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform4uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform4uiv\0", &[b"glUniform4uivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetFramebufferAttachmentParameteriv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetFramebufferAttachmentParameteriv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetFramebufferAttachmentParameteriv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetFramebufferAttachmentParameteriv\0", &[b"glGetFramebufferAttachmentParameterivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetVertexArrayIndexediv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetVertexArrayIndexediv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetVertexArrayIndexediv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetVertexArrayIndexediv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TexParameterIiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexParameterIiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexParameterIiv = FnPtr::new(metaloadfn(&mut loadfn, b"glTexParameterIiv\0", &[b"glTexParameterIivEXT\0", b"glTexParameterIivOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetNamedBufferParameteri64v {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetNamedBufferParameteri64v.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetNamedBufferParameteri64v = FnPtr::new(metaloadfn(&mut loadfn, b"glGetNamedBufferParameteri64v\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod UniformMatrix3fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UniformMatrix3fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UniformMatrix3fv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniformMatrix3fv\0", &[b"glUniformMatrix3fvARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ClearBufferData {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ClearBufferData.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ClearBufferData = FnPtr::new(metaloadfn(&mut loadfn, b"glClearBufferData\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexP4uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexP4uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexP4uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexP4uiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod CopyImageSubData {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CopyImageSubData.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CopyImageSubData = FnPtr::new(metaloadfn(&mut loadfn, b"glCopyImageSubData\0", &[b"glCopyImageSubDataEXT\0", b"glCopyImageSubDataOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform4dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform4dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform4dv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform4dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GenTextures {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GenTextures.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GenTextures = FnPtr::new(metaloadfn(&mut loadfn, b"glGenTextures\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TexCoordP2uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexCoordP2uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexCoordP2uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glTexCoordP2uiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribL3dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribL3dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribL3dv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribL3dv\0", &[b"glVertexAttribL3dvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod CompressedTexImage1D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CompressedTexImage1D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CompressedTexImage1D = FnPtr::new(metaloadfn(&mut loadfn, b"glCompressedTexImage1D\0", &[b"glCompressedTexImage1DARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetTextureParameterIuiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetTextureParameterIuiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetTextureParameterIuiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetTextureParameterIuiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod InvalidateTexSubImage {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::InvalidateTexSubImage.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::InvalidateTexSubImage = FnPtr::new(metaloadfn(&mut loadfn, b"glInvalidateTexSubImage\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod FenceSync {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::FenceSync.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::FenceSync = FnPtr::new(metaloadfn(&mut loadfn, b"glFenceSync\0", &[b"glFenceSyncAPPLE\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribL1d {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribL1d.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribL1d = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribL1d\0", &[b"glVertexAttribL1dEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod UniformMatrix4x2dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UniformMatrix4x2dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UniformMatrix4x2dv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniformMatrix4x2dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod PauseTransformFeedback {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::PauseTransformFeedback.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::PauseTransformFeedback = FnPtr::new(metaloadfn(&mut loadfn, b"glPauseTransformFeedback\0", &[b"glPauseTransformFeedbackNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib4iv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib4iv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib4iv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib4iv\0", &[b"glVertexAttrib4ivARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod FramebufferTextureLayer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::FramebufferTextureLayer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::FramebufferTextureLayer = FnPtr::new(metaloadfn(&mut loadfn, b"glFramebufferTextureLayer\0", &[b"glFramebufferTextureLayerARB\0", b"glFramebufferTextureLayerEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TextureSubImage2D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TextureSubImage2D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TextureSubImage2D = FnPtr::new(metaloadfn(&mut loadfn, b"glTextureSubImage2D\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ColorP4ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ColorP4ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ColorP4ui = FnPtr::new(metaloadfn(&mut loadfn, b"glColorP4ui\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod TexParameterfv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexParameterfv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexParameterfv = FnPtr::new(metaloadfn(&mut loadfn, b"glTexParameterfv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod PushDebugGroup {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::PushDebugGroup.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::PushDebugGroup = FnPtr::new(metaloadfn(&mut loadfn, b"glPushDebugGroup\0", &[b"glPushDebugGroupKHR\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod MinSampleShading {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::MinSampleShading.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::MinSampleShading = FnPtr::new(metaloadfn(&mut loadfn, b"glMinSampleShading\0", &[b"glMinSampleShadingARB\0", b"glMinSampleShadingOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod BindFragDataLocationIndexed {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindFragDataLocationIndexed.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindFragDataLocationIndexed = FnPtr::new(metaloadfn(&mut loadfn, b"glBindFragDataLocationIndexed\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ScissorIndexed {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ScissorIndexed.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ScissorIndexed = FnPtr::new(metaloadfn(&mut loadfn, b"glScissorIndexed\0", &[b"glScissorIndexedNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib1d {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib1d.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib1d = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib1d\0", &[b"glVertexAttrib1dARB\0", b"glVertexAttrib1dNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod LogicOp {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::LogicOp.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::LogicOp = FnPtr::new(metaloadfn(&mut loadfn, b"glLogicOp\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetBooleani_v {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetBooleani_v.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetBooleani_v = FnPtr::new(metaloadfn(&mut loadfn, b"glGetBooleani_v\0", &[b"glGetBooleanIndexedvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetActiveUniform {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetActiveUniform.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetActiveUniform = FnPtr::new(metaloadfn(&mut loadfn, b"glGetActiveUniform\0", &[b"glGetActiveUniformARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib2fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib2fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib2fv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib2fv\0", &[b"glVertexAttrib2fvARB\0", b"glVertexAttrib2fvNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform4ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform4ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform4ui = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform4ui\0", &[b"glUniform4uiEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform3d {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform3d.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform3d = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform3d\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribI1i {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribI1i.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribI1i = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribI1i\0", &[b"glVertexAttribI1iEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribPointer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribPointer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribPointer = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribPointer\0", &[b"glVertexAttribPointerARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetUniformLocation {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetUniformLocation.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetUniformLocation = FnPtr::new(metaloadfn(&mut loadfn, b"glGetUniformLocation\0", &[b"glGetUniformLocationARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod CreateFramebuffers {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CreateFramebuffers.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CreateFramebuffers = FnPtr::new(metaloadfn(&mut loadfn, b"glCreateFramebuffers\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BindSamplers {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindSamplers.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindSamplers = FnPtr::new(metaloadfn(&mut loadfn, b"glBindSamplers\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetProgramResourceIndex {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetProgramResourceIndex.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetProgramResourceIndex = FnPtr::new(metaloadfn(&mut loadfn, b"glGetProgramResourceIndex\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetTexParameterIiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetTexParameterIiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetTexParameterIiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetTexParameterIiv\0", &[b"glGetTexParameterIivEXT\0", b"glGetTexParameterIivOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetQueryObjectiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetQueryObjectiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetQueryObjectiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetQueryObjectiv\0", &[b"glGetQueryObjectivARB\0", b"glGetQueryObjectivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib4Nbv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib4Nbv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib4Nbv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib4Nbv\0", &[b"glVertexAttrib4NbvARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetString {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetString.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetString = FnPtr::new(metaloadfn(&mut loadfn, b"glGetString\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod MultiTexCoordP4ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::MultiTexCoordP4ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::MultiTexCoordP4ui = FnPtr::new(metaloadfn(&mut loadfn, b"glMultiTexCoordP4ui\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniformMatrix4dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniformMatrix4dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniformMatrix4dv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniformMatrix4dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ColorMaski {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ColorMaski.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ColorMaski = FnPtr::new(metaloadfn(&mut loadfn, b"glColorMaski\0", &[b"glColorMaskIndexedEXT\0", b"glColorMaskiEXT\0", b"glColorMaskiOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod BindFramebuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BindFramebuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BindFramebuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glBindFramebuffer\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetSubroutineUniformLocation {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetSubroutineUniformLocation.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetSubroutineUniformLocation = FnPtr::new(metaloadfn(&mut loadfn, b"glGetSubroutineUniformLocation\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod NamedFramebufferTexture {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::NamedFramebufferTexture.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::NamedFramebufferTexture = FnPtr::new(metaloadfn(&mut loadfn, b"glNamedFramebufferTexture\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod SamplerParameterIiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::SamplerParameterIiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::SamplerParameterIiv = FnPtr::new(metaloadfn(&mut loadfn, b"glSamplerParameterIiv\0", &[b"glSamplerParameterIivEXT\0", b"glSamplerParameterIivOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TexCoordP3ui {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexCoordP3ui.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexCoordP3ui = FnPtr::new(metaloadfn(&mut loadfn, b"glTexCoordP3ui\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod FramebufferRenderbuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::FramebufferRenderbuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::FramebufferRenderbuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glFramebufferRenderbuffer\0", &[b"glFramebufferRenderbufferEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetProgramResourceName {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetProgramResourceName.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetProgramResourceName = FnPtr::new(metaloadfn(&mut loadfn, b"glGetProgramResourceName\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform3uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform3uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform3uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform3uiv\0", &[b"glProgramUniform3uivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod CompressedTexSubImage1D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CompressedTexSubImage1D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CompressedTexSubImage1D = FnPtr::new(metaloadfn(&mut loadfn, b"glCompressedTexSubImage1D\0", &[b"glCompressedTexSubImage1DARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TextureParameterIuiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TextureParameterIuiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TextureParameterIuiv = FnPtr::new(metaloadfn(&mut loadfn, b"glTextureParameterIuiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod UniformMatrix3x2dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UniformMatrix3x2dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UniformMatrix3x2dv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniformMatrix3x2dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetTextureParameterIiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetTextureParameterIiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetTextureParameterIiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetTextureParameterIiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod PrimitiveRestartIndex {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::PrimitiveRestartIndex.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::PrimitiveRestartIndex = FnPtr::new(metaloadfn(&mut loadfn, b"glPrimitiveRestartIndex\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod StencilMaskSeparate {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::StencilMaskSeparate.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::StencilMaskSeparate = FnPtr::new(metaloadfn(&mut loadfn, b"glStencilMaskSeparate\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform4d {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform4d.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform4d = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform4d\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DepthRange {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DepthRange.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DepthRange = FnPtr::new(metaloadfn(&mut loadfn, b"glDepthRange\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod StencilFunc {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::StencilFunc.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::StencilFunc = FnPtr::new(metaloadfn(&mut loadfn, b"glStencilFunc\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DrawElementsBaseVertex {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DrawElementsBaseVertex.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DrawElementsBaseVertex = FnPtr::new(metaloadfn(&mut loadfn, b"glDrawElementsBaseVertex\0", &[b"glDrawElementsBaseVertexEXT\0", b"glDrawElementsBaseVertexOES\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod Uniform4iv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::Uniform4iv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::Uniform4iv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniform4iv\0", &[b"glUniform4ivARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform1f {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform1f.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform1f = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform1f\0", &[b"glProgramUniform1fEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribI3uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribI3uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribI3uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribI3uiv\0", &[b"glVertexAttribI3uivEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod CompressedTextureSubImage2D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::CompressedTextureSubImage2D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::CompressedTextureSubImage2D = FnPtr::new(metaloadfn(&mut loadfn, b"glCompressedTextureSubImage2D\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BlitFramebuffer {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BlitFramebuffer.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BlitFramebuffer = FnPtr::new(metaloadfn(&mut loadfn, b"glBlitFramebuffer\0", &[b"glBlitFramebufferEXT\0", b"glBlitFramebufferNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod BeginQuery {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BeginQuery.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BeginQuery = FnPtr::new(metaloadfn(&mut loadfn, b"glBeginQuery\0", &[b"glBeginQueryARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod UniformMatrix3dv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::UniformMatrix3dv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::UniformMatrix3dv = FnPtr::new(metaloadfn(&mut loadfn, b"glUniformMatrix3dv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DisableVertexArrayAttrib {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DisableVertexArrayAttrib.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DisableVertexArrayAttrib = FnPtr::new(metaloadfn(&mut loadfn, b"glDisableVertexArrayAttrib\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttrib4f {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttrib4f.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttrib4f = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttrib4f\0", &[b"glVertexAttrib4fARB\0", b"glVertexAttrib4fNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ObjectLabel {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ObjectLabel.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ObjectLabel = FnPtr::new(metaloadfn(&mut loadfn, b"glObjectLabel\0", &[b"glObjectLabelKHR\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod MultiTexCoordP3uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::MultiTexCoordP3uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::MultiTexCoordP3uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glMultiTexCoordP3uiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetNamedFramebufferParameteriv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetNamedFramebufferParameteriv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetNamedFramebufferParameteriv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetNamedFramebufferParameteriv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod EndQuery {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::EndQuery.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::EndQuery = FnPtr::new(metaloadfn(&mut loadfn, b"glEndQuery\0", &[b"glEndQueryARB\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniform1d {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniform1d.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniform1d = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniform1d\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod VertexAttribP3uiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::VertexAttribP3uiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::VertexAttribP3uiv = FnPtr::new(metaloadfn(&mut loadfn, b"glVertexAttribP3uiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetInternalformativ {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetInternalformativ.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetInternalformativ = FnPtr::new(metaloadfn(&mut loadfn, b"glGetInternalformativ\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ClearBufferSubData {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ClearBufferSubData.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ClearBufferSubData = FnPtr::new(metaloadfn(&mut loadfn, b"glClearBufferSubData\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod BeginConditionalRender {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::BeginConditionalRender.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::BeginConditionalRender = FnPtr::new(metaloadfn(&mut loadfn, b"glBeginConditionalRender\0", &[b"glBeginConditionalRenderNV\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod DrawArrays {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DrawArrays.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DrawArrays = FnPtr::new(metaloadfn(&mut loadfn, b"glDrawArrays\0", &[b"glDrawArraysEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod TexImage2D {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::TexImage2D.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::TexImage2D = FnPtr::new(metaloadfn(&mut loadfn, b"glTexImage2D\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod DeleteSamplers {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::DeleteSamplers.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::DeleteSamplers = FnPtr::new(metaloadfn(&mut loadfn, b"glDeleteSamplers\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod ProgramUniformMatrix2x4fv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::ProgramUniformMatrix2x4fv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::ProgramUniformMatrix2x4fv = FnPtr::new(metaloadfn(&mut loadfn, b"glProgramUniformMatrix2x4fv\0", &[b"glProgramUniformMatrix2x4fvEXT\0"])) }
    }
}


#[allow(non_snake_case)]
pub mod GetTransformFeedbackiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetTransformFeedbackiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetTransformFeedbackiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetTransformFeedbackiv\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetFragDataIndex {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetFragDataIndex.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetFragDataIndex = FnPtr::new(metaloadfn(&mut loadfn, b"glGetFragDataIndex\0", &[])) }
    }
}


#[allow(non_snake_case)]
pub mod GetProgramPipelineiv {
    use super::{storage, metaloadfn};
    use c_types::*;
    use super::FnPtr;

    #[inline]
    #[allow(dead_code)]
    pub fn is_loaded() -> bool {
        unsafe { storage::GetProgramPipelineiv.is_loaded }
    }

    #[allow(dead_code)]
    pub fn load_with<F>(mut loadfn: F)
        where F: FnMut(&[u8]) -> *const c_void
    {
        unsafe { storage::GetProgramPipelineiv = FnPtr::new(metaloadfn(&mut loadfn, b"glGetProgramPipelineiv\0", &[])) }
    }
}

#[inline(never)]
fn missing_fn_panic() -> ! {
    panic!("gl function was not loaded")
}

/// Load each OpenGL symbol using a custom load function. This allows for the
/// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
/// ~~~ignore
/// gl::load_with(|s| glfw.get_proc_address(s));
/// ~~~
#[allow(dead_code)]
pub fn load_with<F>(mut loadfn: F)
    where F: FnMut(&[u8]) -> *const c_void
{
    EndTransformFeedback::load_with(|s| loadfn(s));
    GetProgramResourceLocationIndex::load_with(|s| loadfn(s));
    GetProgramResourceLocation::load_with(|s| loadfn(s));
    VertexAttribL3d::load_with(|s| loadfn(s));
    ObjectPtrLabel::load_with(|s| loadfn(s));
    ActiveShaderProgram::load_with(|s| loadfn(s));
    BindProgramPipeline::load_with(|s| loadfn(s));
    CreateProgramPipelines::load_with(|s| loadfn(s));
    NormalP3ui::load_with(|s| loadfn(s));
    UseProgramStages::load_with(|s| loadfn(s));
    VertexAttribL2d::load_with(|s| loadfn(s));
    GetnHistogram::load_with(|s| loadfn(s));
    ScissorArrayv::load_with(|s| loadfn(s));
    VertexAttribDivisor::load_with(|s| loadfn(s));
    GetTexImage::load_with(|s| loadfn(s));
    SamplerParameteri::load_with(|s| loadfn(s));
    TextureBarrier::load_with(|s| loadfn(s));
    TextureParameteri::load_with(|s| loadfn(s));
    GetObjectLabel::load_with(|s| loadfn(s));
    ReadBuffer::load_with(|s| loadfn(s));
    StencilOpSeparate::load_with(|s| loadfn(s));
    TexSubImage2D::load_with(|s| loadfn(s));
    GetTransformFeedbackVarying::load_with(|s| loadfn(s));
    MapNamedBufferRange::load_with(|s| loadfn(s));
    SamplerParameteriv::load_with(|s| loadfn(s));
    ProgramUniform4fv::load_with(|s| loadfn(s));
    UniformMatrix4x3dv::load_with(|s| loadfn(s));
    ScissorIndexedv::load_with(|s| loadfn(s));
    BindImageTexture::load_with(|s| loadfn(s));
    BlendColor::load_with(|s| loadfn(s));
    GetPointerv::load_with(|s| loadfn(s));
    ProgramUniform2uiv::load_with(|s| loadfn(s));
    DrawElementsInstancedBaseVertexBaseInstance::load_with(|s| loadfn(s));
    GetInteger64v::load_with(|s| loadfn(s));
    VertexAttribI2ui::load_with(|s| loadfn(s));
    ProgramUniform1i::load_with(|s| loadfn(s));
    GetVertexAttribiv::load_with(|s| loadfn(s));
    ProgramUniform4i::load_with(|s| loadfn(s));
    VertexArrayAttribBinding::load_with(|s| loadfn(s));
    GetFloatv::load_with(|s| loadfn(s));
    DispatchComputeIndirect::load_with(|s| loadfn(s));
    Enable::load_with(|s| loadfn(s));
    BindBufferRange::load_with(|s| loadfn(s));
    ShaderSource::load_with(|s| loadfn(s));
    VertexArrayAttribIFormat::load_with(|s| loadfn(s));
    VertexAttribI4ubv::load_with(|s| loadfn(s));
    VertexAttrib1s::load_with(|s| loadfn(s));
    VertexAttribI2iv::load_with(|s| loadfn(s));
    GetObjectPtrLabel::load_with(|s| loadfn(s));
    Uniform2d::load_with(|s| loadfn(s));
    MultiDrawArraysIndirect::load_with(|s| loadfn(s));
    DrawArraysInstanced::load_with(|s| loadfn(s));
    GetVertexArrayIndexed64iv::load_with(|s| loadfn(s));
    GetQueryIndexediv::load_with(|s| loadfn(s));
    GetFragDataLocation::load_with(|s| loadfn(s));
    DispatchCompute::load_with(|s| loadfn(s));
    CopyTextureSubImage2D::load_with(|s| loadfn(s));
    ClearTexImage::load_with(|s| loadfn(s));
    VertexAttribI4ui::load_with(|s| loadfn(s));
    VertexAttrib4Nsv::load_with(|s| loadfn(s));
    VertexAttribI3i::load_with(|s| loadfn(s));
    VertexAttribP4uiv::load_with(|s| loadfn(s));
    VertexAttribP2uiv::load_with(|s| loadfn(s));
    ProgramUniform2ui::load_with(|s| loadfn(s));
    Viewport::load_with(|s| loadfn(s));
    GetError::load_with(|s| loadfn(s));
    DrawBuffers::load_with(|s| loadfn(s));
    GetTextureLevelParameterfv::load_with(|s| loadfn(s));
    NamedBufferStorage::load_with(|s| loadfn(s));
    DrawRangeElementsBaseVertex::load_with(|s| loadfn(s));
    ProgramUniformMatrix2dv::load_with(|s| loadfn(s));
    GetVertexAttribdv::load_with(|s| loadfn(s));
    GetnUniformdv::load_with(|s| loadfn(s));
    ClearBufferuiv::load_with(|s| loadfn(s));
    IsEnabled::load_with(|s| loadfn(s));
    DrawTransformFeedback::load_with(|s| loadfn(s));
    VertexAttribL2dv::load_with(|s| loadfn(s));
    DepthFunc::load_with(|s| loadfn(s));
    MultiDrawElements::load_with(|s| loadfn(s));
    Flush::load_with(|s| loadfn(s));
    GetUniformfv::load_with(|s| loadfn(s));
    GetnPixelMapuiv::load_with(|s| loadfn(s));
    GetQueryObjecti64v::load_with(|s| loadfn(s));
    GenerateMipmap::load_with(|s| loadfn(s));
    DrawTransformFeedbackStream::load_with(|s| loadfn(s));
    GetTexLevelParameterfv::load_with(|s| loadfn(s));
    VertexAttrib4uiv::load_with(|s| loadfn(s));
    UniformMatrix4dv::load_with(|s| loadfn(s));
    VertexAttrib4d::load_with(|s| loadfn(s));
    DepthMask::load_with(|s| loadfn(s));
    VertexAttribL4d::load_with(|s| loadfn(s));
    CopyTexSubImage1D::load_with(|s| loadfn(s));
    Uniform1ui::load_with(|s| loadfn(s));
    VertexAttrib4Nubv::load_with(|s| loadfn(s));
    UniformSubroutinesuiv::load_with(|s| loadfn(s));
    Scissor::load_with(|s| loadfn(s));
    TextureStorage3DMultisample::load_with(|s| loadfn(s));
    StencilFuncSeparate::load_with(|s| loadfn(s));
    TexCoordP3uiv::load_with(|s| loadfn(s));
    ValidateProgram::load_with(|s| loadfn(s));
    InvalidateSubFramebuffer::load_with(|s| loadfn(s));
    VertexAttrib3fv::load_with(|s| loadfn(s));
    DeleteVertexArrays::load_with(|s| loadfn(s));
    VertexAttribI4uiv::load_with(|s| loadfn(s));
    VertexAttrib4sv::load_with(|s| loadfn(s));
    SamplerParameterf::load_with(|s| loadfn(s));
    VertexAttribI1iv::load_with(|s| loadfn(s));
    TexParameteriv::load_with(|s| loadfn(s));
    Uniform4i::load_with(|s| loadfn(s));
    TexCoordP1ui::load_with(|s| loadfn(s));
    IsFramebuffer::load_with(|s| loadfn(s));
    IsTexture::load_with(|s| loadfn(s));
    BlendFunc::load_with(|s| loadfn(s));
    ProgramUniform4ui::load_with(|s| loadfn(s));
    UniformMatrix2dv::load_with(|s| loadfn(s));
    VertexArrayElementBuffer::load_with(|s| loadfn(s));
    GenProgramPipelines::load_with(|s| loadfn(s));
    NamedFramebufferReadBuffer::load_with(|s| loadfn(s));
    DrawElements::load_with(|s| loadfn(s));
    TextureParameteriv::load_with(|s| loadfn(s));
    StencilOp::load_with(|s| loadfn(s));
    BindVertexBuffers::load_with(|s| loadfn(s));
    PopDebugGroup::load_with(|s| loadfn(s));
    Uniform2ui::load_with(|s| loadfn(s));
    SecondaryColorP3uiv::load_with(|s| loadfn(s));
    BindSampler::load_with(|s| loadfn(s));
    Uniform1dv::load_with(|s| loadfn(s));
    VertexAttrib3d::load_with(|s| loadfn(s));
    GetNamedBufferPointerv::load_with(|s| loadfn(s));
    CreateSamplers::load_with(|s| loadfn(s));
    EndQueryIndexed::load_with(|s| loadfn(s));
    ClearBufferfv::load_with(|s| loadfn(s));
    UniformMatrix4x2fv::load_with(|s| loadfn(s));
    StencilMask::load_with(|s| loadfn(s));
    UniformMatrix4fv::load_with(|s| loadfn(s));
    PolygonMode::load_with(|s| loadfn(s));
    CompressedTexSubImage3D::load_with(|s| loadfn(s));
    VertexAttribP4ui::load_with(|s| loadfn(s));
    VertexAttribIPointer::load_with(|s| loadfn(s));
    NamedFramebufferTextureLayer::load_with(|s| loadfn(s));
    DeleteFramebuffers::load_with(|s| loadfn(s));
    Disable::load_with(|s| loadfn(s));
    GetShaderInfoLog::load_with(|s| loadfn(s));
    Uniform3d::load_with(|s| loadfn(s));
    CopyTextureSubImage3D::load_with(|s| loadfn(s));
    InvalidateBufferData::load_with(|s| loadfn(s));
    EndConditionalRender::load_with(|s| loadfn(s));
    ReleaseShaderCompiler::load_with(|s| loadfn(s));
    NamedBufferSubData::load_with(|s| loadfn(s));
    GetnPixelMapfv::load_with(|s| loadfn(s));
    UniformMatrix3x2fv::load_with(|s| loadfn(s));
    CopyNamedBufferSubData::load_with(|s| loadfn(s));
    ProgramUniformMatrix4x2dv::load_with(|s| loadfn(s));
    GetDoublev::load_with(|s| loadfn(s));
    DisableVertexAttribArray::load_with(|s| loadfn(s));
    BindBuffersRange::load_with(|s| loadfn(s));
    ProgramUniform4uiv::load_with(|s| loadfn(s));
    ActiveTexture::load_with(|s| loadfn(s));
    GetProgramiv::load_with(|s| loadfn(s));
    VertexAttribIFormat::load_with(|s| loadfn(s));
    CopyTexSubImage3D::load_with(|s| loadfn(s));
    GetActiveAtomicCounterBufferiv::load_with(|s| loadfn(s));
    DrawElementsIndirect::load_with(|s| loadfn(s));
    ViewportIndexedf::load_with(|s| loadfn(s));
    VertexAttrib4ubv::load_with(|s| loadfn(s));
    ClearBufferfi::load_with(|s| loadfn(s));
    VertexAttribI1uiv::load_with(|s| loadfn(s));
    AttachShader::load_with(|s| loadfn(s));
    VertexAttrib3sv::load_with(|s| loadfn(s));
    BindTransformFeedback::load_with(|s| loadfn(s));
    ProgramUniform3i::load_with(|s| loadfn(s));
    ClearBufferiv::load_with(|s| loadfn(s));
    ProgramUniform3iv::load_with(|s| loadfn(s));
    GetCompressedTexImage::load_with(|s| loadfn(s));
    GetQueryBufferObjecti64v::load_with(|s| loadfn(s));
    ProgramUniform4dv::load_with(|s| loadfn(s));
    VertexArrayVertexBuffer::load_with(|s| loadfn(s));
    Uniform2f::load_with(|s| loadfn(s));
    GetNamedRenderbufferParameteriv::load_with(|s| loadfn(s));
    VertexAttrib2sv::load_with(|s| loadfn(s));
    GetTextureSubImage::load_with(|s| loadfn(s));
    VertexAttribI3ui::load_with(|s| loadfn(s));
    GetQueryiv::load_with(|s| loadfn(s));
    MemoryBarrierByRegion::load_with(|s| loadfn(s));
    ProgramUniformMatrix3fv::load_with(|s| loadfn(s));
    VertexAttrib1sv::load_with(|s| loadfn(s));
    BindTexture::load_with(|s| loadfn(s));
    TextureBufferRange::load_with(|s| loadfn(s));
    Uniform4f::load_with(|s| loadfn(s));
    ClearDepth::load_with(|s| loadfn(s));
    FrontFace::load_with(|s| loadfn(s));
    GetTextureParameterfv::load_with(|s| loadfn(s));
    MemoryBarrier::load_with(|s| loadfn(s));
    ViewportArrayv::load_with(|s| loadfn(s));
    BeginQueryIndexed::load_with(|s| loadfn(s));
    PatchParameterfv::load_with(|s| loadfn(s));
    BindTextures::load_with(|s| loadfn(s));
    GetProgramPipelineInfoLog::load_with(|s| loadfn(s));
    GetUniformuiv::load_with(|s| loadfn(s));
    MultiDrawArrays::load_with(|s| loadfn(s));
    ProgramUniform1ui::load_with(|s| loadfn(s));
    GetStringi::load_with(|s| loadfn(s));
    GetShaderSource::load_with(|s| loadfn(s));
    MapBufferRange::load_with(|s| loadfn(s));
    VertexAttrib4Nuiv::load_with(|s| loadfn(s));
    ClearColor::load_with(|s| loadfn(s));
    Uniform3ui::load_with(|s| loadfn(s));
    CreateProgram::load_with(|s| loadfn(s));
    IsProgramPipeline::load_with(|s| loadfn(s));
    Uniform3f::load_with(|s| loadfn(s));
    CreateQueries::load_with(|s| loadfn(s));
    GetNamedBufferParameteriv::load_with(|s| loadfn(s));
    GetShaderiv::load_with(|s| loadfn(s));
    PointSize::load_with(|s| loadfn(s));
    DrawTransformFeedbackInstanced::load_with(|s| loadfn(s));
    IsVertexArray::load_with(|s| loadfn(s));
    GetCompressedTextureSubImage::load_with(|s| loadfn(s));
    GetnPixelMapusv::load_with(|s| loadfn(s));
    BeginTransformFeedback::load_with(|s| loadfn(s));
    GetGraphicsResetStatus::load_with(|s| loadfn(s));
    Clear::load_with(|s| loadfn(s));
    ColorP3ui::load_with(|s| loadfn(s));
    CreateBuffers::load_with(|s| loadfn(s));
    TexParameteri::load_with(|s| loadfn(s));
    Uniform2i::load_with(|s| loadfn(s));
    IsShader::load_with(|s| loadfn(s));
    GetBufferParameteriv::load_with(|s| loadfn(s));
    GetCompressedTextureImage::load_with(|s| loadfn(s));
    Uniform1f::load_with(|s| loadfn(s));
    ClearNamedFramebufferuiv::load_with(|s| loadfn(s));
    BlendEquationi::load_with(|s| loadfn(s));
    CopyBufferSubData::load_with(|s| loadfn(s));
    PointParameteriv::load_with(|s| loadfn(s));
    GetnUniformiv::load_with(|s| loadfn(s));
    GetActiveUniformsiv::load_with(|s| loadfn(s));
    BindBuffer::load_with(|s| loadfn(s));
    DeleteProgram::load_with(|s| loadfn(s));
    VertexAttrib2dv::load_with(|s| loadfn(s));
    ProgramUniformMatrix2x3fv::load_with(|s| loadfn(s));
    BindAttribLocation::load_with(|s| loadfn(s));
    ProvokingVertex::load_with(|s| loadfn(s));
    GetTransformFeedbacki_v::load_with(|s| loadfn(s));
    ProgramUniform4f::load_with(|s| loadfn(s));
    CompressedTextureSubImage1D::load_with(|s| loadfn(s));
    TexStorage1D::load_with(|s| loadfn(s));
    VertexAttribI4usv::load_with(|s| loadfn(s));
    IsRenderbuffer::load_with(|s| loadfn(s));
    VertexAttribP1ui::load_with(|s| loadfn(s));
    Uniform3uiv::load_with(|s| loadfn(s));
    ProgramUniformMatrix4x3fv::load_with(|s| loadfn(s));
    GetUniformIndices::load_with(|s| loadfn(s));
    GenSamplers::load_with(|s| loadfn(s));
    ProgramUniformMatrix4fv::load_with(|s| loadfn(s));
    VertexArrayBindingDivisor::load_with(|s| loadfn(s));
    VertexP2uiv::load_with(|s| loadfn(s));
    VertexAttrib4s::load_with(|s| loadfn(s));
    DeleteTextures::load_with(|s| loadfn(s));
    BindImageTextures::load_with(|s| loadfn(s));
    WaitSync::load_with(|s| loadfn(s));
    BindVertexArray::load_with(|s| loadfn(s));
    GetActiveAttrib::load_with(|s| loadfn(s));
    TextureStorage2DMultisample::load_with(|s| loadfn(s));
    DebugMessageInsert::load_with(|s| loadfn(s));
    DeleteTransformFeedbacks::load_with(|s| loadfn(s));
    TextureSubImage1D::load_with(|s| loadfn(s));
    VertexAttribL1dv::load_with(|s| loadfn(s));
    VertexAttrib1fv::load_with(|s| loadfn(s));
    GetBufferParameteri64v::load_with(|s| loadfn(s));
    DeleteRenderbuffers::load_with(|s| loadfn(s));
    GetRenderbufferParameteriv::load_with(|s| loadfn(s));
    TextureParameterfv::load_with(|s| loadfn(s));
    TexBufferRange::load_with(|s| loadfn(s));
    NamedBufferData::load_with(|s| loadfn(s));
    PixelStorei::load_with(|s| loadfn(s));
    GetActiveSubroutineUniformName::load_with(|s| loadfn(s));
    BlendEquation::load_with(|s| loadfn(s));
    BufferData::load_with(|s| loadfn(s));
    CompressedTexSubImage2D::load_with(|s| loadfn(s));
    FramebufferTexture3D::load_with(|s| loadfn(s));
    ProgramUniformMatrix4x3dv::load_with(|s| loadfn(s));
    GetnCompressedTexImage::load_with(|s| loadfn(s));
    GetProgramStageiv::load_with(|s| loadfn(s));
    ClampColor::load_with(|s| loadfn(s));
    ValidateProgramPipeline::load_with(|s| loadfn(s));
    GetVertexAttribfv::load_with(|s| loadfn(s));
    ProgramUniformMatrix2x4dv::load_with(|s| loadfn(s));
    UniformMatrix4x3fv::load_with(|s| loadfn(s));
    MultiTexCoordP2uiv::load_with(|s| loadfn(s));
    DeleteShader::load_with(|s| loadfn(s));
    NamedFramebufferRenderbuffer::load_with(|s| loadfn(s));
    GetAttribLocation::load_with(|s| loadfn(s));
    GetInteger64i_v::load_with(|s| loadfn(s));
    CopyTexImage1D::load_with(|s| loadfn(s));
    VertexAttrib2f::load_with(|s| loadfn(s));
    VertexAttribI4iv::load_with(|s| loadfn(s));
    ClearDepthf::load_with(|s| loadfn(s));
    UniformMatrix2x3dv::load_with(|s| loadfn(s));
    GetTexLevelParameteriv::load_with(|s| loadfn(s));
    ReadnPixels::load_with(|s| loadfn(s));
    LinkProgram::load_with(|s| loadfn(s));
    EnableVertexArrayAttrib::load_with(|s| loadfn(s));
    VertexAttribLPointer::load_with(|s| loadfn(s));
    TextureView::load_with(|s| loadfn(s));
    GetActiveSubroutineUniformiv::load_with(|s| loadfn(s));
    GetQueryBufferObjectui64v::load_with(|s| loadfn(s));
    CompileShader::load_with(|s| loadfn(s));
    Uniform2fv::load_with(|s| loadfn(s));
    TexSubImage3D::load_with(|s| loadfn(s));
    TexImage2DMultisample::load_with(|s| loadfn(s));
    Uniform4d::load_with(|s| loadfn(s));
    GetTransformFeedbacki64_v::load_with(|s| loadfn(s));
    ProgramUniformMatrix3x2fv::load_with(|s| loadfn(s));
    ProgramUniformMatrix2fv::load_with(|s| loadfn(s));
    CreateVertexArrays::load_with(|s| loadfn(s));
    BindBufferBase::load_with(|s| loadfn(s));
    GetSamplerParameteriv::load_with(|s| loadfn(s));
    ReadPixels::load_with(|s| loadfn(s));
    VertexAttribLFormat::load_with(|s| loadfn(s));
    GetQueryBufferObjectuiv::load_with(|s| loadfn(s));
    FramebufferTexture::load_with(|s| loadfn(s));
    TexParameterf::load_with(|s| loadfn(s));
    FramebufferParameteri::load_with(|s| loadfn(s));
    TextureParameterIiv::load_with(|s| loadfn(s));
    BindBuffersBase::load_with(|s| loadfn(s));
    TexStorage3DMultisample::load_with(|s| loadfn(s));
    VertexAttribI4i::load_with(|s| loadfn(s));
    DrawRangeElements::load_with(|s| loadfn(s));
    TexImage3D::load_with(|s| loadfn(s));
    TextureStorage2D::load_with(|s| loadfn(s));
    TransformFeedbackBufferRange::load_with(|s| loadfn(s));
    VertexP4ui::load_with(|s| loadfn(s));
    BlendFuncSeparate::load_with(|s| loadfn(s));
    Uniform4fv::load_with(|s| loadfn(s));
    CreateShaderProgramv::load_with(|s| loadfn(s));
    BindVertexBuffer::load_with(|s| loadfn(s));
    TexStorage2DMultisample::load_with(|s| loadfn(s));
    ShaderStorageBlockBinding::load_with(|s| loadfn(s));
    NamedRenderbufferStorageMultisample::load_with(|s| loadfn(s));
    GetProgramResourceiv::load_with(|s| loadfn(s));
    EnableVertexAttribArray::load_with(|s| loadfn(s));
    TexCoordP2ui::load_with(|s| loadfn(s));
    TexStorage2D::load_with(|s| loadfn(s));
    VertexAttrib4Niv::load_with(|s| loadfn(s));
    VertexArrayVertexBuffers::load_with(|s| loadfn(s));
    ProgramUniform2iv::load_with(|s| loadfn(s));
    UniformMatrix2fv::load_with(|s| loadfn(s));
    GetnMinmax::load_with(|s| loadfn(s));
    UniformMatrix2x4fv::load_with(|s| loadfn(s));
    Finish::load_with(|s| loadfn(s));
    MultiDrawElementsIndirect::load_with(|s| loadfn(s));
    DebugMessageCallback::load_with(|s| loadfn(s));
    GetnUniformfv::load_with(|s| loadfn(s));
    SamplerParameterIuiv::load_with(|s| loadfn(s));
    CopyTexImage2D::load_with(|s| loadfn(s));
    UniformMatrix2x4dv::load_with(|s| loadfn(s));
    FramebufferTexture2D::load_with(|s| loadfn(s));
    VertexAttribFormat::load_with(|s| loadfn(s));
    ClearNamedBufferData::load_with(|s| loadfn(s));
    CheckFramebufferStatus::load_with(|s| loadfn(s));
    VertexAttribI2uiv::load_with(|s| loadfn(s));
    BufferStorage::load_with(|s| loadfn(s));
    PointParameterf::load_with(|s| loadfn(s));
    GetnColorTable::load_with(|s| loadfn(s));
    GetnTexImage::load_with(|s| loadfn(s));
    DeleteQueries::load_with(|s| loadfn(s));
    CreateTransformFeedbacks::load_with(|s| loadfn(s));
    ProgramUniform3fv::load_with(|s| loadfn(s));
    TransformFeedbackBufferBase::load_with(|s| loadfn(s));
    UnmapNamedBuffer::load_with(|s| loadfn(s));
    GetUniformdv::load_with(|s| loadfn(s));
    CompressedTexImage3D::load_with(|s| loadfn(s));
    DrawElementsInstanced::load_with(|s| loadfn(s));
    GenQueries::load_with(|s| loadfn(s));
    CopyTexSubImage2D::load_with(|s| loadfn(s));
    DrawArraysInstancedBaseInstance::load_with(|s| loadfn(s));
    TexCoordP4ui::load_with(|s| loadfn(s));
    VertexAttribP2ui::load_with(|s| loadfn(s));
    VertexAttrib4dv::load_with(|s| loadfn(s));
    ColorP4uiv::load_with(|s| loadfn(s));
    GetActiveSubroutineName::load_with(|s| loadfn(s));
    TexCoordP4uiv::load_with(|s| loadfn(s));
    ProgramUniform3f::load_with(|s| loadfn(s));
    ProgramUniform1iv::load_with(|s| loadfn(s));
    VertexAttrib1f::load_with(|s| loadfn(s));
    Uniform1d::load_with(|s| loadfn(s));
    Uniform2iv::load_with(|s| loadfn(s));
    CompressedTexImage2D::load_with(|s| loadfn(s));
    DrawBuffer::load_with(|s| loadfn(s));
    ClearNamedFramebufferiv::load_with(|s| loadfn(s));
    Hint::load_with(|s| loadfn(s));
    DeleteBuffers::load_with(|s| loadfn(s));
    VertexArrayAttribFormat::load_with(|s| loadfn(s));
    GenTransformFeedbacks::load_with(|s| loadfn(s));
    IsBuffer::load_with(|s| loadfn(s));
    DrawElementsInstancedBaseVertex::load_with(|s| loadfn(s));
    Uniform3i::load_with(|s| loadfn(s));
    GetProgramBinary::load_with(|s| loadfn(s));
    GetVertexAttribPointerv::load_with(|s| loadfn(s));
    GetActiveUniformBlockiv::load_with(|s| loadfn(s));
    ProgramUniform3dv::load_with(|s| loadfn(s));
    TexStorage3D::load_with(|s| loadfn(s));
    GetQueryBufferObjectiv::load_with(|s| loadfn(s));
    DepthRangef::load_with(|s| loadfn(s));
    DeleteProgramPipelines::load_with(|s| loadfn(s));
    VertexAttrib4Nusv::load_with(|s| loadfn(s));
    ClearTexSubImage::load_with(|s| loadfn(s));
    MultiTexCoordP3ui::load_with(|s| loadfn(s));
    ProgramUniform2f::load_with(|s| loadfn(s));
    IsQuery::load_with(|s| loadfn(s));
    GetnSeparableFilter::load_with(|s| loadfn(s));
    GetProgramInfoLog::load_with(|s| loadfn(s));
    BindRenderbuffer::load_with(|s| loadfn(s));
    RenderbufferStorage::load_with(|s| loadfn(s));
    DebugMessageControl::load_with(|s| loadfn(s));
    GetnUniformuiv::load_with(|s| loadfn(s));
    PolygonOffset::load_with(|s| loadfn(s));
    MultiDrawElementsBaseVertex::load_with(|s| loadfn(s));
    NamedFramebufferDrawBuffer::load_with(|s| loadfn(s));
    VertexAttrib2d::load_with(|s| loadfn(s));
    CreateTextures::load_with(|s| loadfn(s));
    GetUniformSubroutineuiv::load_with(|s| loadfn(s));
    ClearNamedFramebufferfv::load_with(|s| loadfn(s));
    CreateRenderbuffers::load_with(|s| loadfn(s));
    IsSampler::load_with(|s| loadfn(s));
    MultiTexCoordP4uiv::load_with(|s| loadfn(s));
    GetSynciv::load_with(|s| loadfn(s));
    UnmapBuffer::load_with(|s| loadfn(s));
    GetBufferPointerv::load_with(|s| loadfn(s));
    GenVertexArrays::load_with(|s| loadfn(s));
    SampleMaski::load_with(|s| loadfn(s));
    ClearStencil::load_with(|s| loadfn(s));
    BlendFuncSeparatei::load_with(|s| loadfn(s));
    VertexAttrib4Nub::load_with(|s| loadfn(s));
    ShaderBinary::load_with(|s| loadfn(s));
    TextureSubImage3D::load_with(|s| loadfn(s));
    GetUniformiv::load_with(|s| loadfn(s));
    Uniform1uiv::load_with(|s| loadfn(s));
    VertexAttribI4sv::load_with(|s| loadfn(s));
    BlitNamedFramebuffer::load_with(|s| loadfn(s));
    GetAttachedShaders::load_with(|s| loadfn(s));
    InvalidateBufferSubData::load_with(|s| loadfn(s));
    InvalidateFramebuffer::load_with(|s| loadfn(s));
    TextureStorage1D::load_with(|s| loadfn(s));
    FramebufferTexture1D::load_with(|s| loadfn(s));
    GetnMapiv::load_with(|s| loadfn(s));
    GetQueryObjectuiv::load_with(|s| loadfn(s));
    DetachShader::load_with(|s| loadfn(s));
    GetActiveUniformBlockName::load_with(|s| loadfn(s));
    IsSync::load_with(|s| loadfn(s));
    GetBooleanv::load_with(|s| loadfn(s));
    QueryCounter::load_with(|s| loadfn(s));
    InvalidateNamedFramebufferData::load_with(|s| loadfn(s));
    TexSubImage1D::load_with(|s| loadfn(s));
    CopyTextureSubImage1D::load_with(|s| loadfn(s));
    GetIntegeri_v::load_with(|s| loadfn(s));
    Uniform3fv::load_with(|s| loadfn(s));
    VertexAttrib1dv::load_with(|s| loadfn(s));
    Disablei::load_with(|s| loadfn(s));
    ViewportIndexedfv::load_with(|s| loadfn(s));
    PatchParameteri::load_with(|s| loadfn(s));
    VertexAttribI2i::load_with(|s| loadfn(s));
    Uniform1i::load_with(|s| loadfn(s));
    UniformMatrix3x4dv::load_with(|s| loadfn(s));
    VertexAttribL4dv::load_with(|s| loadfn(s));
    SamplerParameterfv::load_with(|s| loadfn(s));
    VertexAttrib3dv::load_with(|s| loadfn(s));
    ColorMask::load_with(|s| loadfn(s));
    GetUniformBlockIndex::load_with(|s| loadfn(s));
    TextureParameterf::load_with(|s| loadfn(s));
    GetMultisamplefv::load_with(|s| loadfn(s));
    ProgramParameteri::load_with(|s| loadfn(s));
    MapNamedBuffer::load_with(|s| loadfn(s));
    TextureBuffer::load_with(|s| loadfn(s));
    NormalP3uiv::load_with(|s| loadfn(s));
    BlendFunci::load_with(|s| loadfn(s));
    VertexAttrib2s::load_with(|s| loadfn(s));
    VertexAttribP3ui::load_with(|s| loadfn(s));
    GetNamedFramebufferAttachmentParameteriv::load_with(|s| loadfn(s));
    NamedRenderbufferStorage::load_with(|s| loadfn(s));
    ProgramUniform1fv::load_with(|s| loadfn(s));
    BlendEquationSeparate::load_with(|s| loadfn(s));
    TexBuffer::load_with(|s| loadfn(s));
    TexImage1D::load_with(|s| loadfn(s));
    TexParameterIuiv::load_with(|s| loadfn(s));
    VertexP2ui::load_with(|s| loadfn(s));
    GenRenderbuffers::load_with(|s| loadfn(s));
    VertexBindingDivisor::load_with(|s| loadfn(s));
    ProgramUniform2i::load_with(|s| loadfn(s));
    Enablei::load_with(|s| loadfn(s));
    GetnMapfv::load_with(|s| loadfn(s));
    IsEnabledi::load_with(|s| loadfn(s));
    CompressedTextureSubImage3D::load_with(|s| loadfn(s));
    GetShaderPrecisionFormat::load_with(|s| loadfn(s));
    GetTextureImage::load_with(|s| loadfn(s));
    UniformMatrix3x4fv::load_with(|s| loadfn(s));
    Uniform2uiv::load_with(|s| loadfn(s));
    GetInternalformati64v::load_with(|s| loadfn(s));
    ProgramUniform2dv::load_with(|s| loadfn(s));
    VertexAttrib3s::load_with(|s| loadfn(s));
    FlushMappedBufferRange::load_with(|s| loadfn(s));
    InvalidateTexImage::load_with(|s| loadfn(s));
    GetProgramInterfaceiv::load_with(|s| loadfn(s));
    CullFace::load_with(|s| loadfn(s));
    GetFramebufferParameteriv::load_with(|s| loadfn(s));
    CreateShader::load_with(|s| loadfn(s));
    ProgramUniformMatrix3dv::load_with(|s| loadfn(s));
    PointParameterfv::load_with(|s| loadfn(s));
    DrawArraysIndirect::load_with(|s| loadfn(s));
    UseProgram::load_with(|s| loadfn(s));
    ProgramUniformMatrix3x2dv::load_with(|s| loadfn(s));
    SampleCoverage::load_with(|s| loadfn(s));
    Uniform3iv::load_with(|s| loadfn(s));
    VertexAttribI3iv::load_with(|s| loadfn(s));
    ProgramUniform1dv::load_with(|s| loadfn(s));
    BlendEquationSeparatei::load_with(|s| loadfn(s));
    GetFloati_v::load_with(|s| loadfn(s));
    ProgramUniform4iv::load_with(|s| loadfn(s));
    SecondaryColorP3ui::load_with(|s| loadfn(s));
    VertexAttribI1ui::load_with(|s| loadfn(s));
    Uniform1iv::load_with(|s| loadfn(s));
    GetVertexArrayiv::load_with(|s| loadfn(s));
    IsProgram::load_with(|s| loadfn(s));
    BindTextureUnit::load_with(|s| loadfn(s));
    GetnPolygonStipple::load_with(|s| loadfn(s));
    GetIntegerv::load_with(|s| loadfn(s));
    NamedFramebufferParameteri::load_with(|s| loadfn(s));
    VertexP3uiv::load_with(|s| loadfn(s));
    VertexAttrib4usv::load_with(|s| loadfn(s));
    UniformMatrix2x3fv::load_with(|s| loadfn(s));
    GetnMapdv::load_with(|s| loadfn(s));
    TexCoordP1uiv::load_with(|s| loadfn(s));
    Uniform1fv::load_with(|s| loadfn(s));
    GetNamedBufferSubData::load_with(|s| loadfn(s));
    TransformFeedbackVaryings::load_with(|s| loadfn(s));
    InvalidateNamedFramebufferSubData::load_with(|s| loadfn(s));
    PointParameteri::load_with(|s| loadfn(s));
    GetTexParameterfv::load_with(|s| loadfn(s));
    IsTransformFeedback::load_with(|s| loadfn(s));
    TextureStorage3D::load_with(|s| loadfn(s));
    ClearNamedBufferSubData::load_with(|s| loadfn(s));
    GetBufferSubData::load_with(|s| loadfn(s));
    VertexAttrib4fv::load_with(|s| loadfn(s));
    GetVertexAttribIiv::load_with(|s| loadfn(s));
    GetDebugMessageLog::load_with(|s| loadfn(s));
    UniformBlockBinding::load_with(|s| loadfn(s));
    MapBuffer::load_with(|s| loadfn(s));
    NamedFramebufferDrawBuffers::load_with(|s| loadfn(s));
    VertexAttribP1uiv::load_with(|s| loadfn(s));
    ClientWaitSync::load_with(|s| loadfn(s));
    GetSamplerParameterIuiv::load_with(|s| loadfn(s));
    ProgramUniformMatrix4x2fv::load_with(|s| loadfn(s));
    VertexAttribI4bv::load_with(|s| loadfn(s));
    GenFramebuffers::load_with(|s| loadfn(s));
    GetVertexAttribIuiv::load_with(|s| loadfn(s));
    ProgramUniformMatrix2x3dv::load_with(|s| loadfn(s));
    BufferSubData::load_with(|s| loadfn(s));
    VertexAttrib3f::load_with(|s| loadfn(s));
    TexImage3DMultisample::load_with(|s| loadfn(s));
    GetTexParameteriv::load_with(|s| loadfn(s));
    GetnConvolutionFilter::load_with(|s| loadfn(s));
    VertexAttrib4bv::load_with(|s| loadfn(s));
    GetDoublei_v::load_with(|s| loadfn(s));
    DeleteSync::load_with(|s| loadfn(s));
    FlushMappedNamedBufferRange::load_with(|s| loadfn(s));
    GetActiveUniformName::load_with(|s| loadfn(s));
    ProgramUniform1uiv::load_with(|s| loadfn(s));
    ProgramBinary::load_with(|s| loadfn(s));
    GenerateTextureMipmap::load_with(|s| loadfn(s));
    DepthRangeArrayv::load_with(|s| loadfn(s));
    ProgramUniform2d::load_with(|s| loadfn(s));
    CheckNamedFramebufferStatus::load_with(|s| loadfn(s));
    ResumeTransformFeedback::load_with(|s| loadfn(s));
    VertexAttribBinding::load_with(|s| loadfn(s));
    PixelStoref::load_with(|s| loadfn(s));
    MultiTexCoordP1ui::load_with(|s| loadfn(s));
    GetSamplerParameterfv::load_with(|s| loadfn(s));
    GetTexParameterIuiv::load_with(|s| loadfn(s));
    ClipControl::load_with(|s| loadfn(s));
    GetSubroutineIndex::load_with(|s| loadfn(s));
    GenBuffers::load_with(|s| loadfn(s));
    GetSamplerParameterIiv::load_with(|s| loadfn(s));
    Uniform3dv::load_with(|s| loadfn(s));
    ProgramUniformMatrix3x4fv::load_with(|s| loadfn(s));
    LineWidth::load_with(|s| loadfn(s));
    VertexArrayAttribLFormat::load_with(|s| loadfn(s));
    DepthRangeIndexed::load_with(|s| loadfn(s));
    ProgramUniformMatrix3x4dv::load_with(|s| loadfn(s));
    GetTextureParameteriv::load_with(|s| loadfn(s));
    DrawElementsInstancedBaseInstance::load_with(|s| loadfn(s));
    GetVertexAttribLdv::load_with(|s| loadfn(s));
    VertexP3ui::load_with(|s| loadfn(s));
    ClearNamedFramebufferfi::load_with(|s| loadfn(s));
    DrawTransformFeedbackStreamInstanced::load_with(|s| loadfn(s));
    ProgramUniform3ui::load_with(|s| loadfn(s));
    GetTextureLevelParameteriv::load_with(|s| loadfn(s));
    Uniform2dv::load_with(|s| loadfn(s));
    GetQueryObjectui64v::load_with(|s| loadfn(s));
    ProgramUniform2fv::load_with(|s| loadfn(s));
    MultiTexCoordP1uiv::load_with(|s| loadfn(s));
    RenderbufferStorageMultisample::load_with(|s| loadfn(s));
    ColorP3uiv::load_with(|s| loadfn(s));
    MultiTexCoordP2ui::load_with(|s| loadfn(s));
    BindFragDataLocation::load_with(|s| loadfn(s));
    Uniform4uiv::load_with(|s| loadfn(s));
    GetFramebufferAttachmentParameteriv::load_with(|s| loadfn(s));
    GetVertexArrayIndexediv::load_with(|s| loadfn(s));
    TexParameterIiv::load_with(|s| loadfn(s));
    GetNamedBufferParameteri64v::load_with(|s| loadfn(s));
    UniformMatrix3fv::load_with(|s| loadfn(s));
    ClearBufferData::load_with(|s| loadfn(s));
    VertexP4uiv::load_with(|s| loadfn(s));
    CopyImageSubData::load_with(|s| loadfn(s));
    Uniform4dv::load_with(|s| loadfn(s));
    GenTextures::load_with(|s| loadfn(s));
    TexCoordP2uiv::load_with(|s| loadfn(s));
    VertexAttribL3dv::load_with(|s| loadfn(s));
    CompressedTexImage1D::load_with(|s| loadfn(s));
    GetTextureParameterIuiv::load_with(|s| loadfn(s));
    InvalidateTexSubImage::load_with(|s| loadfn(s));
    FenceSync::load_with(|s| loadfn(s));
    VertexAttribL1d::load_with(|s| loadfn(s));
    UniformMatrix4x2dv::load_with(|s| loadfn(s));
    PauseTransformFeedback::load_with(|s| loadfn(s));
    VertexAttrib4iv::load_with(|s| loadfn(s));
    FramebufferTextureLayer::load_with(|s| loadfn(s));
    TextureSubImage2D::load_with(|s| loadfn(s));
    ColorP4ui::load_with(|s| loadfn(s));
    TexParameterfv::load_with(|s| loadfn(s));
    PushDebugGroup::load_with(|s| loadfn(s));
    MinSampleShading::load_with(|s| loadfn(s));
    BindFragDataLocationIndexed::load_with(|s| loadfn(s));
    ScissorIndexed::load_with(|s| loadfn(s));
    VertexAttrib1d::load_with(|s| loadfn(s));
    LogicOp::load_with(|s| loadfn(s));
    GetBooleani_v::load_with(|s| loadfn(s));
    GetActiveUniform::load_with(|s| loadfn(s));
    VertexAttrib2fv::load_with(|s| loadfn(s));
    Uniform4ui::load_with(|s| loadfn(s));
    ProgramUniform3d::load_with(|s| loadfn(s));
    VertexAttribI1i::load_with(|s| loadfn(s));
    VertexAttribPointer::load_with(|s| loadfn(s));
    GetUniformLocation::load_with(|s| loadfn(s));
    CreateFramebuffers::load_with(|s| loadfn(s));
    BindSamplers::load_with(|s| loadfn(s));
    GetProgramResourceIndex::load_with(|s| loadfn(s));
    GetTexParameterIiv::load_with(|s| loadfn(s));
    GetQueryObjectiv::load_with(|s| loadfn(s));
    VertexAttrib4Nbv::load_with(|s| loadfn(s));
    GetString::load_with(|s| loadfn(s));
    MultiTexCoordP4ui::load_with(|s| loadfn(s));
    ProgramUniformMatrix4dv::load_with(|s| loadfn(s));
    ColorMaski::load_with(|s| loadfn(s));
    BindFramebuffer::load_with(|s| loadfn(s));
    GetSubroutineUniformLocation::load_with(|s| loadfn(s));
    NamedFramebufferTexture::load_with(|s| loadfn(s));
    SamplerParameterIiv::load_with(|s| loadfn(s));
    TexCoordP3ui::load_with(|s| loadfn(s));
    FramebufferRenderbuffer::load_with(|s| loadfn(s));
    GetProgramResourceName::load_with(|s| loadfn(s));
    ProgramUniform3uiv::load_with(|s| loadfn(s));
    CompressedTexSubImage1D::load_with(|s| loadfn(s));
    TextureParameterIuiv::load_with(|s| loadfn(s));
    UniformMatrix3x2dv::load_with(|s| loadfn(s));
    GetTextureParameterIiv::load_with(|s| loadfn(s));
    PrimitiveRestartIndex::load_with(|s| loadfn(s));
    StencilMaskSeparate::load_with(|s| loadfn(s));
    ProgramUniform4d::load_with(|s| loadfn(s));
    DepthRange::load_with(|s| loadfn(s));
    StencilFunc::load_with(|s| loadfn(s));
    DrawElementsBaseVertex::load_with(|s| loadfn(s));
    Uniform4iv::load_with(|s| loadfn(s));
    ProgramUniform1f::load_with(|s| loadfn(s));
    VertexAttribI3uiv::load_with(|s| loadfn(s));
    CompressedTextureSubImage2D::load_with(|s| loadfn(s));
    BlitFramebuffer::load_with(|s| loadfn(s));
    BeginQuery::load_with(|s| loadfn(s));
    UniformMatrix3dv::load_with(|s| loadfn(s));
    DisableVertexArrayAttrib::load_with(|s| loadfn(s));
    VertexAttrib4f::load_with(|s| loadfn(s));
    ObjectLabel::load_with(|s| loadfn(s));
    MultiTexCoordP3uiv::load_with(|s| loadfn(s));
    GetNamedFramebufferParameteriv::load_with(|s| loadfn(s));
    EndQuery::load_with(|s| loadfn(s));
    ProgramUniform1d::load_with(|s| loadfn(s));
    VertexAttribP3uiv::load_with(|s| loadfn(s));
    GetInternalformativ::load_with(|s| loadfn(s));
    ClearBufferSubData::load_with(|s| loadfn(s));
    BeginConditionalRender::load_with(|s| loadfn(s));
    DrawArrays::load_with(|s| loadfn(s));
    TexImage2D::load_with(|s| loadfn(s));
    DeleteSamplers::load_with(|s| loadfn(s));
    ProgramUniformMatrix2x4fv::load_with(|s| loadfn(s));
    GetTransformFeedbackiv::load_with(|s| loadfn(s));
    GetFragDataIndex::load_with(|s| loadfn(s));
    GetProgramPipelineiv::load_with(|s| loadfn(s));
}
