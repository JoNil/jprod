#![allow(clippy::too_many_arguments)]
#![allow(clippy::upper_case_acronyms)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(static_mut_refs)]

use crate::c_types::*;
use crate::win32;
use core::mem;

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

pub type GLDEBUGPROC = extern "system" fn(
    source: GLenum,
    gltype: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut c_void,
);
pub type GLDEBUGPROCARB = extern "system" fn(
    source: GLenum,
    gltype: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut c_void,
);
pub type GLDEBUGPROCKHR = extern "system" fn(
    source: GLenum,
    gltype: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut c_void,
);

// Vendor extension types
pub type GLDEBUGPROCAMD = extern "system" fn(
    id: GLuint,
    category: GLenum,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut c_void,
);
pub type GLhalfNV = c_ushort;
pub type GLvdpauSurfaceNV = GLintptr;

pub const SAMPLE_ALPHA_TO_COVERAGE: GLenum = 0x809E;
pub const INT_IMAGE_2D: GLenum = 0x9058;
pub const GEOMETRY_SUBROUTINE_UNIFORM: GLenum = 0x92F1;
pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: GLenum = 0x906B;
pub const MAX_TESS_PATCH_COMPONENTS: GLenum = 0x8E84;
pub const TEXTURE_COMPARE_MODE: GLenum = 0x884C;
pub const UNSIGNED_INT_24_8: GLenum = 0x84FA;
pub const UNPACK_LSB_FIRST: GLenum = 0x0CF1;
pub const DEBUG_SEVERITY_MEDIUM: GLenum = 0x9147;
pub const COLOR_ATTACHMENT18: GLenum = 0x8CF2;
pub const LOCATION: GLenum = 0x930E;
pub const DEBUG_TYPE_POP_GROUP: GLenum = 0x826A;
pub const MAX_VERTEX_STREAMS: GLenum = 0x8E71;
pub const TEXTURE25: GLenum = 0x84D9;
pub const INT_SAMPLER_CUBE: GLenum = 0x8DCC;
pub const HIGH_FLOAT: GLenum = 0x8DF2;
pub const MAX_RECTANGLE_TEXTURE_SIZE: GLenum = 0x84F8;
pub const TEXTURE_IMAGE_TYPE: GLenum = 0x8290;
pub const MAX_FRAMEBUFFER_HEIGHT: GLenum = 0x9316;
pub const UNSIGNED_INT_VEC4: GLenum = 0x8DC8;
pub const TEXTURE_GATHER_SHADOW: GLenum = 0x82A3;
pub const CONDITION_SATISFIED: GLenum = 0x911C;
pub const DITHER: GLenum = 0x0BD0;
pub const SAMPLE_SHADING: GLenum = 0x8C36;
pub const STENCIL_REF: GLenum = 0x0B97;
pub const TEXTURE_VIEW_NUM_LEVELS: GLenum = 0x82DC;
pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 0x8CD7;
pub const DOUBLEBUFFER: GLenum = 0x0C32;
pub const VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 0x8623;
pub const TEXTURE6: GLenum = 0x84C6;
pub const QUERY_BUFFER_BARRIER_BIT: GLenum = 0x00008000;
pub const UNPACK_SKIP_IMAGES: GLenum = 0x806D;
pub const DEPTH_TEST: GLenum = 0x0B71;
pub const PRIMITIVES_GENERATED: GLenum = 0x8C87;
pub const MAX_FRAMEBUFFER_SAMPLES: GLenum = 0x9318;
pub const TEXTURE_LOD_BIAS: GLenum = 0x8501;
pub const REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x930A;
pub const MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CE;
pub const SAMPLER_BINDING: GLenum = 0x8919;
pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = 0x8211;
pub const INTERNALFORMAT_DEPTH_SIZE: GLenum = 0x8275;
pub const R8: GLenum = 0x8229;
pub const DRAW_BUFFER2: GLenum = 0x8827;
pub const DEPTH_STENCIL_ATTACHMENT: GLenum = 0x821A;
pub const PROGRAM_INPUT: GLenum = 0x92E3;
pub const CONTEXT_FLAGS: GLenum = 0x821E;
pub const MAX_COLOR_ATTACHMENTS: GLenum = 0x8CDF;
pub const SRC_COLOR: GLenum = 0x0300;
pub const DECR: GLenum = 0x1E03;
pub const FILL: GLenum = 0x1B02;
pub const MAX_INTEGER_SAMPLES: GLenum = 0x9110;
pub const UNSIGNED_SHORT_5_6_5: GLenum = 0x8363;
pub const DOUBLE_VEC4: GLenum = 0x8FFE;
pub const UNSIGNED_INT_SAMPLER_CUBE: GLenum = 0x8DD4;
pub const FLOAT_MAT4x3: GLenum = 0x8B6A;
pub const DRAW_BUFFER5: GLenum = 0x882A;
pub const FRAMEBUFFER_RENDERABLE: GLenum = 0x8289;
pub const UNSIGNED_INT: GLenum = 0x1405;
pub const TEXTURE_BINDING_1D: GLenum = 0x8068;
pub const PROGRAM_SEPARABLE: GLenum = 0x8258;
pub const TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 0x8517;
pub const BUFFER_MAP_POINTER: GLenum = 0x88BD;
pub const DRAW_BUFFER3: GLenum = 0x8828;
pub const UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x906A;
pub const DEBUG_SOURCE_WINDOW_SYSTEM: GLenum = 0x8247;
pub const COMPRESSED_SIGNED_R11_EAC: GLenum = 0x9271;
pub const FRAGMENT_SUBROUTINE_UNIFORM: GLenum = 0x92F2;
pub const MAX_IMAGE_UNITS: GLenum = 0x8F38;
pub const DOUBLE_MAT4x2: GLenum = 0x8F4D;
pub const TEXTURE_TARGET: GLenum = 0x1006;
pub const FLOAT_VEC4: GLenum = 0x8B52;
pub const PACK_SKIP_ROWS: GLenum = 0x0D03;
pub const BGRA_INTEGER: GLenum = 0x8D9B;
pub const MAX_VARYING_VECTORS: GLenum = 0x8DFC;
pub const TEXTURE31: GLenum = 0x84DF;
pub const TRANSFORM_FEEDBACK: GLenum = 0x8E22;
pub const COMPRESSED_SIGNED_RED_RGTC1: GLenum = 0x8DBC;
pub const CURRENT_VERTEX_ATTRIB: GLenum = 0x8626;
pub const ACTIVE_SUBROUTINE_MAX_LENGTH: GLenum = 0x8E48;
pub const SRGB: GLenum = 0x8C40;
pub const VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D5;
pub const NEAREST: GLenum = 0x2600;
pub const FUNC_REVERSE_SUBTRACT: GLenum = 0x800B;
pub const DST_COLOR: GLenum = 0x0306;
pub const MAX_VARYING_FLOATS: GLenum = 0x8B4B;
pub const READ_FRAMEBUFFER: GLenum = 0x8CA8;
pub const BACK_RIGHT: GLenum = 0x0403;
pub const ARRAY_BUFFER_BINDING: GLenum = 0x8894;
pub const RG8UI: GLenum = 0x8238;
pub const UNSIGNED_BYTE_3_3_2: GLenum = 0x8032;
pub const UNSIGNED_SHORT_5_6_5_REV: GLenum = 0x8364;
pub const PROGRAM_PIPELINE: GLenum = 0x82E4;
pub const INT_SAMPLER_1D_ARRAY: GLenum = 0x8DCE;
pub const COLOR_RENDERABLE: GLenum = 0x8286;
pub const VIEW_CLASS_16_BITS: GLenum = 0x82CA;
pub const SHADER_SOURCE_LENGTH: GLenum = 0x8B88;
pub const MAX_TESS_CONTROL_UNIFORM_BLOCKS: GLenum = 0x8E89;
pub const COMPRESSED_SRGB_ALPHA_BPTC_UNORM: GLenum = 0x8E8D;
pub const RGBA32I: GLenum = 0x8D82;
pub const FRAGMENT_SUBROUTINE: GLenum = 0x92EC;
pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x84F0;
pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = 0x8214;
pub const UNIFORM_BLOCK_INDEX: GLenum = 0x8A3A;
pub const RGBA8: GLenum = 0x8058;
pub const MIN: GLenum = 0x8007;
pub const FRONT_AND_BACK: GLenum = 0x0408;
pub const OFFSET: GLenum = 0x92FC;
pub const VIEW_CLASS_128_BITS: GLenum = 0x82C4;
pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x8A44;
pub const BACK_LEFT: GLenum = 0x0402;
pub const NEGATIVE_ONE_TO_ONE: GLenum = 0x935E;
pub const COPY: GLenum = 0x1503;
pub const PIXEL_UNPACK_BUFFER: GLenum = 0x88EC;
pub const TEXTURE_WRAP_S: GLenum = 0x2802;
pub const FRAGMENT_SHADER: GLenum = 0x8B30;
pub const R8_SNORM: GLenum = 0x8F94;
pub const COMPRESSED_RED: GLenum = 0x8225;
pub const COPY_READ_BUFFER: GLenum = 0x8F36;
pub const KEEP: GLenum = 0x1E00;
pub const COPY_INVERTED: GLenum = 0x150C;
pub const DOUBLE_MAT3x2: GLenum = 0x8F4B;
pub const DRAW_BUFFER12: GLenum = 0x8831;
pub const UNSIGNED_BYTE: GLenum = 0x1401;
pub const TEXTURE_HEIGHT: GLenum = 0x1001;
pub const FRAMEBUFFER_UNSUPPORTED: GLenum = 0x8CDD;
pub const PRIMITIVE_RESTART_FIXED_INDEX: GLenum = 0x8D69;
pub const UNSIGNED_SHORT_4_4_4_4: GLenum = 0x8033;
pub const ALL_BARRIER_BITS: GLenum = 0xFFFFFFFF;
pub const FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
pub const MIRROR_CLAMP_TO_EDGE: GLenum = 0x8743;
pub const DEBUG_SOURCE_OTHER: GLenum = 0x824B;
pub const BUFFER_USAGE: GLenum = 0x8765;
pub const UNIFORM_BUFFER_START: GLenum = 0x8A29;
pub const INT_2_10_10_10_REV: GLenum = 0x8D9F;
pub const COLOR_ATTACHMENT23: GLenum = 0x8CF7;
pub const TEXTURE_WRAP_T: GLenum = 0x2803;
pub const INNOCENT_CONTEXT_RESET: GLenum = 0x8254;
pub const DRAW_BUFFER: GLenum = 0x0C01;
pub const ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x9301;
pub const INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910C;
pub const STENCIL_FAIL: GLenum = 0x0B94;
pub const RENDERBUFFER_HEIGHT: GLenum = 0x8D43;
pub const RENDERBUFFER_INTERNAL_FORMAT: GLenum = 0x8D44;
pub const DEPTH_CLEAR_VALUE: GLenum = 0x0B73;
pub const PROXY_TEXTURE_RECTANGLE: GLenum = 0x84F7;
pub const STENCIL: GLenum = 0x1802;
pub const RENDERBUFFER_RED_SIZE: GLenum = 0x8D50;
pub const RGBA16: GLenum = 0x805B;
pub const DOUBLE_MAT2x4: GLenum = 0x8F4A;
pub const LINE_LOOP: GLenum = 0x0002;
pub const SHADER_STORAGE_BLOCK: GLenum = 0x92E6;
pub const BUFFER_ACCESS: GLenum = 0x88BB;
pub const TEXTURE_MAG_FILTER: GLenum = 0x2800;
pub const TEXTURE17: GLenum = 0x84D1;
pub const MAX_GEOMETRY_INPUT_COMPONENTS: GLenum = 0x9123;
pub const DEPTH_COMPONENT: GLenum = 0x1902;
pub const RGBA8I: GLenum = 0x8D8E;
pub const TEXTURE_COMPRESSED_IMAGE_SIZE: GLenum = 0x86A0;
pub const DONT_CARE: GLenum = 0x1100;
pub const RG32F: GLenum = 0x8230;
pub const IMAGE_2D: GLenum = 0x904D;
pub const IMAGE_BINDING_LAYER: GLenum = 0x8F3D;
pub const STENCIL_WRITEMASK: GLenum = 0x0B98;
pub const DEPTH_STENCIL: GLenum = 0x84F9;
pub const IMAGE_CLASS_1_X_32: GLenum = 0x82BB;
pub const BLEND_SRC_RGB: GLenum = 0x80C9;
pub const TEXTURE_VIEW_MIN_LAYER: GLenum = 0x82DD;
pub const MAX_VERTEX_UNIFORM_BLOCKS: GLenum = 0x8A2B;
pub const UNSIGNED_INT_5_9_9_9_REV: GLenum = 0x8C3E;
pub const FLOAT_MAT3x4: GLenum = 0x8B68;
pub const MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: GLenum = 0x90D8;
pub const TEXTURE29: GLenum = 0x84DD;
pub const CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: GLenum = 0x00000001;
pub const COMPUTE_WORK_GROUP_SIZE: GLenum = 0x8267;
pub const CLIP_DISTANCE1: GLenum = 0x3001;
pub const QUERY_WAIT_INVERTED: GLenum = 0x8E17;
pub const VIEW_CLASS_48_BITS: GLenum = 0x82C7;
pub const IMAGE_CLASS_2_X_16: GLenum = 0x82BD;
pub const INCR: GLenum = 0x1E02;
pub const UNSIGNED_INT_IMAGE_1D_ARRAY: GLenum = 0x9068;
pub const SAMPLE_COVERAGE_INVERT: GLenum = 0x80AB;
pub const CLEAR_TEXTURE: GLenum = 0x9365;
pub const NUM_SHADER_BINARY_FORMATS: GLenum = 0x8DF9;
pub const UNKNOWN_CONTEXT_RESET: GLenum = 0x8255;
pub const IMAGE_PIXEL_FORMAT: GLenum = 0x82A9;
pub const RGB16F: GLenum = 0x881B;
pub const CONTEXT_FLAG_ROBUST_ACCESS_BIT: GLenum = 0x00000004;
pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 0x8A42;
pub const TEXTURE_BINDING_BUFFER: GLenum = 0x8C2C;
pub const MAP_INVALIDATE_RANGE_BIT: GLenum = 0x0004;
pub const QUERY_BY_REGION_NO_WAIT: GLenum = 0x8E16;
pub const INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;
pub const MEDIUM_INT: GLenum = 0x8DF4;
pub const BLEND_DST_RGB: GLenum = 0x80C8;
pub const MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: GLenum = 0x8F39;
pub const UNSIGNED_INT_ATOMIC_COUNTER: GLenum = 0x92DB;
pub const READ_PIXELS_TYPE: GLenum = 0x828E;
pub const TEXTURE2: GLenum = 0x84C2;
pub const PROGRAM_BINARY_RETRIEVABLE_HINT: GLenum = 0x8257;
pub const MAX_FRAMEBUFFER_WIDTH: GLenum = 0x9315;
pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: GLenum = 0x82AE;
pub const IMAGE_1D: GLenum = 0x904C;
pub const TESS_EVALUATION_SUBROUTINE: GLenum = 0x92EA;
pub const VERTEX_SUBROUTINE_UNIFORM: GLenum = 0x92EE;
pub const MAX_SAMPLES: GLenum = 0x8D57;
pub const TEXTURE_IMMUTABLE_LEVELS: GLenum = 0x82DF;
pub const DEBUG_TYPE_PORTABILITY: GLenum = 0x824F;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = 0x8CD4;
pub const SHADER_IMAGE_LOAD: GLenum = 0x82A4;
pub const HIGH_INT: GLenum = 0x8DF5;
pub const MAX_CULL_DISTANCES: GLenum = 0x82F9;
pub const ONE_MINUS_DST_COLOR: GLenum = 0x0307;
pub const ATOMIC_COUNTER_BARRIER_BIT: GLenum = 0x00001000;
pub const MAX_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5F;
pub const VERSION: GLenum = 0x1F02;
pub const TEXTURE_VIEW_MIN_LEVEL: GLenum = 0x82DB;
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = 0x8C80;
pub const TEXTURE_FETCH_BARRIER_BIT: GLenum = 0x00000008;
pub const RGB8_SNORM: GLenum = 0x8F96;
pub const SHADER_STORAGE_BUFFER_SIZE: GLenum = 0x90D5;
pub const STENCIL_PASS_DEPTH_PASS: GLenum = 0x0B96;
pub const IS_ROW_MAJOR: GLenum = 0x9300;
pub const PROXY_TEXTURE_3D: GLenum = 0x8070;
pub const PACK_COMPRESSED_BLOCK_SIZE: GLenum = 0x912E;
pub const UNDEFINED_VERTEX: GLenum = 0x8260;
pub const IMAGE_COMPATIBILITY_CLASS: GLenum = 0x82A8;
pub const COLOR_LOGIC_OP: GLenum = 0x0BF2;
pub const UNPACK_IMAGE_HEIGHT: GLenum = 0x806E;
pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = 0x8215;
pub const TEXTURE15: GLenum = 0x84CF;
pub const MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D0;
pub const MIN_SAMPLE_SHADING_VALUE: GLenum = 0x8C37;
pub const COLOR_CLEAR_VALUE: GLenum = 0x0C22;
pub const INT_IMAGE_CUBE: GLenum = 0x905B;
pub const SHADER_COMPILER: GLenum = 0x8DFA;
pub const READ_PIXELS: GLenum = 0x828C;
pub const COLOR_ATTACHMENT10: GLenum = 0x8CEA;
pub const PACK_SKIP_PIXELS: GLenum = 0x0D04;
pub const VERTEX_TEXTURE: GLenum = 0x829B;
pub const IMAGE_CLASS_1_X_8: GLenum = 0x82C1;
pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910D;
pub const MAX_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8263;
pub const MAX_3D_TEXTURE_SIZE: GLenum = 0x8073;
pub const POLYGON_OFFSET_POINT: GLenum = 0x2A01;
pub const MAX_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E7F;
pub const FLOAT_MAT2x4: GLenum = 0x8B66;
pub const CW: GLenum = 0x0900;
pub const DEBUG_SEVERITY_HIGH: GLenum = 0x9146;
pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = 0x8C88;
pub const VERTEX_ARRAY: GLenum = 0x8074;
pub const INTERNALFORMAT_RED_TYPE: GLenum = 0x8278;
pub const MAX_TESS_EVALUATION_UNIFORM_BLOCKS: GLenum = 0x8E8A;
pub const COLOR_ATTACHMENT15: GLenum = 0x8CEF;
pub const UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90EC;
pub const TEXTURE12: GLenum = 0x84CC;
pub const CONTEXT_FLAG_DEBUG_BIT: GLenum = 0x00000002;
pub const SHADER_IMAGE_ACCESS_BARRIER_BIT: GLenum = 0x00000020;
pub const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: GLenum = 0x8243;
pub const MAX_TESS_EVALUATION_ATOMIC_COUNTERS: GLenum = 0x92D4;
pub const SCISSOR_BOX: GLenum = 0x0C10;
pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: GLenum = 0x82AF;
pub const RENDERBUFFER_SAMPLES: GLenum = 0x8CAB;
pub const FIXED: GLenum = 0x140C;
pub const COMPRESSED_RGB8_ETC2: GLenum = 0x9274;
pub const SRC1_COLOR: GLenum = 0x88F9;
pub const FLOAT_MAT2: GLenum = 0x8B5A;
pub const SAMPLES: GLenum = 0x80A9;
pub const OUT_OF_MEMORY: GLenum = 0x0505;
pub const R8UI: GLenum = 0x8232;
pub const BUFFER_SIZE: GLenum = 0x8764;
pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = 0x8B8A;
pub const MAX_GEOMETRY_OUTPUT_COMPONENTS: GLenum = 0x9124;
pub const MINOR_VERSION: GLenum = 0x821C;
pub const ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
pub const MIPMAP: GLenum = 0x8293;
pub const INT_SAMPLER_1D: GLenum = 0x8DC9;
pub const TESS_CONTROL_SUBROUTINE: GLenum = 0x92E9;
pub const RENDERBUFFER_ALPHA_SIZE: GLenum = 0x8D53;
pub const DEBUG_TYPE_OTHER: GLenum = 0x8251;
pub const TEXTURE_GATHER: GLenum = 0x82A2;
pub const PACK_ROW_LENGTH: GLenum = 0x0D02;
pub const INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x9109;
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x92C8;
pub const TEXTURE_MIN_LOD: GLenum = 0x813A;
pub const PACK_COMPRESSED_BLOCK_DEPTH: GLenum = 0x912D;
pub const COMPRESSED_SRGB: GLenum = 0x8C48;
pub const COLOR_ATTACHMENT20: GLenum = 0x8CF4;
pub const COLOR_ATTACHMENT24: GLenum = 0x8CF8;
pub const ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: GLenum = 0x8E49;
pub const TRUE: GLboolean = 1;
pub const MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 0x8DFD;
pub const QUERY_RESULT_AVAILABLE: GLenum = 0x8867;
pub const BACK: GLenum = 0x0405;
pub const FRAMEBUFFER_DEFAULT_HEIGHT: GLenum = 0x9311;
pub const SHADER_BINARY_FORMATS: GLenum = 0x8DF8;
pub const QUADS: GLenum = 0x0007;
pub const TEXTURE_BINDING_2D_ARRAY: GLenum = 0x8C1D;
pub const SRC_ALPHA: GLenum = 0x0302;
pub const DEBUG_TYPE_PUSH_GROUP: GLenum = 0x8269;
pub const DRAW_BUFFER7: GLenum = 0x882C;
pub const TRANSFORM_FEEDBACK_ACTIVE: GLenum = 0x8E24;
pub const BLEND_SRC_ALPHA: GLenum = 0x80CB;
pub const DEBUG_TYPE_PERFORMANCE: GLenum = 0x8250;
pub const LINEAR_MIPMAP_NEAREST: GLenum = 0x2701;
pub const INTERLEAVED_ATTRIBS: GLenum = 0x8C8C;
pub const TEXTURE_BINDING_RECTANGLE: GLenum = 0x84F6;
pub const STENCIL_INDEX: GLenum = 0x1901;
pub const FRAMEBUFFER_BARRIER_BIT: GLenum = 0x00000400;
pub const TESS_CONTROL_SUBROUTINE_UNIFORM: GLenum = 0x92EF;
pub const STREAM_COPY: GLenum = 0x88E2;
pub const TEXTURE_BLUE_SIZE: GLenum = 0x805E;
pub const MAX_COMPUTE_WORK_GROUP_SIZE: GLenum = 0x91BF;
pub const LOW_INT: GLenum = 0x8DF3;
pub const FRAMEBUFFER_DEFAULT_SAMPLES: GLenum = 0x9313;
pub const CLIENT_STORAGE_BIT: GLenum = 0x0200;
pub const LOSE_CONTEXT_ON_RESET: GLenum = 0x8252;
pub const IMAGE_BINDING_FORMAT: GLenum = 0x906E;
pub const DEBUG_CALLBACK_FUNCTION: GLenum = 0x8244;
pub const SAMPLE_MASK: GLenum = 0x8E51;
pub const MAX_TESS_EVALUATION_IMAGE_UNIFORMS: GLenum = 0x90CC;
pub const QUERY_RESULT_NO_WAIT: GLenum = 0x9194;
pub const UNSIGNED_SHORT_1_5_5_5_REV: GLenum = 0x8366;
pub const VIEW_CLASS_32_BITS: GLenum = 0x82C8;
pub const MAX_TESS_CONTROL_ATOMIC_COUNTERS: GLenum = 0x92D3;
pub const QUERY_NO_WAIT_INVERTED: GLenum = 0x8E18;
pub const PATCH_DEFAULT_INNER_LEVEL: GLenum = 0x8E73;
pub const READ_WRITE: GLenum = 0x88BA;
pub const COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9276;
pub const IMAGE_CLASS_4_X_16: GLenum = 0x82BC;
pub const RG8_SNORM: GLenum = 0x8F95;
pub const VERTEX_ATTRIB_ARRAY_BARRIER_BIT: GLenum = 0x00000001;
pub const TEXTURE_SHADOW: GLenum = 0x82A1;
pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLenum = 0x8C76;
pub const IMAGE_CLASS_4_X_32: GLenum = 0x82B9;
pub const TEXTURE_BASE_LEVEL: GLenum = 0x813C;
pub const TEXTURE_BINDING_CUBE_MAP_ARRAY: GLenum = 0x900A;
pub const CLEAR_BUFFER: GLenum = 0x82B4;
pub const MAP_COHERENT_BIT: GLenum = 0x0080;
pub const FRAMEBUFFER_UNDEFINED: GLenum = 0x8219;
pub const MAX_DEPTH_TEXTURE_SAMPLES: GLenum = 0x910F;
pub const ACTIVE_UNIFORMS: GLenum = 0x8B86;
pub const SYNC_STATUS: GLenum = 0x9114;
pub const SAMPLER: GLenum = 0x82E6;
pub const ISOLINES: GLenum = 0x8E7A;
pub const VERTEX_BINDING_BUFFER: GLenum = 0x8F4F;
pub const RGB16_SNORM: GLenum = 0x8F9A;
pub const MAX_VERTEX_UNIFORM_VECTORS: GLenum = 0x8DFB;
pub const RGBA8_SNORM: GLenum = 0x8F97;
pub const R16I: GLenum = 0x8233;
pub const FLOAT_MAT3: GLenum = 0x8B5B;
pub const UNPACK_COMPRESSED_BLOCK_WIDTH: GLenum = 0x9127;
pub const VERTEX_BINDING_OFFSET: GLenum = 0x82D7;
pub const COLOR_ATTACHMENT12: GLenum = 0x8CEC;
pub const TRANSFORM_FEEDBACK_BUFFER: GLenum = 0x8C8E;
pub const SMOOTH_LINE_WIDTH_RANGE: GLenum = 0x0B22;
pub const TEXTURE0: GLenum = 0x84C0;
pub const INT_IMAGE_2D_MULTISAMPLE: GLenum = 0x9060;
pub const RENDERER: GLenum = 0x1F01;
pub const STENCIL_INDEX1: GLenum = 0x8D46;
pub const TEXTURE: GLenum = 0x1702;
pub const DEBUG_SEVERITY_NOTIFICATION: GLenum = 0x826B;
pub const VIEW_CLASS_S3TC_DXT5_RGBA: GLenum = 0x82CF;
pub const VERTEX_SHADER: GLenum = 0x8B31;
pub const TEXTURE5: GLenum = 0x84C5;
pub const TESS_CONTROL_OUTPUT_VERTICES: GLenum = 0x8E75;
pub const CLIENT_MAPPED_BUFFER_BARRIER_BIT: GLenum = 0x00004000;
pub const SRGB8: GLenum = 0x8C41;
pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: GLenum = 0x92C5;
pub const ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
pub const BLUE_INTEGER: GLenum = 0x8D96;
pub const TIMESTAMP: GLenum = 0x8E28;
pub const STATIC_READ: GLenum = 0x88E5;
pub const PATCHES: GLenum = 0x000E;
pub const TESS_EVALUATION_TEXTURE: GLenum = 0x829D;
pub const MAX_DRAW_BUFFERS: GLenum = 0x8824;
pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 0x886A;
pub const INT_VEC3: GLenum = 0x8B54;
pub const TEXTURE_3D: GLenum = 0x806F;
pub const ATTACHED_SHADERS: GLenum = 0x8B85;
pub const DYNAMIC_STORAGE_BIT: GLenum = 0x0100;
pub const INTERNALFORMAT_ALPHA_SIZE: GLenum = 0x8274;
pub const PROXY_TEXTURE_1D: GLenum = 0x8063;
pub const TEXTURE_RED_SIZE: GLenum = 0x805C;
pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = 0x8C8F;
pub const UNSIGNED_INT_SAMPLER_2D: GLenum = 0x8DD2;
pub const RGBA2: GLenum = 0x8055;
pub const IMAGE_3D: GLenum = 0x904E;
pub const MAX_COMBINED_DIMENSIONS: GLenum = 0x8282;
pub const READ_FRAMEBUFFER_BINDING: GLenum = 0x8CAA;
pub const VERTEX_BINDING_DIVISOR: GLenum = 0x82D6;
pub const DEPTH: GLenum = 0x1801;
pub const REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x9307;
pub const INTERNALFORMAT_DEPTH_TYPE: GLenum = 0x827C;
pub const PIXEL_PACK_BUFFER: GLenum = 0x88EB;
pub const MAX_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 0x8DE8;
pub const MAX_ELEMENT_INDEX: GLenum = 0x8D6B;
pub const INT_VEC4: GLenum = 0x8B55;
pub const VIEWPORT_INDEX_PROVOKING_VERTEX: GLenum = 0x825F;
pub const COLOR_ATTACHMENT0: GLenum = 0x8CE0;
pub const SRGB_WRITE: GLenum = 0x8298;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 0x8518;
pub const TEXTURE_SAMPLES: GLenum = 0x9106;
pub const COLOR_ATTACHMENT6: GLenum = 0x8CE6;
pub const SHADER_TYPE: GLenum = 0x8B4F;
pub const TEXTURE_RECTANGLE: GLenum = 0x84F5;
pub const MAX_FRAGMENT_IMAGE_UNIFORMS: GLenum = 0x90CE;
pub const RGB4: GLenum = 0x804F;
pub const OBJECT_TYPE: GLenum = 0x9112;
pub const DYNAMIC_DRAW: GLenum = 0x88E8;
pub const POINT_FADE_THRESHOLD_SIZE: GLenum = 0x8128;
pub const COMPILE_STATUS: GLenum = 0x8B81;
pub const GEOMETRY_INPUT_TYPE: GLenum = 0x8917;
pub const TESS_EVALUATION_SUBROUTINE_UNIFORM: GLenum = 0x92F0;
pub const PROGRAM_BINARY_LENGTH: GLenum = 0x8741;
pub const BLEND_DST: GLenum = 0x0BE0;
pub const MAX_VARYING_COMPONENTS: GLenum = 0x8B4B;
pub const MAX_COMBINED_IMAGE_UNIFORMS: GLenum = 0x90CF;
pub const DYNAMIC_READ: GLenum = 0x88E9;
pub const IMAGE_TEXEL_SIZE: GLenum = 0x82A7;
pub const MAX_COMBINED_SHADER_OUTPUT_RESOURCES: GLenum = 0x8F39;
pub const COLOR_ENCODING: GLenum = 0x8296;
pub const TEXTURE_COMPRESSED: GLenum = 0x86A1;
pub const COLOR_ATTACHMENT28: GLenum = 0x8CFC;
pub const UNSIGNED_INT_10F_11F_11F_REV: GLenum = 0x8C3B;
pub const UNPACK_SKIP_ROWS: GLenum = 0x0CF3;
pub const COLOR_ATTACHMENT21: GLenum = 0x8CF5;
pub const REFERENCED_BY_VERTEX_SHADER: GLenum = 0x9306;
pub const MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8E85;
pub const BLEND_DST_ALPHA: GLenum = 0x80CA;
pub const NUM_SAMPLE_COUNTS: GLenum = 0x9380;
pub const RG: GLenum = 0x8227;
pub const TEXTURE24: GLenum = 0x84D8;
pub const INTERNALFORMAT_SUPPORTED: GLenum = 0x826F;
pub const LINEAR_MIPMAP_LINEAR: GLenum = 0x2703;
pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8B49;
pub const SAMPLER_1D: GLenum = 0x8B5D;
pub const COMPRESSED_RG11_EAC: GLenum = 0x9272;
pub const TESS_GEN_SPACING: GLenum = 0x8E77;
pub const COLOR_ATTACHMENT19: GLenum = 0x8CF3;
pub const MATRIX_STRIDE: GLenum = 0x92FF;
pub const SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900C;
pub const DEPTH_COMPONENTS: GLenum = 0x8284;
pub const DOUBLE_MAT4: GLenum = 0x8F48;
pub const DOUBLE_VEC2: GLenum = 0x8FFC;
pub const COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9277;
pub const IMAGE_2D_ARRAY: GLenum = 0x9053;
pub const SMOOTH_LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
pub const BOOL_VEC3: GLenum = 0x8B58;
pub const VERTEX_ARRAY_BINDING: GLenum = 0x85B5;
pub const DEBUG_SOURCE_API: GLenum = 0x8246;
pub const TEXTURE_2D_MULTISAMPLE: GLenum = 0x9100;
pub const TEXTURE_RED_TYPE: GLenum = 0x8C10;
pub const GET_TEXTURE_IMAGE_TYPE: GLenum = 0x8292;
pub const COMPATIBLE_SUBROUTINES: GLenum = 0x8E4B;
pub const INVALID_INDEX: GLuint = 0xFFFFFFFF;
pub const SHADER_STORAGE_BARRIER_BIT: GLenum = 0x00002000;
pub const LINE_SMOOTH: GLenum = 0x0B20;
pub const UNIFORM_BLOCK_NAME_LENGTH: GLenum = 0x8A41;
pub const HALF_FLOAT: GLenum = 0x140B;
pub const MAX_SUBROUTINES: GLenum = 0x8DE7;
pub const WAIT_FAILED: GLenum = 0x911D;
pub const BUFFER_UPDATE_BARRIER_BIT: GLenum = 0x00000200;
pub const UNIFORM_TYPE: GLenum = 0x8A37;
pub const MAX_FRAGMENT_ATOMIC_COUNTERS: GLenum = 0x92D6;
pub const LEQUAL: GLenum = 0x0203;
pub const COLOR_ATTACHMENT26: GLenum = 0x8CFA;
pub const UNPACK_SWAP_BYTES: GLenum = 0x0CF0;
pub const RG16_SNORM: GLenum = 0x8F99;
pub const VIEW_CLASS_S3TC_DXT1_RGB: GLenum = 0x82CC;
pub const TEXTURE_DEPTH_SIZE: GLenum = 0x884A;
pub const UNPACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x9128;
pub const MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x8264;
pub const TESS_GEN_MODE: GLenum = 0x8E76;
pub const FRAMEBUFFER: GLenum = 0x8D40;
pub const SAMPLER_CUBE_MAP_ARRAY_SHADOW: GLenum = 0x900D;
pub const OR_INVERTED: GLenum = 0x150D;
pub const COMPRESSED_SIGNED_RG11_EAC: GLenum = 0x9273;
pub const FRAMEBUFFER_DEFAULT_LAYERS: GLenum = 0x9312;
pub const MAX_TESS_CONTROL_IMAGE_UNIFORMS: GLenum = 0x90CB;
pub const RED: GLenum = 0x1903;
pub const STENCIL_ATTACHMENT: GLenum = 0x8D20;
pub const AND: GLenum = 0x1501;
pub const STENCIL_TEST: GLenum = 0x0B90;
pub const TESS_CONTROL_TEXTURE: GLenum = 0x829C;
pub const CONSTANT_COLOR: GLenum = 0x8001;
pub const POLYGON_MODE: GLenum = 0x0B40;
pub const TEXTURE30: GLenum = 0x84DE;
pub const DEPTH_CLAMP: GLenum = 0x864F;
pub const COLOR_ATTACHMENT16: GLenum = 0x8CF0;
pub const DOUBLE_MAT3: GLenum = 0x8F47;
pub const MAX_COMBINED_UNIFORM_BLOCKS: GLenum = 0x8A2E;
pub const POLYGON_OFFSET_FILL: GLenum = 0x8037;
pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x8A34;
pub const BOOL_VEC2: GLenum = 0x8B57;
pub const READ_PIXELS_FORMAT: GLenum = 0x828D;
pub const DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826D;
pub const TIMEOUT_EXPIRED: GLenum = 0x911B;
pub const TEXTURE_CUBE_MAP: GLenum = 0x8513;
pub const NUM_COMPATIBLE_SUBROUTINES: GLenum = 0x8E4A;
pub const DOUBLE_VEC3: GLenum = 0x8FFD;
pub const STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 0x8803;
pub const RGBA32F: GLenum = 0x8814;
pub const MAX_ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92D8;
pub const FLOAT: GLenum = 0x1406;
pub const MIN_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5B;
pub const UNSIGNED_INT_SAMPLER_BUFFER: GLenum = 0x8DD8;
pub const ACTIVE_VARIABLES: GLenum = 0x9305;
pub const RENDERBUFFER_DEPTH_SIZE: GLenum = 0x8D54;
pub const COPY_WRITE_BUFFER_BINDING: GLenum = 0x8F37;
pub const TRIANGLE_STRIP_ADJACENCY: GLenum = 0x000D;
pub const NUM_PROGRAM_BINARY_FORMATS: GLenum = 0x87FE;
pub const REPEAT: GLenum = 0x2901;
pub const STENCIL_CLEAR_VALUE: GLenum = 0x0B91;
pub const GEOMETRY_SHADER_BIT: GLenum = 0x00000004;
pub const IMAGE_CLASS_1_X_16: GLenum = 0x82BE;
pub const INT_IMAGE_2D_ARRAY: GLenum = 0x905E;
pub const MAX_SAMPLE_MASK_WORDS: GLenum = 0x8E59;
pub const TEXTURE_BORDER_COLOR: GLenum = 0x1004;
pub const MAX_WIDTH: GLenum = 0x827E;
pub const RGB32UI: GLenum = 0x8D71;
pub const CULL_FACE_MODE: GLenum = 0x0B45;
pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: GLenum = 0x92C6;
pub const VERTEX_ATTRIB_BINDING: GLenum = 0x82D4;
pub const IMAGE_2D_RECT: GLenum = 0x904F;
pub const COMPRESSED_RGB: GLenum = 0x84ED;
pub const STENCIL_BACK_FAIL: GLenum = 0x8801;
pub const UNIFORM_ARRAY_STRIDE: GLenum = 0x8A3C;
pub const MAX_VERTEX_ATOMIC_COUNTERS: GLenum = 0x92D2;
pub const COMPRESSED_RGBA8_ETC2_EAC: GLenum = 0x9278;
pub const SAMPLER_2D_RECT_SHADOW: GLenum = 0x8B64;
pub const TEXTURE_BINDING_3D: GLenum = 0x806A;
pub const PACK_SWAP_BYTES: GLenum = 0x0D00;
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = 0x8C8B;
pub const BGR_INTEGER: GLenum = 0x8D9A;
pub const TIME_ELAPSED: GLenum = 0x88BF;
pub const STENCIL_BACK_VALUE_MASK: GLenum = 0x8CA4;
pub const COLOR_ATTACHMENT9: GLenum = 0x8CE9;
pub const ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92C3;
pub const SAMPLER_2D_MULTISAMPLE: GLenum = 0x9108;
pub const MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 0x851C;
pub const TIMEOUT_IGNORED: GLuint64 = 0xFFFFFFFFFFFFFFFF;
pub const IMAGE_CUBE_MAP_ARRAY: GLenum = 0x9054;
pub const TEXTURE_MAX_LEVEL: GLenum = 0x813D;
pub const MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: GLenum = 0x90D9;
pub const MAX_VERTEX_ATTRIB_STRIDE: GLenum = 0x82E5;
pub const MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CD;
pub const SAMPLER_2D: GLenum = 0x8B5E;
pub const DOUBLE_MAT3x4: GLenum = 0x8F4C;
pub const INTERNALFORMAT_BLUE_TYPE: GLenum = 0x827A;
pub const MAX_TEXTURE_IMAGE_UNITS: GLenum = 0x8872;
pub const DOUBLE_MAT2x3: GLenum = 0x8F49;
pub const IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 0x8B9A;
pub const INVERT: GLenum = 0x150A;
pub const ATOMIC_COUNTER_BUFFER_DATA_SIZE: GLenum = 0x92C4;
pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = 0x8216;
pub const TEXTURE_DEPTH: GLenum = 0x8071;
pub const READ_ONLY: GLenum = 0x88B8;
pub const RGB16: GLenum = 0x8054;
pub const SAMPLE_BUFFERS: GLenum = 0x80A8;
pub const INT_IMAGE_1D: GLenum = 0x9057;
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90ED;
pub const MAX_TEXTURE_LOD_BIAS: GLenum = 0x84FD;
pub const DRAW_BUFFER8: GLenum = 0x882D;
pub const TEXTURE_BINDING_2D_MULTISAMPLE: GLenum = 0x9104;
pub const NUM_EXTENSIONS: GLenum = 0x821D;
pub const INTERNALFORMAT_SHARED_SIZE: GLenum = 0x8277;
pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = 0x8212;
pub const ACTIVE_UNIFORM_BLOCKS: GLenum = 0x8A36;
pub const TEXTURE_ALPHA_SIZE: GLenum = 0x805F;
pub const FRONT_RIGHT: GLenum = 0x0401;
pub const UNSIGNED_SHORT: GLenum = 0x1403;
pub const SAMPLER_2D_SHADOW: GLenum = 0x8B62;
pub const GEOMETRY_SHADER: GLenum = 0x8DD9;
pub const TEXTURE_DEPTH_TYPE: GLenum = 0x8C16;
pub const DEBUG_OUTPUT_SYNCHRONOUS: GLenum = 0x8242;
pub const FRAGMENT_INTERPOLATION_OFFSET_BITS: GLenum = 0x8E5D;
pub const R16UI: GLenum = 0x8234;
pub const PROXY_TEXTURE_1D_ARRAY: GLenum = 0x8C19;
pub const TEXTURE_BUFFER: GLenum = 0x8C2A;
pub const READ_BUFFER: GLenum = 0x0C02;
pub const UNSIGNED_INT_SAMPLER_1D: GLenum = 0x8DD1;
pub const UNIFORM_BLOCK: GLenum = 0x92E2;
pub const RG16F: GLenum = 0x822F;
pub const SRGB8_ALPHA8: GLenum = 0x8C43;
pub const STACK_UNDERFLOW: GLenum = 0x0504;
pub const TEXTURE28: GLenum = 0x84DC;
pub const MAX_TESS_CONTROL_INPUT_COMPONENTS: GLenum = 0x886C;
pub const MAX_UNIFORM_LOCATIONS: GLenum = 0x826E;
pub const CONTEXT_RELEASE_BEHAVIOR_FLUSH: GLenum = 0x82FC;
pub const COMPRESSED_RG_RGTC2: GLenum = 0x8DBD;
pub const SHADER_IMAGE_STORE: GLenum = 0x82A5;
pub const FRAMEBUFFER_BLEND: GLenum = 0x828B;
pub const DECR_WRAP: GLenum = 0x8508;
pub const DRAW_FRAMEBUFFER: GLenum = 0x8CA9;
pub const VALIDATE_STATUS: GLenum = 0x8B83;
pub const DEBUG_TYPE_ERROR: GLenum = 0x824C;
pub const OR: GLenum = 0x1507;
pub const ONE_MINUS_CONSTANT_COLOR: GLenum = 0x8002;
pub const VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = 0x88FE;
pub const MAX_VERTEX_ATTRIB_BINDINGS: GLenum = 0x82DA;
pub const TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 0x8516;
pub const MAX_VERTEX_SHADER_STORAGE_BLOCKS: GLenum = 0x90D6;
pub const SAMPLE_ALPHA_TO_ONE: GLenum = 0x809F;
pub const INT_SAMPLER_2D_ARRAY: GLenum = 0x8DCF;
pub const FLOAT_MAT2x3: GLenum = 0x8B65;
pub const TRANSFORM_FEEDBACK_VARYING: GLenum = 0x92F4;
pub const MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8B4A;
pub const INTERNALFORMAT_STENCIL_TYPE: GLenum = 0x827D;
pub const VENDOR: GLenum = 0x1F00;
pub const RIGHT: GLenum = 0x0407;
pub const BLUE: GLenum = 0x1905;
pub const TESS_CONTROL_SHADER: GLenum = 0x8E88;
pub const COMPRESSED_SRGB8_ETC2: GLenum = 0x9275;
pub const PACK_SKIP_IMAGES: GLenum = 0x806B;
pub const TEXTURE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x919F;
pub const SET: GLenum = 0x150F;
pub const TEXTURE_GREEN_SIZE: GLenum = 0x805D;
pub const PROXY_TEXTURE_CUBE_MAP: GLenum = 0x851B;
pub const UNPACK_COMPRESSED_BLOCK_DEPTH: GLenum = 0x9129;
pub const FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5;
pub const COLOR_ATTACHMENT30: GLenum = 0x8CFE;
pub const MAX_DUAL_SOURCE_DRAW_BUFFERS: GLenum = 0x88FC;
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 0x8CD0;
pub const FUNC_ADD: GLenum = 0x8006;
pub const NEAREST_MIPMAP_NEAREST: GLenum = 0x2700;
pub const MAX_GEOMETRY_IMAGE_UNIFORMS: GLenum = 0x90CD;
pub const RGBA16_SNORM: GLenum = 0x8F9B;
pub const UNSIGNED_INT_IMAGE_2D_ARRAY: GLenum = 0x9069;
pub const GEOMETRY_OUTPUT_TYPE: GLenum = 0x8918;
pub const BOOL_VEC4: GLenum = 0x8B59;
pub const COLOR_ATTACHMENT8: GLenum = 0x8CE8;
pub const COLOR_ATTACHMENT29: GLenum = 0x8CFD;
pub const POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
pub const COMPUTE_SHADER_BIT: GLenum = 0x00000020;
pub const QUERY_RESULT: GLenum = 0x8866;
pub const COLOR_ATTACHMENT7: GLenum = 0x8CE7;
pub const IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 0x8B9B;
pub const DEBUG_LOGGED_MESSAGES: GLenum = 0x9145;
pub const DOUBLE_MAT2: GLenum = 0x8F46;
pub const RGBA16I: GLenum = 0x8D88;
pub const TEXTURE20: GLenum = 0x84D4;
pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: GLenum = 0x82AC;
pub const INTERNALFORMAT_PREFERRED: GLenum = 0x8270;
pub const FUNC_SUBTRACT: GLenum = 0x800A;
pub const IMAGE_CLASS_10_10_10_2: GLenum = 0x82C3;
pub const DYNAMIC_COPY: GLenum = 0x88EA;
pub const STENCIL_RENDERABLE: GLenum = 0x8288;
pub const TEXTURE_BUFFER_BINDING: GLenum = 0x8C2A;
pub const IMAGE_FORMAT_COMPATIBILITY_TYPE: GLenum = 0x90C7;
pub const FILTER: GLenum = 0x829A;
pub const COLOR_WRITEMASK: GLenum = 0x0C23;
pub const COLOR: GLenum = 0x1800;
pub const TEXTURE_SWIZZLE_G: GLenum = 0x8E43;
pub const SAMPLER_1D_ARRAY_SHADOW: GLenum = 0x8DC3;
pub const IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9056;
pub const XOR: GLenum = 0x1506;
pub const VIEW_CLASS_RGTC1_RED: GLenum = 0x82D0;
pub const NUM_SHADING_LANGUAGE_VERSIONS: GLenum = 0x82E9;
pub const COLOR_ATTACHMENT22: GLenum = 0x8CF6;
pub const NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A2;
pub const PRIMITIVE_RESTART_INDEX: GLenum = 0x8F9E;
pub const RGB32F: GLenum = 0x8815;
pub const MAP_INVALIDATE_BUFFER_BIT: GLenum = 0x0008;
pub const SCISSOR_TEST: GLenum = 0x0C11;
pub const PACK_IMAGE_HEIGHT: GLenum = 0x806C;
pub const ZERO: GLenum = 0;
pub const BLEND_EQUATION_ALPHA: GLenum = 0x883D;
pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: GLenum = 0x8CDB;
pub const UNIFORM_SIZE: GLenum = 0x8A38;
pub const CLIP_DEPTH_MODE: GLenum = 0x935D;
pub const FLOAT_VEC2: GLenum = 0x8B50;
pub const SRGB_ALPHA: GLenum = 0x8C42;
pub const UNSIGNED_SHORT_4_4_4_4_REV: GLenum = 0x8365;
pub const WRITE_ONLY: GLenum = 0x88B9;
pub const TEXTURE_CUBE_MAP_SEAMLESS: GLenum = 0x884F;
pub const UNPACK_ROW_LENGTH: GLenum = 0x0CF2;
pub const MAX_TEXTURE_SIZE: GLenum = 0x0D33;
pub const SAMPLE_POSITION: GLenum = 0x8E50;
pub const VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 0x8645;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 0x851A;
pub const RGB16I: GLenum = 0x8D89;
pub const TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x9009;
pub const RGBA: GLenum = 0x1908;
pub const DOUBLE: GLenum = 0x140A;
pub const REPLACE: GLenum = 0x1E01;
pub const LOGIC_OP_MODE: GLenum = 0x0BF0;
pub const TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 0x8515;
pub const UNIFORM_OFFSET: GLenum = 0x8A3B;
pub const ACTIVE_ATTRIBUTES: GLenum = 0x8B89;
pub const COMPARE_REF_TO_TEXTURE: GLenum = 0x884E;
pub const BUFFER_IMMUTABLE_STORAGE: GLenum = 0x821F;
pub const SHADER: GLenum = 0x82E1;
pub const TEXTURE9: GLenum = 0x84C9;
pub const ANY_SAMPLES_PASSED_CONSERVATIVE: GLenum = 0x8D6A;
pub const BUFFER_ACCESS_FLAGS: GLenum = 0x911F;
pub const LOWER_LEFT: GLenum = 0x8CA1;
pub const PROVOKING_VERTEX: GLenum = 0x8E4F;
pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 0x8CD6;
pub const MIN_MAP_BUFFER_ALIGNMENT: GLenum = 0x90BC;
pub const VERTEX_BINDING_STRIDE: GLenum = 0x82D8;
pub const UNIFORM_BUFFER_SIZE: GLenum = 0x8A2A;
pub const DEPTH_RENDERABLE: GLenum = 0x8287;
pub const MAX_IMAGE_SAMPLES: GLenum = 0x906D;
pub const BUFFER_DATA_SIZE: GLenum = 0x9303;
pub const COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: GLenum = 0x8E8F;
pub const GEOMETRY_VERTICES_OUT: GLenum = 0x8916;
pub const SYNC_GPU_COMMANDS_COMPLETE: GLenum = 0x9117;
pub const TEXTURE21: GLenum = 0x84D5;
pub const SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x90DF;
pub const DEPTH24_STENCIL8: GLenum = 0x88F0;
pub const COLOR_ATTACHMENT11: GLenum = 0x8CEB;
pub const UNSIGNED_INT_10_10_10_2: GLenum = 0x8036;
pub const BOOL: GLenum = 0x8B56;
pub const FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = 0x8B8B;
pub const DEPTH32F_STENCIL8: GLenum = 0x8CAD;
pub const NO_RESET_NOTIFICATION: GLenum = 0x8261;
pub const R16F: GLenum = 0x822D;
pub const TRIANGLE_FAN: GLenum = 0x0006;
pub const NONE: GLenum = 0;
pub const ALREADY_SIGNALED: GLenum = 0x911A;
pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: GLenum = 0x82AD;
pub const POLYGON_OFFSET_LINE: GLenum = 0x2A02;
pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = 0x8213;
pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4C;
pub const TEXTURE_2D_ARRAY: GLenum = 0x8C1A;
pub const ONE_MINUS_SRC1_COLOR: GLenum = 0x88FA;
pub const ONE: GLenum = 1;
pub const STACK_OVERFLOW: GLenum = 0x0503;
pub const IMAGE_BINDING_LEVEL: GLenum = 0x8F3B;
pub const DRAW_BUFFER4: GLenum = 0x8829;
pub const R3_G3_B2: GLenum = 0x2A10;
pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = 0x8217;
pub const MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D9;
pub const DEBUG_TYPE_MARKER: GLenum = 0x8268;
pub const CONTEXT_CORE_PROFILE_BIT: GLenum = 0x00000001;
pub const MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CC;
pub const VIEW_COMPATIBILITY_CLASS: GLenum = 0x82B6;
pub const SYNC_FENCE: GLenum = 0x9116;
pub const STENCIL_INDEX4: GLenum = 0x8D47;
pub const TRIANGLES: GLenum = 0x0004;
pub const MAX_CLIP_DISTANCES: GLenum = 0x0D32;
pub const VIEWPORT_BOUNDS_RANGE: GLenum = 0x825D;
pub const TEXTURE14: GLenum = 0x84CE;
pub const FRAMEBUFFER_DEFAULT: GLenum = 0x8218;
pub const MAX_COMBINED_SHADER_STORAGE_BLOCKS: GLenum = 0x90DC;
pub const MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E1E;
pub const STENCIL_INDEX16: GLenum = 0x8D49;
pub const SAMPLER_1D_ARRAY: GLenum = 0x8DC0;
pub const ACTIVE_SUBROUTINES: GLenum = 0x8DE5;
pub const MAX_COLOR_TEXTURE_SAMPLES: GLenum = 0x910E;
pub const SRC1_ALPHA: GLenum = 0x8589;
pub const FIRST_VERTEX_CONVENTION: GLenum = 0x8E4D;
pub const POINTS: GLenum = 0x0000;
pub const SRGB_READ: GLenum = 0x8297;
pub const PIXEL_UNPACK_BUFFER_BINDING: GLenum = 0x88EF;
pub const SAMPLE_COVERAGE_VALUE: GLenum = 0x80AA;
pub const INTERNALFORMAT_RED_SIZE: GLenum = 0x8271;
pub const TEXTURE19: GLenum = 0x84D3;
pub const PROGRAM_PIPELINE_BINDING: GLenum = 0x825A;
pub const FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 0x8DAD;
pub const ATOMIC_COUNTER_BUFFER_BINDING: GLenum = 0x92C1;
pub const COLOR_ATTACHMENT31: GLenum = 0x8CFF;
pub const UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x8A45;
pub const MAX_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8DDF;
pub const MAX_NUM_COMPATIBLE_SUBROUTINES: GLenum = 0x92F8;
pub const STATIC_DRAW: GLenum = 0x88E4;
pub const INTERNALFORMAT_ALPHA_TYPE: GLenum = 0x827B;
pub const STENCIL_VALUE_MASK: GLenum = 0x0B93;
pub const INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9061;
pub const VIEW_CLASS_24_BITS: GLenum = 0x82C9;
pub const ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = 0x8A43;
pub const POINT_SIZE: GLenum = 0x0B11;
pub const ONE_MINUS_CONSTANT_ALPHA: GLenum = 0x8004;
pub const PACK_LSB_FIRST: GLenum = 0x0D01;
pub const STENCIL_BACK_FUNC: GLenum = 0x8800;
pub const DRAW_BUFFER0: GLenum = 0x8825;
pub const TOP_LEVEL_ARRAY_SIZE: GLenum = 0x930C;
pub const LINE_SMOOTH_HINT: GLenum = 0x0C52;
pub const QUERY_BY_REGION_WAIT_INVERTED: GLenum = 0x8E19;
pub const INTERNALFORMAT_STENCIL_SIZE: GLenum = 0x8276;
pub const COLOR_ATTACHMENT4: GLenum = 0x8CE4;
pub const BUFFER: GLenum = 0x82E0;
pub const RENDERBUFFER_BINDING: GLenum = 0x8CA7;
pub const TEXTURE3: GLenum = 0x84C3;
pub const VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 0x8625;
pub const STEREO: GLenum = 0x0C33;
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x92CA;
pub const NOOP: GLenum = 0x1505;
pub const UNSIGNED_INT_2_10_10_10_REV: GLenum = 0x8368;
pub const ELEMENT_ARRAY_BARRIER_BIT: GLenum = 0x00000002;
pub const LAST_VERTEX_CONVENTION: GLenum = 0x8E4E;
pub const SAMPLE_COVERAGE: GLenum = 0x80A0;
pub const DRAW_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
pub const MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: GLenum = 0x90D7;
pub const SHADER_IMAGE_ATOMIC: GLenum = 0x82A6;
pub const PACK_ALIGNMENT: GLenum = 0x0D05;
pub const DEPTH_BUFFER_BIT: GLenum = 0x00000100;
pub const CONTEXT_LOST: GLenum = 0x0507;
pub const VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = 0x88FD;
pub const RENDERBUFFER_BLUE_SIZE: GLenum = 0x8D52;
pub const MAX_ELEMENTS_VERTICES: GLenum = 0x80E8;
pub const UNIFORM_IS_ROW_MAJOR: GLenum = 0x8A3E;
pub const DISPLAY_LIST: GLenum = 0x82E7;
pub const TEXTURE18: GLenum = 0x84D2;
pub const NOR: GLenum = 0x1508;
pub const INFO_LOG_LENGTH: GLenum = 0x8B84;
pub const CURRENT_PROGRAM: GLenum = 0x8B8D;
pub const MAX_DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826C;
pub const MAX_SHADER_STORAGE_BUFFER_BINDINGS: GLenum = 0x90DD;
pub const LINE_WIDTH: GLenum = 0x0B21;
pub const TEXTURE8: GLenum = 0x84C8;
pub const UNSIGNED_INT_VEC3: GLenum = 0x8DC7;
pub const TEXTURE_SHARED_SIZE: GLenum = 0x8C3F;
pub const TEXTURE_ALPHA_TYPE: GLenum = 0x8C13;
pub const CONTEXT_PROFILE_MASK: GLenum = 0x9126;
pub const MAX_TRANSFORM_FEEDBACK_BUFFERS: GLenum = 0x8E70;
pub const MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: GLenum = 0x8E86;
pub const MIN_PROGRAM_TEXEL_OFFSET: GLenum = 0x8904;
pub const MEDIUM_FLOAT: GLenum = 0x8DF1;
pub const RGBA16UI: GLenum = 0x8D76;
pub const MAX_FRAMEBUFFER_LAYERS: GLenum = 0x9317;
pub const FULL_SUPPORT: GLenum = 0x82B7;
pub const CLIP_DISTANCE5: GLenum = 0x3005;
pub const COMPUTE_SUBROUTINE: GLenum = 0x92ED;
pub const COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: GLenum = 0x9279;
pub const COLOR_ATTACHMENT17: GLenum = 0x8CF1;
pub const SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910B;
pub const TEXTURE_COMPRESSED_BLOCK_SIZE: GLenum = 0x82B3;
pub const TEXTURE_STENCIL_SIZE: GLenum = 0x88F1;
pub const INVALID_VALUE: GLenum = 0x0501;
pub const MAX_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5C;
pub const MIRRORED_REPEAT: GLenum = 0x8370;
pub const FLOAT_VEC3: GLenum = 0x8B51;
pub const ALPHA: GLenum = 0x1906;
pub const GEOMETRY_TEXTURE: GLenum = 0x829E;
pub const POINT_SIZE_RANGE: GLenum = 0x0B12;
pub const ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 0x8E47;
pub const TRANSFORM_FEEDBACK_BUFFER_STRIDE: GLenum = 0x934C;
pub const RG16I: GLenum = 0x8239;
pub const PROXY_TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x900B;
pub const QUERY_BUFFER_BINDING: GLenum = 0x9193;
pub const DRAW_BUFFER1: GLenum = 0x8826;
pub const DRAW_BUFFER11: GLenum = 0x8830;
pub const NUM_ACTIVE_VARIABLES: GLenum = 0x9304;
pub const RGB5_A1: GLenum = 0x8057;
pub const ALWAYS: GLenum = 0x0207;
pub const INT: GLenum = 0x1404;
pub const VERTEX_PROGRAM_POINT_SIZE: GLenum = 0x8642;
pub const DEPTH_ATTACHMENT: GLenum = 0x8D00;
pub const INTERNALFORMAT_GREEN_TYPE: GLenum = 0x8279;
pub const FRAMEBUFFER_ATTACHMENT_LAYERED: GLenum = 0x8DA7;
pub const PATCH_DEFAULT_OUTER_LEVEL: GLenum = 0x8E74;
pub const INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900E;
pub const AUTO_GENERATE_MIPMAP: GLenum = 0x8295;
pub const SIGNALED: GLenum = 0x9119;
pub const SHADING_LANGUAGE_VERSION: GLenum = 0x8B8C;
pub const TRANSFORM_FEEDBACK_BUFFER_START: GLenum = 0x8C84;
pub const GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x887F;
pub const MAX_ARRAY_TEXTURE_LAYERS: GLenum = 0x88FF;
pub const ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
pub const SAMPLER_3D: GLenum = 0x8B5F;
pub const RGBA32UI: GLenum = 0x8D70;
pub const CLIP_ORIGIN: GLenum = 0x935C;
pub const ATOMIC_COUNTER_BUFFER_START: GLenum = 0x92C2;
pub const COMPRESSED_RG: GLenum = 0x8226;
pub const DRAW_INDIRECT_BUFFER_BINDING: GLenum = 0x8F43;
pub const DEBUG_CALLBACK_USER_PARAM: GLenum = 0x8245;
pub const MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CF;
pub const DEPTH_COMPONENT24: GLenum = 0x81A6;
pub const TEXTURE_BUFFER_SIZE: GLenum = 0x919E;
pub const COLOR_ATTACHMENT13: GLenum = 0x8CED;
pub const LESS: GLenum = 0x0201;
pub const RG_INTEGER: GLenum = 0x8228;
pub const MAX_COMPUTE_WORK_GROUP_COUNT: GLenum = 0x91BE;
pub const RGBA12: GLenum = 0x805A;
pub const ONE_MINUS_SRC1_ALPHA: GLenum = 0x88FB;
pub const VIEW_CLASS_RGTC2_RG: GLenum = 0x82D1;
pub const RENDERBUFFER_STENCIL_SIZE: GLenum = 0x8D55;
pub const SAMPLER_BUFFER: GLenum = 0x8DC2;
pub const TRANSFORM_FEEDBACK_BUFFER_PAUSED: GLenum = 0x8E23;
pub const INT_IMAGE_2D_RECT: GLenum = 0x905A;
pub const IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: GLenum = 0x90C9;
pub const TEXTURE_2D: GLenum = 0x0DE1;
pub const TEXTURE13: GLenum = 0x84CD;
pub const UNSIGNED_INT_SAMPLER_2D_RECT: GLenum = 0x8DD5;
pub const FIXED_ONLY: GLenum = 0x891D;
pub const TEXTURE16: GLenum = 0x84D0;
pub const GREATER: GLenum = 0x0204;
pub const RGB8UI: GLenum = 0x8D7D;
pub const MAX_COMPUTE_ATOMIC_COUNTERS: GLenum = 0x8265;
pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4D;
pub const CLEAR: GLenum = 0x1500;
pub const R11F_G11F_B10F: GLenum = 0x8C3A;
pub const DEPTH_COMPONENT32: GLenum = 0x81A7;
pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = 0x8C8A;
pub const MAP_FLUSH_EXPLICIT_BIT: GLenum = 0x0010;
pub const TEXTURE23: GLenum = 0x84D7;
pub const MAX_TESS_GEN_LEVEL: GLenum = 0x8E7E;
pub const R16_SNORM: GLenum = 0x8F98;
pub const VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 0x8624;
pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8A31;
pub const DEBUG_SOURCE_THIRD_PARTY: GLenum = 0x8249;
pub const FRACTIONAL_EVEN: GLenum = 0x8E7C;
pub const RENDERBUFFER_WIDTH: GLenum = 0x8D42;
pub const SEPARATE_ATTRIBS: GLenum = 0x8C8D;
pub const INT_IMAGE_1D_ARRAY: GLenum = 0x905D;
pub const MAX_COMBINED_CLIP_AND_CULL_DISTANCES: GLenum = 0x82FA;
pub const IMAGE_2D_MULTISAMPLE: GLenum = 0x9055;
pub const TRANSFORM_FEEDBACK_BUFFER_ACTIVE: GLenum = 0x8E24;
pub const COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A3;
pub const SAMPLER_CUBE_SHADOW: GLenum = 0x8DC5;
pub const COMPRESSED_RED_RGTC1: GLenum = 0x8DBB;
pub const ACTIVE_UNIFORM_MAX_LENGTH: GLenum = 0x8B87;
pub const TEXTURE_BUFFER_DATA_STORE_BINDING: GLenum = 0x8C2D;
pub const INT_SAMPLER_3D: GLenum = 0x8DCB;
pub const ALIASED_LINE_WIDTH_RANGE: GLenum = 0x846E;
pub const IMAGE_PIXEL_TYPE: GLenum = 0x82AA;
pub const NAND: GLenum = 0x150E;
pub const AND_REVERSE: GLenum = 0x1502;
pub const TEXTURE_COMPRESSION_HINT: GLenum = 0x84EF;
pub const COLOR_ATTACHMENT25: GLenum = 0x8CF9;
pub const TEXTURE_COMPARE_FUNC: GLenum = 0x884D;
pub const TEXTURE_SWIZZLE_RGBA: GLenum = 0x8E46;
pub const COLOR_ATTACHMENT1: GLenum = 0x8CE1;
pub const MAX_PROGRAM_TEXEL_OFFSET: GLenum = 0x8905;
pub const CLIP_DISTANCE4: GLenum = 0x3004;
pub const REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x9309;
pub const VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 0x8622;
pub const DEPTH_FUNC: GLenum = 0x0B74;
pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8DE1;
pub const NEVER: GLenum = 0x0200;
pub const UNIFORM_MATRIX_STRIDE: GLenum = 0x8A3D;
pub const UNSIGNED_INT_IMAGE_CUBE: GLenum = 0x9066;
pub const RG32I: GLenum = 0x823B;
pub const VERTEX_SHADER_BIT: GLenum = 0x00000001;
pub const IMAGE_CLASS_2_X_32: GLenum = 0x82BA;
pub const RG16UI: GLenum = 0x823A;
pub const LINK_STATUS: GLenum = 0x8B82;
pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLenum = 0x8DA8;
pub const INT_SAMPLER_2D: GLenum = 0x8DCA;
pub const MAX_TESS_EVALUATION_INPUT_COMPONENTS: GLenum = 0x886D;
pub const TEXTURE_GREEN_TYPE: GLenum = 0x8C11;
pub const TEXTURE_VIEW_NUM_LAYERS: GLenum = 0x82DE;
pub const MAP_WRITE_BIT: GLenum = 0x0002;
pub const TEXTURE7: GLenum = 0x84C7;
pub const TEXTURE_BLUE_TYPE: GLenum = 0x8C12;
pub const GEOMETRY_SUBROUTINE: GLenum = 0x92EB;
pub const UNSIGNED_INT_SAMPLER_1D_ARRAY: GLenum = 0x8DD6;
pub const STENCIL_BACK_REF: GLenum = 0x8CA3;
pub const LAYER_PROVOKING_VERTEX: GLenum = 0x825E;
pub const DEPTH_STENCIL_TEXTURE_MODE: GLenum = 0x90EA;
pub const FRONT_FACE: GLenum = 0x0B46;
pub const FRAGMENT_SHADER_BIT: GLenum = 0x00000002;
pub const RGBA8UI: GLenum = 0x8D7C;
pub const LINE_STRIP_ADJACENCY: GLenum = 0x000B;
pub const FRAMEBUFFER_SRGB: GLenum = 0x8DB9;
pub const TRIANGLE_STRIP: GLenum = 0x0005;
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x92CB;
pub const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GLenum = 0x9105;
pub const MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: GLenum = 0x8E81;
pub const TRANSFORM_FEEDBACK_PAUSED: GLenum = 0x8E23;
pub const DEBUG_OUTPUT: GLenum = 0x92E0;
pub const MAX_LAYERS: GLenum = 0x8281;
pub const GREEN_INTEGER: GLenum = 0x8D95;
pub const CLIP_DISTANCE0: GLenum = 0x3000;
pub const BUFFER_MAPPED: GLenum = 0x88BC;
pub const VERTEX_SUBROUTINE: GLenum = 0x92E8;
pub const UNIFORM_BLOCK_DATA_SIZE: GLenum = 0x8A40;
pub const UNPACK_ALIGNMENT: GLenum = 0x0CF5;
pub const TYPE: GLenum = 0x92FA;
pub const REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x9308;
pub const QUERY: GLenum = 0x82E3;
pub const MAX_DEPTH: GLenum = 0x8280;
pub const UNSIGNED_BYTE_2_3_3_REV: GLenum = 0x8362;
pub const POINT_SPRITE_COORD_ORIGIN: GLenum = 0x8CA0;
pub const UNSIGNED_INT_IMAGE_2D: GLenum = 0x9063;
pub const SYNC_FLUSH_COMMANDS_BIT: GLenum = 0x00000001;
pub const COMPRESSED_SIGNED_RG_RGTC2: GLenum = 0x8DBE;
pub const SAMPLER_2D_ARRAY_SHADOW: GLenum = 0x8DC4;
pub const LINE_STRIP: GLenum = 0x0003;
pub const LOW_FLOAT: GLenum = 0x8DF0;
pub const PACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x912C;
pub const TEXTURE_BINDING_1D_ARRAY: GLenum = 0x8C1C;
pub const UPPER_LEFT: GLenum = 0x8CA2;
pub const MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: GLenum = 0x8E82;
pub const UNPACK_COMPRESSED_BLOCK_SIZE: GLenum = 0x912A;
pub const POLYGON_SMOOTH: GLenum = 0x0B41;
pub const MAX_NAME_LENGTH: GLenum = 0x92F6;
pub const NEAREST_MIPMAP_LINEAR: GLenum = 0x2702;
pub const TESS_EVALUATION_SHADER_BIT: GLenum = 0x00000010;
pub const MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 0x9122;
pub const RGB8: GLenum = 0x8051;
pub const MAX_VERTEX_ATTRIBS: GLenum = 0x8869;
pub const NOTEQUAL: GLenum = 0x0205;
pub const RASTERIZER_DISCARD: GLenum = 0x8C89;
pub const MANUAL_GENERATE_MIPMAP: GLenum = 0x8294;
pub const TEXTURE_SWIZZLE_B: GLenum = 0x8E44;
pub const MAX_SHADER_STORAGE_BLOCK_SIZE: GLenum = 0x90DE;
pub const UNSIGNED_INT_8_8_8_8: GLenum = 0x8035;
pub const GEQUAL: GLenum = 0x0206;
pub const INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x905F;
pub const MAX_VERTEX_IMAGE_UNIFORMS: GLenum = 0x90CA;
pub const UNIFORM_NAME_LENGTH: GLenum = 0x8A39;
pub const FRACTIONAL_ODD: GLenum = 0x8E7B;
pub const INT_VEC2: GLenum = 0x8B53;
pub const UNSIGNED_INT_IMAGE_2D_RECT: GLenum = 0x9065;
pub const LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR: GLenum = 0x824E;
pub const UNSIGNALED: GLenum = 0x9118;
pub const STENCIL_INDEX8: GLenum = 0x8D48;
pub const RENDERBUFFER_GREEN_SIZE: GLenum = 0x8D51;
pub const IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: GLenum = 0x90C8;
pub const PROXY_TEXTURE_2D_ARRAY: GLenum = 0x8C1B;
pub const TEXTURE_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x82B2;
pub const DEPTH_WRITEMASK: GLenum = 0x0B72;
pub const AND_INVERTED: GLenum = 0x1504;
pub const UNSIGNED_INT_SAMPLER_3D: GLenum = 0x8DD3;
pub const IMAGE_BINDING_NAME: GLenum = 0x8F3A;
pub const MAX_COMPUTE_UNIFORM_BLOCKS: GLenum = 0x91BB;
pub const FLOAT_MAT3x2: GLenum = 0x8B67;
pub const POLYGON_OFFSET_FACTOR: GLenum = 0x8038;
pub const TEXTURE1: GLenum = 0x84C1;
pub const IMAGE_CLASS_4_X_8: GLenum = 0x82BF;
pub const ACTIVE_SUBROUTINE_UNIFORMS: GLenum = 0x8DE6;
pub const FRAGMENT_TEXTURE: GLenum = 0x829F;
pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x8A46;
pub const FRAMEBUFFER_DEFAULT_WIDTH: GLenum = 0x9310;
pub const DELETE_STATUS: GLenum = 0x8B80;
pub const UNSIGNED_INT_IMAGE_1D: GLenum = 0x9062;
pub const UNIFORM_BARRIER_BIT: GLenum = 0x00000004;
pub const CLIP_DISTANCE2: GLenum = 0x3002;
pub const UNSIGNED_INT_8_8_8_8_REV: GLenum = 0x8367;
pub const DRAW_BUFFER10: GLenum = 0x882F;
pub const TOP_LEVEL_ARRAY_STRIDE: GLenum = 0x930D;
pub const VIEWPORT: GLenum = 0x0BA2;
pub const COMPRESSED_RGB_BPTC_SIGNED_FLOAT: GLenum = 0x8E8E;
pub const DRAW_BUFFER6: GLenum = 0x882B;
pub const INVALID_ENUM: GLenum = 0x0500;
pub const BGR: GLenum = 0x80E0;
pub const COMPRESSED_RGBA: GLenum = 0x84EE;
pub const COLOR_COMPONENTS: GLenum = 0x8283;
pub const ALL_SHADER_BITS: GLenum = 0xFFFFFFFF;
pub const R8I: GLenum = 0x8231;
pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x906C;
pub const SYNC_FLAGS: GLenum = 0x9115;
pub const RGB8I: GLenum = 0x8D8F;
pub const PRIMITIVE_RESTART: GLenum = 0x8F9D;
pub const SHADER_STORAGE_BUFFER_START: GLenum = 0x90D4;
pub const SAMPLER_CUBE: GLenum = 0x8B60;
pub const COLOR_ATTACHMENT5: GLenum = 0x8CE5;
pub const EQUIV: GLenum = 0x1509;
pub const BLEND_SRC: GLenum = 0x0BE1;
pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER: GLenum = 0x8CDC;
pub const IMAGE_BINDING_LAYERED: GLenum = 0x8F3C;
pub const FRAMEBUFFER_RENDERABLE_LAYERED: GLenum = 0x828A;
pub const RGB10: GLenum = 0x8052;
pub const PATCH_VERTICES: GLenum = 0x8E72;
pub const COPY_READ_BUFFER_BINDING: GLenum = 0x8F36;
pub const PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: GLenum = 0x8221;
pub const BLOCK_INDEX: GLenum = 0x92FD;
pub const GREEN: GLenum = 0x1904;
pub const RGB16UI: GLenum = 0x8D77;
pub const MAX_PATCH_VERTICES: GLenum = 0x8E7D;
pub const TEXTURE_COMPRESSED_BLOCK_WIDTH: GLenum = 0x82B1;
pub const INT_SAMPLER_2D_RECT: GLenum = 0x8DCD;
pub const BGRA: GLenum = 0x80E1;
pub const INT_IMAGE_BUFFER: GLenum = 0x905C;
pub const TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 0x8519;
pub const TEXTURE_BINDING_CUBE_MAP: GLenum = 0x8514;
pub const IMAGE_CLASS_11_11_10: GLenum = 0x82C2;
pub const TEXTURE11: GLenum = 0x84CB;
pub const MAX_HEIGHT: GLenum = 0x827F;
pub const COLOR_ATTACHMENT14: GLenum = 0x8CEE;
pub const MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E1F;
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 0x8CD1;
pub const POINT: GLenum = 0x1B00;
pub const STENCIL_BUFFER_BIT: GLenum = 0x00000400;
pub const TEXTURE27: GLenum = 0x84DB;
pub const COMPUTE_SUBROUTINE_UNIFORM: GLenum = 0x92F3;
pub const MAX_TEXTURE_BUFFER_SIZE: GLenum = 0x8C2B;
pub const VIEW_CLASS_S3TC_DXT1_RGBA: GLenum = 0x82CD;
pub const STENCIL_PASS_DEPTH_FAIL: GLenum = 0x0B95;
pub const LINE: GLenum = 0x1B01;
pub const RGB_INTEGER: GLenum = 0x8D98;
pub const DISPATCH_INDIRECT_BUFFER: GLenum = 0x90EE;
pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = 0x8C85;
pub const MAX_COMPUTE_SHARED_MEMORY_SIZE: GLenum = 0x8262;
pub const LINE_WIDTH_RANGE: GLenum = 0x0B22;
pub const RGBA_INTEGER: GLenum = 0x8D99;
pub const STREAM_DRAW: GLenum = 0x88E0;
pub const INT_SAMPLER_BUFFER: GLenum = 0x8DD0;
pub const TEXTURE_INTERNAL_FORMAT: GLenum = 0x1003;
pub const COMPUTE_TEXTURE: GLenum = 0x82A0;
pub const UNSIGNED_INT_IMAGE_BUFFER: GLenum = 0x9067;
pub const UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x92DA;
pub const TRANSFORM_FEEDBACK_BUFFER_INDEX: GLenum = 0x934B;
pub const DEPTH_COMPONENT16: GLenum = 0x81A5;
pub const NO_ERROR: GLenum = 0;
pub const PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9103;
pub const TRIANGLES_ADJACENCY: GLenum = 0x000C;
pub const DEPTH_COMPONENT32F: GLenum = 0x8CAC;
pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLenum = 0x8A35;
pub const UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900F;
pub const SHADER_STORAGE_BUFFER_BINDING: GLenum = 0x90D3;
pub const NAME_LENGTH: GLenum = 0x92F9;
pub const TRANSFORM_FEEDBACK_VARYINGS: GLenum = 0x8C83;
pub const MAX_COMPUTE_IMAGE_UNIFORMS: GLenum = 0x91BD;
pub const PROGRAM_OUTPUT: GLenum = 0x92E4;
pub const TEXTURE_WRAP_R: GLenum = 0x8072;
pub const MAX_VIEWPORTS: GLenum = 0x825B;
pub const MAX_UNIFORM_BUFFER_BINDINGS: GLenum = 0x8A2F;
pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR: GLenum = 0x824D;
pub const RGBA4: GLenum = 0x8056;
pub const RGB12: GLenum = 0x8053;
pub const MAX_GEOMETRY_OUTPUT_VERTICES: GLenum = 0x8DE0;
pub const QUERY_TARGET: GLenum = 0x82EA;
pub const QUERY_BY_REGION_WAIT: GLenum = 0x8E15;
pub const MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E80;
pub const UNIFORM_BUFFER: GLenum = 0x8A11;
pub const BUFFER_MAP_OFFSET: GLenum = 0x9121;
pub const UNSIGNED_SHORT_5_5_5_1: GLenum = 0x8034;
pub const IMAGE_BINDING_ACCESS: GLenum = 0x8F3E;
pub const MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 0x9125;
pub const TEXTURE22: GLenum = 0x84D6;
pub const RG16: GLenum = 0x822C;
pub const DEBUG_SOURCE_SHADER_COMPILER: GLenum = 0x8248;
pub const SUBPIXEL_BITS: GLenum = 0x0D50;
pub const TEXTURE_WIDTH: GLenum = 0x1000;
pub const DRAW_INDIRECT_BUFFER: GLenum = 0x8F3F;
pub const UNIFORM_BLOCK_BINDING: GLenum = 0x8A3F;
pub const VIEW_CLASS_S3TC_DXT3_RGBA: GLenum = 0x82CE;
pub const MULTISAMPLE: GLenum = 0x809D;
pub const LINES: GLenum = 0x0001;
pub const ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 0x8895;
pub const POLYGON_OFFSET_UNITS: GLenum = 0x2A00;
pub const TRANSFORM_FEEDBACK_BINDING: GLenum = 0x8E25;
pub const PROGRAM: GLenum = 0x82E2;
pub const TEXTURE_IMMUTABLE_FORMAT: GLenum = 0x912F;
pub const TESS_CONTROL_SHADER_BIT: GLenum = 0x00000008;
pub const VERTEX_ATTRIB_ARRAY_LONG: GLenum = 0x874E;
pub const COLOR_ATTACHMENT27: GLenum = 0x8CFB;
pub const RGB10_A2UI: GLenum = 0x906F;
pub const STENCIL_FUNC: GLenum = 0x0B92;
pub const MAX_LABEL_LENGTH: GLenum = 0x82E8;
pub const IMAGE_CUBE: GLenum = 0x9050;
pub const BUFFER_STORAGE_FLAGS: GLenum = 0x8220;
pub const CULL_FACE: GLenum = 0x0B44;
pub const IMAGE_1D_ARRAY: GLenum = 0x9052;
pub const BUFFER_BINDING: GLenum = 0x9302;
pub const PIXEL_BUFFER_BARRIER_BIT: GLenum = 0x00000080;
pub const ARRAY_STRIDE: GLenum = 0x92FE;
pub const MAX: GLenum = 0x8008;
pub const VIEW_CLASS_BPTC_FLOAT: GLenum = 0x82D3;
pub const COPY_WRITE_BUFFER: GLenum = 0x8F37;
pub const POLYGON_SMOOTH_HINT: GLenum = 0x0C53;
pub const FRONT_LEFT: GLenum = 0x0400;
pub const CONTEXT_COMPATIBILITY_PROFILE_BIT: GLenum = 0x00000002;
pub const TEXTURE10: GLenum = 0x84CA;
pub const COMPUTE_SHADER: GLenum = 0x91B9;
pub const RGB9_E5: GLenum = 0x8C3D;
pub const MAX_GEOMETRY_UNIFORM_BLOCKS: GLenum = 0x8A2C;
pub const MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = 0x8A2D;
pub const CLIP_DISTANCE7: GLenum = 0x3007;
pub const MAX_GEOMETRY_ATOMIC_COUNTERS: GLenum = 0x92D5;
pub const ACTIVE_PROGRAM: GLenum = 0x8259;
pub const BYTE: GLenum = 0x1400;
pub const MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D1;
pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = 0x8D56;
pub const SRC_ALPHA_SATURATE: GLenum = 0x0308;
pub const ACTIVE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D9;
pub const UNIFORM_BUFFER_BINDING: GLenum = 0x8A28;
pub const MAJOR_VERSION: GLenum = 0x821B;
pub const TEXTURE26: GLenum = 0x84DA;
pub const CLAMP_READ_COLOR: GLenum = 0x891C;
pub const TEXTURE_MAX_LOD: GLenum = 0x813B;
pub const TESS_EVALUATION_SHADER: GLenum = 0x8E87;
pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x910A;
pub const QUERY_WAIT: GLenum = 0x8E13;
pub const DOUBLE_MAT4x3: GLenum = 0x8F4E;
pub const TEXTURE_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9107;
pub const STATIC_COPY: GLenum = 0x88E6;
pub const RGB: GLenum = 0x1907;
pub const PROXY_TEXTURE_2D: GLenum = 0x8064;
pub const LEFT: GLenum = 0x0406;
pub const SAMPLER_2D_RECT: GLenum = 0x8B63;
pub const QUERY_COUNTER_BITS: GLenum = 0x8864;
pub const SHORT: GLenum = 0x1402;
pub const BUFFER_MAP_LENGTH: GLenum = 0x9120;
pub const MAX_DEBUG_MESSAGE_LENGTH: GLenum = 0x9143;
pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = 0x8DD7;
pub const IS_PER_PATCH: GLenum = 0x92E7;
pub const TEXTURE_1D_ARRAY: GLenum = 0x8C18;
pub const INCR_WRAP: GLenum = 0x8507;
pub const TEXTURE_SWIZZLE_R: GLenum = 0x8E42;
pub const RG8: GLenum = 0x822B;
pub const FLOAT_MAT4x2: GLenum = 0x8B69;
pub const TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 0x8C7F;
pub const ACTIVE_TEXTURE: GLenum = 0x84E0;
pub const MAX_UNIFORM_BLOCK_SIZE: GLenum = 0x8A30;
pub const MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8266;
pub const VIEW_CLASS_64_BITS: GLenum = 0x82C6;
pub const INTERNALFORMAT_GREEN_SIZE: GLenum = 0x8272;
pub const ARRAY_BUFFER: GLenum = 0x8892;
pub const IMAGE_CLASS_2_X_8: GLenum = 0x82C0;
pub const RGB32I: GLenum = 0x8D83;
pub const MAX_COMPUTE_SHADER_STORAGE_BLOCKS: GLenum = 0x90DB;
pub const DEPTH_RANGE: GLenum = 0x0B70;
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x92C7;
pub const RGBA16F: GLenum = 0x881A;
pub const MAX_GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x8E5A;
pub const LOCATION_COMPONENT: GLenum = 0x934A;
pub const UNIFORM: GLenum = 0x92E1;
pub const EQUAL: GLenum = 0x0202;
pub const RGB565: GLenum = 0x8D62;
pub const CONTEXT_RELEASE_BEHAVIOR: GLenum = 0x82FB;
pub const QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: GLenum = 0x8E4C;
pub const TEXTURE_IMAGE_FORMAT: GLenum = 0x828F;
pub const STENCIL_BACK_WRITEMASK: GLenum = 0x8CA5;
pub const MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: GLenum = 0x90DA;
pub const MAX_COMPUTE_WORK_GROUP_INVOCATIONS: GLenum = 0x90EB;
pub const STENCIL_COMPONENTS: GLenum = 0x8285;
pub const MAX_NUM_ACTIVE_VARIABLES: GLenum = 0x92F7;
pub const TEXTURE_1D: GLenum = 0x0DE0;
pub const CURRENT_QUERY: GLenum = 0x8865;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 0x8CD2;
pub const COLOR_ATTACHMENT2: GLenum = 0x8CE2;
pub const STREAM_READ: GLenum = 0x88E1;
pub const SAMPLER_2D_ARRAY: GLenum = 0x8DC1;
pub const INDEX: GLenum = 0x8222;
pub const DRAW_BUFFER15: GLenum = 0x8834;
pub const RED_INTEGER: GLenum = 0x8D94;
pub const TEXTURE_BUFFER_OFFSET: GLenum = 0x919D;
pub const BLEND_EQUATION_RGB: GLenum = 0x8009;
pub const BLEND: GLenum = 0x0BE2;
pub const TEXTURE_SWIZZLE_A: GLenum = 0x8E45;
pub const TEXTURE_BINDING_2D: GLenum = 0x8069;
pub const MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: GLenum = 0x92DC;
pub const LINEAR: GLenum = 0x2601;
pub const R32F: GLenum = 0x822E;
pub const QUERY_BY_REGION_NO_WAIT_INVERTED: GLenum = 0x8E1A;
pub const INVALID_OPERATION: GLenum = 0x0502;
pub const GET_TEXTURE_IMAGE_FORMAT: GLenum = 0x8291;
pub const MAX_COMPUTE_TEXTURE_IMAGE_UNITS: GLenum = 0x91BC;
pub const QUERY_BUFFER: GLenum = 0x9192;
pub const LOCATION_INDEX: GLenum = 0x930F;
pub const PROXY_TEXTURE_2D_MULTISAMPLE: GLenum = 0x9101;
pub const RESET_NOTIFICATION_STRATEGY: GLenum = 0x8256;
pub const SHADER_STORAGE_BUFFER: GLenum = 0x90D2;
pub const TEXTURE_VIEW: GLenum = 0x82B5;
pub const TESS_GEN_VERTEX_ORDER: GLenum = 0x8E78;
pub const RGB10_A2: GLenum = 0x8059;
pub const CAVEAT_SUPPORT: GLenum = 0x82B8;
pub const FLOAT_MAT4: GLenum = 0x8B5C;
pub const DEBUG_SOURCE_APPLICATION: GLenum = 0x824A;
pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLenum = 0x8C29;
pub const REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x930B;
pub const VIEW_CLASS_BPTC_UNORM: GLenum = 0x82D2;
pub const VIEW_CLASS_96_BITS: GLenum = 0x82C5;
pub const MIN_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5E;
pub const DISPATCH_INDIRECT_BUFFER_BINDING: GLenum = 0x90EF;
pub const PROGRAM_POINT_SIZE: GLenum = 0x8642;
pub const RG8I: GLenum = 0x8237;
pub const DRAW_BUFFER13: GLenum = 0x8832;
pub const R32I: GLenum = 0x8235;
pub const FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9314;
pub const STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 0x8802;
pub const SMOOTH_POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
pub const COLOR_ATTACHMENT3: GLenum = 0x8CE3;
pub const NICEST: GLenum = 0x1102;
pub const UNSIGNED_INT_VEC2: GLenum = 0x8DC6;
pub const SAMPLES_PASSED: GLenum = 0x8914;
pub const SAMPLER_1D_SHADOW: GLenum = 0x8B61;
pub const VIEWPORT_SUBPIXEL_BITS: GLenum = 0x825C;
pub const TEXTURE_MIN_FILTER: GLenum = 0x2801;
pub const ANY_SAMPLES_PASSED: GLenum = 0x8C2F;
pub const SIGNED_NORMALIZED: GLenum = 0x8F9C;
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x92C9;
pub const MAP_READ_BIT: GLenum = 0x0001;
pub const CONSTANT_ALPHA: GLenum = 0x8003;
pub const OR_REVERSE: GLenum = 0x150B;
pub const GUILTY_CONTEXT_RESET: GLenum = 0x8253;
pub const ATOMIC_COUNTER_BUFFER: GLenum = 0x92C0;
pub const INTERNALFORMAT_BLUE_SIZE: GLenum = 0x8273;
pub const CLAMP_TO_BORDER: GLenum = 0x812D;
pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 0x889F;
pub const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8A32;
pub const RENDERBUFFER: GLenum = 0x8D41;
pub const BUFFER_VARIABLE: GLenum = 0x92E5;
pub const ZERO_TO_ONE: GLenum = 0x935F;
pub const MAX_VIEWPORT_DIMS: GLenum = 0x0D3A;
pub const MAX_RENDERBUFFER_SIZE: GLenum = 0x84E8;
pub const CLAMP_TO_EDGE: GLenum = 0x812F;
pub const COMPRESSED_R11_EAC: GLenum = 0x9270;
pub const COMMAND_BARRIER_BIT: GLenum = 0x00000040;
pub const MAP_PERSISTENT_BIT: GLenum = 0x0040;
pub const FRONT: GLenum = 0x0404;
pub const CCW: GLenum = 0x0901;
pub const IMAGE_BUFFER: GLenum = 0x9051;
pub const PROGRAM_BINARY_FORMATS: GLenum = 0x87FF;
pub const MAX_DEBUG_LOGGED_MESSAGES: GLenum = 0x9144;
pub const DST_ALPHA: GLenum = 0x0304;
pub const R16: GLenum = 0x822A;
pub const FASTEST: GLenum = 0x1101;
pub const MAX_COMBINED_ATOMIC_COUNTERS: GLenum = 0x92D7;
pub const COLOR_BUFFER_BIT: GLenum = 0x00004000;
pub const QUERY_NO_WAIT: GLenum = 0x8E14;
pub const INT_IMAGE_3D: GLenum = 0x9059;
pub const EXTENSIONS: GLenum = 0x1F03;
pub const LINES_ADJACENCY: GLenum = 0x000A;
pub const TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9102;
pub const SYNC_CONDITION: GLenum = 0x9113;
pub const DRAW_BUFFER9: GLenum = 0x882E;
pub const ARRAY_SIZE: GLenum = 0x92FB;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 0x8CD3;
pub const ACTIVE_RESOURCES: GLenum = 0x92F5;
pub const TEXTURE_UPDATE_BARRIER_BIT: GLenum = 0x00000100;
pub const FALSE: GLboolean = 0;
pub const MAX_TESS_CONTROL_OUTPUT_COMPONENTS: GLenum = 0x8E83;
pub const MAX_SERVER_WAIT_TIMEOUT: GLenum = 0x9111;
pub const MAP_UNSYNCHRONIZED_BIT: GLenum = 0x0020;
pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x84F1;
pub const SAMPLE_MASK_VALUE: GLenum = 0x8E52;
pub const TRANSFORM_FEEDBACK_BARRIER_BIT: GLenum = 0x00000800;
pub const PACK_COMPRESSED_BLOCK_WIDTH: GLenum = 0x912B;
pub const RG32UI: GLenum = 0x823C;
pub const SMOOTH_POINT_SIZE_RANGE: GLenum = 0x0B12;
pub const COMPRESSED_SRGB_ALPHA: GLenum = 0x8C49;
pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = 0x8210;
pub const MAX_ELEMENTS_INDICES: GLenum = 0x80E9;
pub const TESS_GEN_POINT_MODE: GLenum = 0x8E79;
pub const CLIP_DISTANCE3: GLenum = 0x3003;
pub const CLIP_DISTANCE6: GLenum = 0x3006;
pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8A33;
pub const TEXTURE4: GLenum = 0x84C4;
pub const R32UI: GLenum = 0x8236;
pub const PIXEL_PACK_BUFFER_BINDING: GLenum = 0x88ED;
pub const DEBUG_SEVERITY_LOW: GLenum = 0x9148;
pub const UNSIGNED_INT_IMAGE_3D: GLenum = 0x9064;
pub const RGB5: GLenum = 0x8050;
pub const UNSIGNED_NORMALIZED: GLenum = 0x8C17;
pub const DRAW_BUFFER14: GLenum = 0x8833;
pub const VIEW_CLASS_8_BITS: GLenum = 0x82CB;
pub const COMPRESSED_RGBA_BPTC_UNORM: GLenum = 0x8E8C;
pub const UNPACK_SKIP_PIXELS: GLenum = 0x0CF4;

const ActiveShaderProgramIdx: u16 = 0;
const ActiveTextureIdx: u16 = 1;
const AttachShaderIdx: u16 = 2;
const BeginConditionalRenderIdx: u16 = 3;
const BeginQueryIdx: u16 = 4;
const BeginQueryIndexedIdx: u16 = 5;
const BeginTransformFeedbackIdx: u16 = 6;
const BindAttribLocationIdx: u16 = 7;
const BindBufferIdx: u16 = 8;
const BindBufferBaseIdx: u16 = 9;
const BindBufferRangeIdx: u16 = 10;
const BindBuffersBaseIdx: u16 = 11;
const BindBuffersRangeIdx: u16 = 12;
const BindFragDataLocationIdx: u16 = 13;
const BindFragDataLocationIndexedIdx: u16 = 14;
const BindFramebufferIdx: u16 = 15;
const BindImageTextureIdx: u16 = 16;
const BindImageTexturesIdx: u16 = 17;
const BindProgramPipelineIdx: u16 = 18;
const BindRenderbufferIdx: u16 = 19;
const BindSamplerIdx: u16 = 20;
const BindSamplersIdx: u16 = 21;
const BindTextureIdx: u16 = 22;
const BindTexturesIdx: u16 = 23;
const BindTextureUnitIdx: u16 = 24;
const BindTransformFeedbackIdx: u16 = 25;
const BindVertexArrayIdx: u16 = 26;
const BindVertexBufferIdx: u16 = 27;
const BindVertexBuffersIdx: u16 = 28;
const BlendColorIdx: u16 = 29;
const BlendEquationIdx: u16 = 30;
const BlendEquationiIdx: u16 = 31;
const BlendEquationSeparateIdx: u16 = 32;
const BlendEquationSeparateiIdx: u16 = 33;
const BlendFuncIdx: u16 = 34;
const BlendFunciIdx: u16 = 35;
const BlendFuncSeparateIdx: u16 = 36;
const BlendFuncSeparateiIdx: u16 = 37;
const BlitFramebufferIdx: u16 = 38;
const BlitNamedFramebufferIdx: u16 = 39;
const BufferDataIdx: u16 = 40;
const BufferStorageIdx: u16 = 41;
const BufferSubDataIdx: u16 = 42;
const CheckFramebufferStatusIdx: u16 = 43;
const CheckNamedFramebufferStatusIdx: u16 = 44;
const ClampColorIdx: u16 = 45;
const ClearIdx: u16 = 46;
const ClearBufferDataIdx: u16 = 47;
const ClearBufferfiIdx: u16 = 48;
const ClearBufferfvIdx: u16 = 49;
const ClearBufferivIdx: u16 = 50;
const ClearBufferSubDataIdx: u16 = 51;
const ClearBufferuivIdx: u16 = 52;
const ClearColorIdx: u16 = 53;
const ClearDepthIdx: u16 = 54;
const ClearDepthfIdx: u16 = 55;
const ClearNamedBufferDataIdx: u16 = 56;
const ClearNamedBufferSubDataIdx: u16 = 57;
const ClearNamedFramebufferfiIdx: u16 = 58;
const ClearNamedFramebufferfvIdx: u16 = 59;
const ClearNamedFramebufferivIdx: u16 = 60;
const ClearNamedFramebufferuivIdx: u16 = 61;
const ClearStencilIdx: u16 = 62;
const ClearTexImageIdx: u16 = 63;
const ClearTexSubImageIdx: u16 = 64;
const ClientWaitSyncIdx: u16 = 65;
const ClipControlIdx: u16 = 66;
const ColorMaskIdx: u16 = 67;
const ColorMaskiIdx: u16 = 68;
const ColorP3uiIdx: u16 = 69;
const ColorP3uivIdx: u16 = 70;
const ColorP4uiIdx: u16 = 71;
const ColorP4uivIdx: u16 = 72;
const CompileShaderIdx: u16 = 73;
const CompressedTexImage1DIdx: u16 = 74;
const CompressedTexImage2DIdx: u16 = 75;
const CompressedTexImage3DIdx: u16 = 76;
const CompressedTexSubImage1DIdx: u16 = 77;
const CompressedTexSubImage2DIdx: u16 = 78;
const CompressedTexSubImage3DIdx: u16 = 79;
const CompressedTextureSubImage1DIdx: u16 = 80;
const CompressedTextureSubImage2DIdx: u16 = 81;
const CompressedTextureSubImage3DIdx: u16 = 82;
const CopyBufferSubDataIdx: u16 = 83;
const CopyImageSubDataIdx: u16 = 84;
const CopyNamedBufferSubDataIdx: u16 = 85;
const CopyTexImage1DIdx: u16 = 86;
const CopyTexImage2DIdx: u16 = 87;
const CopyTexSubImage1DIdx: u16 = 88;
const CopyTexSubImage2DIdx: u16 = 89;
const CopyTexSubImage3DIdx: u16 = 90;
const CopyTextureSubImage1DIdx: u16 = 91;
const CopyTextureSubImage2DIdx: u16 = 92;
const CopyTextureSubImage3DIdx: u16 = 93;
const CreateBuffersIdx: u16 = 94;
const CreateFramebuffersIdx: u16 = 95;
const CreateProgramIdx: u16 = 96;
const CreateProgramPipelinesIdx: u16 = 97;
const CreateQueriesIdx: u16 = 98;
const CreateRenderbuffersIdx: u16 = 99;
const CreateSamplersIdx: u16 = 100;
const CreateShaderIdx: u16 = 101;
const CreateShaderProgramvIdx: u16 = 102;
const CreateTexturesIdx: u16 = 103;
const CreateTransformFeedbacksIdx: u16 = 104;
const CreateVertexArraysIdx: u16 = 105;
const CullFaceIdx: u16 = 106;
const DebugMessageCallbackIdx: u16 = 107;
const DebugMessageControlIdx: u16 = 108;
const DebugMessageInsertIdx: u16 = 109;
const DeleteBuffersIdx: u16 = 110;
const DeleteFramebuffersIdx: u16 = 111;
const DeleteProgramIdx: u16 = 112;
const DeleteProgramPipelinesIdx: u16 = 113;
const DeleteQueriesIdx: u16 = 114;
const DeleteRenderbuffersIdx: u16 = 115;
const DeleteSamplersIdx: u16 = 116;
const DeleteShaderIdx: u16 = 117;
const DeleteSyncIdx: u16 = 118;
const DeleteTexturesIdx: u16 = 119;
const DeleteTransformFeedbacksIdx: u16 = 120;
const DeleteVertexArraysIdx: u16 = 121;
const DepthFuncIdx: u16 = 122;
const DepthMaskIdx: u16 = 123;
const DepthRangeIdx: u16 = 124;
const DepthRangeArrayvIdx: u16 = 125;
const DepthRangefIdx: u16 = 126;
const DepthRangeIndexedIdx: u16 = 127;
const DetachShaderIdx: u16 = 128;
const DisableIdx: u16 = 129;
const DisableiIdx: u16 = 130;
const DisableVertexArrayAttribIdx: u16 = 131;
const DisableVertexAttribArrayIdx: u16 = 132;
const DispatchComputeIdx: u16 = 133;
const DispatchComputeIndirectIdx: u16 = 134;
const DrawArraysIdx: u16 = 135;
const DrawArraysIndirectIdx: u16 = 136;
const DrawArraysInstancedIdx: u16 = 137;
const DrawArraysInstancedBaseInstanceIdx: u16 = 138;
const DrawBufferIdx: u16 = 139;
const DrawBuffersIdx: u16 = 140;
const DrawElementsIdx: u16 = 141;
const DrawElementsBaseVertexIdx: u16 = 142;
const DrawElementsIndirectIdx: u16 = 143;
const DrawElementsInstancedIdx: u16 = 144;
const DrawElementsInstancedBaseInstanceIdx: u16 = 145;
const DrawElementsInstancedBaseVertexIdx: u16 = 146;
const DrawElementsInstancedBaseVertexBaseInstanceIdx: u16 = 147;
const DrawRangeElementsIdx: u16 = 148;
const DrawRangeElementsBaseVertexIdx: u16 = 149;
const DrawTransformFeedbackIdx: u16 = 150;
const DrawTransformFeedbackInstancedIdx: u16 = 151;
const DrawTransformFeedbackStreamIdx: u16 = 152;
const DrawTransformFeedbackStreamInstancedIdx: u16 = 153;
const EnableIdx: u16 = 154;
const EnableiIdx: u16 = 155;
const EnableVertexArrayAttribIdx: u16 = 156;
const EnableVertexAttribArrayIdx: u16 = 157;
const EndConditionalRenderIdx: u16 = 158;
const EndQueryIdx: u16 = 159;
const EndQueryIndexedIdx: u16 = 160;
const EndTransformFeedbackIdx: u16 = 161;
const FenceSyncIdx: u16 = 162;
const FinishIdx: u16 = 163;
const FlushIdx: u16 = 164;
const FlushMappedBufferRangeIdx: u16 = 165;
const FlushMappedNamedBufferRangeIdx: u16 = 166;
const FramebufferParameteriIdx: u16 = 167;
const FramebufferRenderbufferIdx: u16 = 168;
const FramebufferTexture1DIdx: u16 = 169;
const FramebufferTexture2DIdx: u16 = 170;
const FramebufferTexture3DIdx: u16 = 171;
const FramebufferTextureIdx: u16 = 172;
const FramebufferTextureLayerIdx: u16 = 173;
const FrontFaceIdx: u16 = 174;
const GenBuffersIdx: u16 = 175;
const GenerateMipmapIdx: u16 = 176;
const GenerateTextureMipmapIdx: u16 = 177;
const GenFramebuffersIdx: u16 = 178;
const GenProgramPipelinesIdx: u16 = 179;
const GenQueriesIdx: u16 = 180;
const GenRenderbuffersIdx: u16 = 181;
const GenSamplersIdx: u16 = 182;
const GenTexturesIdx: u16 = 183;
const GenTransformFeedbacksIdx: u16 = 184;
const GenVertexArraysIdx: u16 = 185;
const GetActiveAtomicCounterBufferivIdx: u16 = 186;
const GetActiveAttribIdx: u16 = 187;
const GetActiveSubroutineNameIdx: u16 = 188;
const GetActiveSubroutineUniformivIdx: u16 = 189;
const GetActiveSubroutineUniformNameIdx: u16 = 190;
const GetActiveUniformIdx: u16 = 191;
const GetActiveUniformBlockivIdx: u16 = 192;
const GetActiveUniformBlockNameIdx: u16 = 193;
const GetActiveUniformNameIdx: u16 = 194;
const GetActiveUniformsivIdx: u16 = 195;
const GetAttachedShadersIdx: u16 = 196;
const GetAttribLocationIdx: u16 = 197;
const GetBooleani_vIdx: u16 = 198;
const GetBooleanvIdx: u16 = 199;
const GetBufferParameteri64vIdx: u16 = 200;
const GetBufferParameterivIdx: u16 = 201;
const GetBufferPointervIdx: u16 = 202;
const GetBufferSubDataIdx: u16 = 203;
const GetCompressedTexImageIdx: u16 = 204;
const GetCompressedTextureImageIdx: u16 = 205;
const GetCompressedTextureSubImageIdx: u16 = 206;
const GetDebugMessageLogIdx: u16 = 207;
const GetDoublei_vIdx: u16 = 208;
const GetDoublevIdx: u16 = 209;
const GetErrorIdx: u16 = 210;
const GetFloati_vIdx: u16 = 211;
const GetFloatvIdx: u16 = 212;
const GetFragDataIndexIdx: u16 = 213;
const GetFragDataLocationIdx: u16 = 214;
const GetFramebufferAttachmentParameterivIdx: u16 = 215;
const GetFramebufferParameterivIdx: u16 = 216;
const GetGraphicsResetStatusIdx: u16 = 217;
const GetInteger64i_vIdx: u16 = 218;
const GetInteger64vIdx: u16 = 219;
const GetIntegeri_vIdx: u16 = 220;
const GetIntegervIdx: u16 = 221;
const GetInternalformati64vIdx: u16 = 222;
const GetInternalformativIdx: u16 = 223;
const GetMultisamplefvIdx: u16 = 224;
const GetNamedBufferParameteri64vIdx: u16 = 225;
const GetNamedBufferParameterivIdx: u16 = 226;
const GetNamedBufferPointervIdx: u16 = 227;
const GetNamedBufferSubDataIdx: u16 = 228;
const GetNamedFramebufferAttachmentParameterivIdx: u16 = 229;
const GetNamedFramebufferParameterivIdx: u16 = 230;
const GetNamedRenderbufferParameterivIdx: u16 = 231;
const GetnColorTableIdx: u16 = 232;
const GetnCompressedTexImageIdx: u16 = 233;
const GetnConvolutionFilterIdx: u16 = 234;
const GetnHistogramIdx: u16 = 235;
const GetnMapdvIdx: u16 = 236;
const GetnMapfvIdx: u16 = 237;
const GetnMapivIdx: u16 = 238;
const GetnMinmaxIdx: u16 = 239;
const GetnPixelMapfvIdx: u16 = 240;
const GetnPixelMapuivIdx: u16 = 241;
const GetnPixelMapusvIdx: u16 = 242;
const GetnPolygonStippleIdx: u16 = 243;
const GetnSeparableFilterIdx: u16 = 244;
const GetnTexImageIdx: u16 = 245;
const GetnUniformdvIdx: u16 = 246;
const GetnUniformfvIdx: u16 = 247;
const GetnUniformivIdx: u16 = 248;
const GetnUniformuivIdx: u16 = 249;
const GetObjectLabelIdx: u16 = 250;
const GetObjectPtrLabelIdx: u16 = 251;
const GetPointervIdx: u16 = 252;
const GetProgramBinaryIdx: u16 = 253;
const GetProgramInfoLogIdx: u16 = 254;
const GetProgramInterfaceivIdx: u16 = 255;
const GetProgramivIdx: u16 = 256;
const GetProgramPipelineInfoLogIdx: u16 = 257;
const GetProgramPipelineivIdx: u16 = 258;
const GetProgramResourceIndexIdx: u16 = 259;
const GetProgramResourceivIdx: u16 = 260;
const GetProgramResourceLocationIdx: u16 = 261;
const GetProgramResourceLocationIndexIdx: u16 = 262;
const GetProgramResourceNameIdx: u16 = 263;
const GetProgramStageivIdx: u16 = 264;
const GetQueryBufferObjecti64vIdx: u16 = 265;
const GetQueryBufferObjectivIdx: u16 = 266;
const GetQueryBufferObjectui64vIdx: u16 = 267;
const GetQueryBufferObjectuivIdx: u16 = 268;
const GetQueryIndexedivIdx: u16 = 269;
const GetQueryivIdx: u16 = 270;
const GetQueryObjecti64vIdx: u16 = 271;
const GetQueryObjectivIdx: u16 = 272;
const GetQueryObjectui64vIdx: u16 = 273;
const GetQueryObjectuivIdx: u16 = 274;
const GetRenderbufferParameterivIdx: u16 = 275;
const GetSamplerParameterfvIdx: u16 = 276;
const GetSamplerParameterIivIdx: u16 = 277;
const GetSamplerParameterIuivIdx: u16 = 278;
const GetSamplerParameterivIdx: u16 = 279;
const GetShaderInfoLogIdx: u16 = 280;
const GetShaderivIdx: u16 = 281;
const GetShaderPrecisionFormatIdx: u16 = 282;
const GetShaderSourceIdx: u16 = 283;
const GetStringIdx: u16 = 284;
const GetStringiIdx: u16 = 285;
const GetSubroutineIndexIdx: u16 = 286;
const GetSubroutineUniformLocationIdx: u16 = 287;
const GetSyncivIdx: u16 = 288;
const GetTexImageIdx: u16 = 289;
const GetTexLevelParameterfvIdx: u16 = 290;
const GetTexLevelParameterivIdx: u16 = 291;
const GetTexParameterfvIdx: u16 = 292;
const GetTexParameterIivIdx: u16 = 293;
const GetTexParameterIuivIdx: u16 = 294;
const GetTexParameterivIdx: u16 = 295;
const GetTextureImageIdx: u16 = 296;
const GetTextureLevelParameterfvIdx: u16 = 297;
const GetTextureLevelParameterivIdx: u16 = 298;
const GetTextureParameterfvIdx: u16 = 299;
const GetTextureParameterIivIdx: u16 = 300;
const GetTextureParameterIuivIdx: u16 = 301;
const GetTextureParameterivIdx: u16 = 302;
const GetTextureSubImageIdx: u16 = 303;
const GetTransformFeedbacki64_vIdx: u16 = 304;
const GetTransformFeedbacki_vIdx: u16 = 305;
const GetTransformFeedbackivIdx: u16 = 306;
const GetTransformFeedbackVaryingIdx: u16 = 307;
const GetUniformBlockIndexIdx: u16 = 308;
const GetUniformdvIdx: u16 = 309;
const GetUniformfvIdx: u16 = 310;
const GetUniformIndicesIdx: u16 = 311;
const GetUniformivIdx: u16 = 312;
const GetUniformLocationIdx: u16 = 313;
const GetUniformSubroutineuivIdx: u16 = 314;
const GetUniformuivIdx: u16 = 315;
const GetVertexArrayIndexed64ivIdx: u16 = 316;
const GetVertexArrayIndexedivIdx: u16 = 317;
const GetVertexArrayivIdx: u16 = 318;
const GetVertexAttribdvIdx: u16 = 319;
const GetVertexAttribfvIdx: u16 = 320;
const GetVertexAttribIivIdx: u16 = 321;
const GetVertexAttribIuivIdx: u16 = 322;
const GetVertexAttribivIdx: u16 = 323;
const GetVertexAttribLdvIdx: u16 = 324;
const GetVertexAttribPointervIdx: u16 = 325;
const HintIdx: u16 = 326;
const InvalidateBufferDataIdx: u16 = 327;
const InvalidateBufferSubDataIdx: u16 = 328;
const InvalidateFramebufferIdx: u16 = 329;
const InvalidateNamedFramebufferDataIdx: u16 = 330;
const InvalidateNamedFramebufferSubDataIdx: u16 = 331;
const InvalidateSubFramebufferIdx: u16 = 332;
const InvalidateTexImageIdx: u16 = 333;
const InvalidateTexSubImageIdx: u16 = 334;
const IsBufferIdx: u16 = 335;
const IsEnabledIdx: u16 = 336;
const IsEnablediIdx: u16 = 337;
const IsFramebufferIdx: u16 = 338;
const IsProgramIdx: u16 = 339;
const IsProgramPipelineIdx: u16 = 340;
const IsQueryIdx: u16 = 341;
const IsRenderbufferIdx: u16 = 342;
const IsSamplerIdx: u16 = 343;
const IsShaderIdx: u16 = 344;
const IsSyncIdx: u16 = 345;
const IsTextureIdx: u16 = 346;
const IsTransformFeedbackIdx: u16 = 347;
const IsVertexArrayIdx: u16 = 348;
const LineWidthIdx: u16 = 349;
const LinkProgramIdx: u16 = 350;
const LogicOpIdx: u16 = 351;
const MapBufferIdx: u16 = 352;
const MapBufferRangeIdx: u16 = 353;
const MapNamedBufferIdx: u16 = 354;
const MapNamedBufferRangeIdx: u16 = 355;
const MemoryBarrierIdx: u16 = 356;
const MemoryBarrierByRegionIdx: u16 = 357;
const MinSampleShadingIdx: u16 = 358;
const MultiDrawArraysIdx: u16 = 359;
const MultiDrawArraysIndirectIdx: u16 = 360;
const MultiDrawElementsIdx: u16 = 361;
const MultiDrawElementsBaseVertexIdx: u16 = 362;
const MultiDrawElementsIndirectIdx: u16 = 363;
const MultiTexCoordP1uiIdx: u16 = 364;
const MultiTexCoordP1uivIdx: u16 = 365;
const MultiTexCoordP2uiIdx: u16 = 366;
const MultiTexCoordP2uivIdx: u16 = 367;
const MultiTexCoordP3uiIdx: u16 = 368;
const MultiTexCoordP3uivIdx: u16 = 369;
const MultiTexCoordP4uiIdx: u16 = 370;
const MultiTexCoordP4uivIdx: u16 = 371;
const NamedBufferDataIdx: u16 = 372;
const NamedBufferStorageIdx: u16 = 373;
const NamedBufferSubDataIdx: u16 = 374;
const NamedFramebufferDrawBufferIdx: u16 = 375;
const NamedFramebufferDrawBuffersIdx: u16 = 376;
const NamedFramebufferParameteriIdx: u16 = 377;
const NamedFramebufferReadBufferIdx: u16 = 378;
const NamedFramebufferRenderbufferIdx: u16 = 379;
const NamedFramebufferTextureIdx: u16 = 380;
const NamedFramebufferTextureLayerIdx: u16 = 381;
const NamedRenderbufferStorageIdx: u16 = 382;
const NamedRenderbufferStorageMultisampleIdx: u16 = 383;
const NormalP3uiIdx: u16 = 384;
const NormalP3uivIdx: u16 = 385;
const ObjectLabelIdx: u16 = 386;
const ObjectPtrLabelIdx: u16 = 387;
const PatchParameterfvIdx: u16 = 388;
const PatchParameteriIdx: u16 = 389;
const PauseTransformFeedbackIdx: u16 = 390;
const PixelStorefIdx: u16 = 391;
const PixelStoreiIdx: u16 = 392;
const PointParameterfIdx: u16 = 393;
const PointParameterfvIdx: u16 = 394;
const PointParameteriIdx: u16 = 395;
const PointParameterivIdx: u16 = 396;
const PointSizeIdx: u16 = 397;
const PolygonModeIdx: u16 = 398;
const PolygonOffsetIdx: u16 = 399;
const PopDebugGroupIdx: u16 = 400;
const PrimitiveRestartIndexIdx: u16 = 401;
const ProgramBinaryIdx: u16 = 402;
const ProgramParameteriIdx: u16 = 403;
const ProgramUniform1dIdx: u16 = 404;
const ProgramUniform1dvIdx: u16 = 405;
const ProgramUniform1fIdx: u16 = 406;
const ProgramUniform1fvIdx: u16 = 407;
const ProgramUniform1iIdx: u16 = 408;
const ProgramUniform1ivIdx: u16 = 409;
const ProgramUniform1uiIdx: u16 = 410;
const ProgramUniform1uivIdx: u16 = 411;
const ProgramUniform2dIdx: u16 = 412;
const ProgramUniform2dvIdx: u16 = 413;
const ProgramUniform2fIdx: u16 = 414;
const ProgramUniform2fvIdx: u16 = 415;
const ProgramUniform2iIdx: u16 = 416;
const ProgramUniform2ivIdx: u16 = 417;
const ProgramUniform2uiIdx: u16 = 418;
const ProgramUniform2uivIdx: u16 = 419;
const ProgramUniform3dIdx: u16 = 420;
const ProgramUniform3dvIdx: u16 = 421;
const ProgramUniform3fIdx: u16 = 422;
const ProgramUniform3fvIdx: u16 = 423;
const ProgramUniform3iIdx: u16 = 424;
const ProgramUniform3ivIdx: u16 = 425;
const ProgramUniform3uiIdx: u16 = 426;
const ProgramUniform3uivIdx: u16 = 427;
const ProgramUniform4dIdx: u16 = 428;
const ProgramUniform4dvIdx: u16 = 429;
const ProgramUniform4fIdx: u16 = 430;
const ProgramUniform4fvIdx: u16 = 431;
const ProgramUniform4iIdx: u16 = 432;
const ProgramUniform4ivIdx: u16 = 433;
const ProgramUniform4uiIdx: u16 = 434;
const ProgramUniform4uivIdx: u16 = 435;
const ProgramUniformMatrix2dvIdx: u16 = 436;
const ProgramUniformMatrix2fvIdx: u16 = 437;
const ProgramUniformMatrix2x3dvIdx: u16 = 438;
const ProgramUniformMatrix2x3fvIdx: u16 = 439;
const ProgramUniformMatrix2x4dvIdx: u16 = 440;
const ProgramUniformMatrix2x4fvIdx: u16 = 441;
const ProgramUniformMatrix3dvIdx: u16 = 442;
const ProgramUniformMatrix3fvIdx: u16 = 443;
const ProgramUniformMatrix3x2dvIdx: u16 = 444;
const ProgramUniformMatrix3x2fvIdx: u16 = 445;
const ProgramUniformMatrix3x4dvIdx: u16 = 446;
const ProgramUniformMatrix3x4fvIdx: u16 = 447;
const ProgramUniformMatrix4dvIdx: u16 = 448;
const ProgramUniformMatrix4fvIdx: u16 = 449;
const ProgramUniformMatrix4x2dvIdx: u16 = 450;
const ProgramUniformMatrix4x2fvIdx: u16 = 451;
const ProgramUniformMatrix4x3dvIdx: u16 = 452;
const ProgramUniformMatrix4x3fvIdx: u16 = 453;
const ProvokingVertexIdx: u16 = 454;
const PushDebugGroupIdx: u16 = 455;
const QueryCounterIdx: u16 = 456;
const ReadBufferIdx: u16 = 457;
const ReadnPixelsIdx: u16 = 458;
const ReadPixelsIdx: u16 = 459;
const ReleaseShaderCompilerIdx: u16 = 460;
const RenderbufferStorageIdx: u16 = 461;
const RenderbufferStorageMultisampleIdx: u16 = 462;
const ResumeTransformFeedbackIdx: u16 = 463;
const SampleCoverageIdx: u16 = 464;
const SampleMaskiIdx: u16 = 465;
const SamplerParameterfIdx: u16 = 466;
const SamplerParameterfvIdx: u16 = 467;
const SamplerParameteriIdx: u16 = 468;
const SamplerParameterIivIdx: u16 = 469;
const SamplerParameterIuivIdx: u16 = 470;
const SamplerParameterivIdx: u16 = 471;
const ScissorIdx: u16 = 472;
const ScissorArrayvIdx: u16 = 473;
const ScissorIndexedIdx: u16 = 474;
const ScissorIndexedvIdx: u16 = 475;
const SecondaryColorP3uiIdx: u16 = 476;
const SecondaryColorP3uivIdx: u16 = 477;
const ShaderBinaryIdx: u16 = 478;
const ShaderSourceIdx: u16 = 479;
const ShaderStorageBlockBindingIdx: u16 = 480;
const StencilFuncIdx: u16 = 481;
const StencilFuncSeparateIdx: u16 = 482;
const StencilMaskIdx: u16 = 483;
const StencilMaskSeparateIdx: u16 = 484;
const StencilOpIdx: u16 = 485;
const StencilOpSeparateIdx: u16 = 486;
const TexBufferIdx: u16 = 487;
const TexBufferRangeIdx: u16 = 488;
const TexCoordP1uiIdx: u16 = 489;
const TexCoordP1uivIdx: u16 = 490;
const TexCoordP2uiIdx: u16 = 491;
const TexCoordP2uivIdx: u16 = 492;
const TexCoordP3uiIdx: u16 = 493;
const TexCoordP3uivIdx: u16 = 494;
const TexCoordP4uiIdx: u16 = 495;
const TexCoordP4uivIdx: u16 = 496;
const TexImage1DIdx: u16 = 497;
const TexImage2DIdx: u16 = 498;
const TexImage2DMultisampleIdx: u16 = 499;
const TexImage3DIdx: u16 = 500;
const TexImage3DMultisampleIdx: u16 = 501;
const TexParameterfIdx: u16 = 502;
const TexParameterfvIdx: u16 = 503;
const TexParameteriIdx: u16 = 504;
const TexParameterIivIdx: u16 = 505;
const TexParameterIuivIdx: u16 = 506;
const TexParameterivIdx: u16 = 507;
const TexStorage1DIdx: u16 = 508;
const TexStorage2DIdx: u16 = 509;
const TexStorage2DMultisampleIdx: u16 = 510;
const TexStorage3DIdx: u16 = 511;
const TexStorage3DMultisampleIdx: u16 = 512;
const TexSubImage1DIdx: u16 = 513;
const TexSubImage2DIdx: u16 = 514;
const TexSubImage3DIdx: u16 = 515;
const TextureBarrierIdx: u16 = 516;
const TextureBufferIdx: u16 = 517;
const TextureBufferRangeIdx: u16 = 518;
const TextureParameterfIdx: u16 = 519;
const TextureParameterfvIdx: u16 = 520;
const TextureParameteriIdx: u16 = 521;
const TextureParameterIivIdx: u16 = 522;
const TextureParameterIuivIdx: u16 = 523;
const TextureParameterivIdx: u16 = 524;
const TextureStorage1DIdx: u16 = 525;
const TextureStorage2DIdx: u16 = 526;
const TextureStorage2DMultisampleIdx: u16 = 527;
const TextureStorage3DIdx: u16 = 528;
const TextureStorage3DMultisampleIdx: u16 = 529;
const TextureSubImage1DIdx: u16 = 530;
const TextureSubImage2DIdx: u16 = 531;
const TextureSubImage3DIdx: u16 = 532;
const TextureViewIdx: u16 = 533;
const TransformFeedbackBufferBaseIdx: u16 = 534;
const TransformFeedbackBufferRangeIdx: u16 = 535;
const TransformFeedbackVaryingsIdx: u16 = 536;
const Uniform1dIdx: u16 = 537;
const Uniform1dvIdx: u16 = 538;
const Uniform1fIdx: u16 = 539;
const Uniform1fvIdx: u16 = 540;
const Uniform1iIdx: u16 = 541;
const Uniform1ivIdx: u16 = 542;
const Uniform1uiIdx: u16 = 543;
const Uniform1uivIdx: u16 = 544;
const Uniform2dIdx: u16 = 545;
const Uniform2dvIdx: u16 = 546;
const Uniform2fIdx: u16 = 547;
const Uniform2fvIdx: u16 = 548;
const Uniform2iIdx: u16 = 549;
const Uniform2ivIdx: u16 = 550;
const Uniform2uiIdx: u16 = 551;
const Uniform2uivIdx: u16 = 552;
const Uniform3dIdx: u16 = 553;
const Uniform3dvIdx: u16 = 554;
const Uniform3fIdx: u16 = 555;
const Uniform3fvIdx: u16 = 556;
const Uniform3iIdx: u16 = 557;
const Uniform3ivIdx: u16 = 558;
const Uniform3uiIdx: u16 = 559;
const Uniform3uivIdx: u16 = 560;
const Uniform4dIdx: u16 = 561;
const Uniform4dvIdx: u16 = 562;
const Uniform4fIdx: u16 = 563;
const Uniform4fvIdx: u16 = 564;
const Uniform4iIdx: u16 = 565;
const Uniform4ivIdx: u16 = 566;
const Uniform4uiIdx: u16 = 567;
const Uniform4uivIdx: u16 = 568;
const UniformBlockBindingIdx: u16 = 569;
const UniformMatrix2dvIdx: u16 = 570;
const UniformMatrix2fvIdx: u16 = 571;
const UniformMatrix2x3dvIdx: u16 = 572;
const UniformMatrix2x3fvIdx: u16 = 573;
const UniformMatrix2x4dvIdx: u16 = 574;
const UniformMatrix2x4fvIdx: u16 = 575;
const UniformMatrix3dvIdx: u16 = 576;
const UniformMatrix3fvIdx: u16 = 577;
const UniformMatrix3x2dvIdx: u16 = 578;
const UniformMatrix3x2fvIdx: u16 = 579;
const UniformMatrix3x4dvIdx: u16 = 580;
const UniformMatrix3x4fvIdx: u16 = 581;
const UniformMatrix4dvIdx: u16 = 582;
const UniformMatrix4fvIdx: u16 = 583;
const UniformMatrix4x2dvIdx: u16 = 584;
const UniformMatrix4x2fvIdx: u16 = 585;
const UniformMatrix4x3dvIdx: u16 = 586;
const UniformMatrix4x3fvIdx: u16 = 587;
const UniformSubroutinesuivIdx: u16 = 588;
const UnmapBufferIdx: u16 = 589;
const UnmapNamedBufferIdx: u16 = 590;
const UseProgramIdx: u16 = 591;
const UseProgramStagesIdx: u16 = 592;
const ValidateProgramIdx: u16 = 593;
const ValidateProgramPipelineIdx: u16 = 594;
const VertexArrayAttribBindingIdx: u16 = 595;
const VertexArrayAttribFormatIdx: u16 = 596;
const VertexArrayAttribIFormatIdx: u16 = 597;
const VertexArrayAttribLFormatIdx: u16 = 598;
const VertexArrayBindingDivisorIdx: u16 = 599;
const VertexArrayElementBufferIdx: u16 = 600;
const VertexArrayVertexBufferIdx: u16 = 601;
const VertexArrayVertexBuffersIdx: u16 = 602;
const VertexAttrib1dIdx: u16 = 603;
const VertexAttrib1dvIdx: u16 = 604;
const VertexAttrib1fIdx: u16 = 605;
const VertexAttrib1fvIdx: u16 = 606;
const VertexAttrib1sIdx: u16 = 607;
const VertexAttrib1svIdx: u16 = 608;
const VertexAttrib2dIdx: u16 = 609;
const VertexAttrib2dvIdx: u16 = 610;
const VertexAttrib2fIdx: u16 = 611;
const VertexAttrib2fvIdx: u16 = 612;
const VertexAttrib2sIdx: u16 = 613;
const VertexAttrib2svIdx: u16 = 614;
const VertexAttrib3dIdx: u16 = 615;
const VertexAttrib3dvIdx: u16 = 616;
const VertexAttrib3fIdx: u16 = 617;
const VertexAttrib3fvIdx: u16 = 618;
const VertexAttrib3sIdx: u16 = 619;
const VertexAttrib3svIdx: u16 = 620;
const VertexAttrib4bvIdx: u16 = 621;
const VertexAttrib4dIdx: u16 = 622;
const VertexAttrib4dvIdx: u16 = 623;
const VertexAttrib4fIdx: u16 = 624;
const VertexAttrib4fvIdx: u16 = 625;
const VertexAttrib4ivIdx: u16 = 626;
const VertexAttrib4NbvIdx: u16 = 627;
const VertexAttrib4NivIdx: u16 = 628;
const VertexAttrib4NsvIdx: u16 = 629;
const VertexAttrib4NubIdx: u16 = 630;
const VertexAttrib4NubvIdx: u16 = 631;
const VertexAttrib4NuivIdx: u16 = 632;
const VertexAttrib4NusvIdx: u16 = 633;
const VertexAttrib4sIdx: u16 = 634;
const VertexAttrib4svIdx: u16 = 635;
const VertexAttrib4ubvIdx: u16 = 636;
const VertexAttrib4uivIdx: u16 = 637;
const VertexAttrib4usvIdx: u16 = 638;
const VertexAttribBindingIdx: u16 = 639;
const VertexAttribDivisorIdx: u16 = 640;
const VertexAttribFormatIdx: u16 = 641;
const VertexAttribI1iIdx: u16 = 642;
const VertexAttribI1ivIdx: u16 = 643;
const VertexAttribI1uiIdx: u16 = 644;
const VertexAttribI1uivIdx: u16 = 645;
const VertexAttribI2iIdx: u16 = 646;
const VertexAttribI2ivIdx: u16 = 647;
const VertexAttribI2uiIdx: u16 = 648;
const VertexAttribI2uivIdx: u16 = 649;
const VertexAttribI3iIdx: u16 = 650;
const VertexAttribI3ivIdx: u16 = 651;
const VertexAttribI3uiIdx: u16 = 652;
const VertexAttribI3uivIdx: u16 = 653;
const VertexAttribI4bvIdx: u16 = 654;
const VertexAttribI4iIdx: u16 = 655;
const VertexAttribI4ivIdx: u16 = 656;
const VertexAttribI4svIdx: u16 = 657;
const VertexAttribI4ubvIdx: u16 = 658;
const VertexAttribI4uiIdx: u16 = 659;
const VertexAttribI4uivIdx: u16 = 660;
const VertexAttribI4usvIdx: u16 = 661;
const VertexAttribIFormatIdx: u16 = 662;
const VertexAttribIPointerIdx: u16 = 663;
const VertexAttribL1dIdx: u16 = 664;
const VertexAttribL1dvIdx: u16 = 665;
const VertexAttribL2dIdx: u16 = 666;
const VertexAttribL2dvIdx: u16 = 667;
const VertexAttribL3dIdx: u16 = 668;
const VertexAttribL3dvIdx: u16 = 669;
const VertexAttribL4dIdx: u16 = 670;
const VertexAttribL4dvIdx: u16 = 671;
const VertexAttribLFormatIdx: u16 = 672;
const VertexAttribLPointerIdx: u16 = 673;
const VertexAttribP1uiIdx: u16 = 674;
const VertexAttribP1uivIdx: u16 = 675;
const VertexAttribP2uiIdx: u16 = 676;
const VertexAttribP2uivIdx: u16 = 677;
const VertexAttribP3uiIdx: u16 = 678;
const VertexAttribP3uivIdx: u16 = 679;
const VertexAttribP4uiIdx: u16 = 680;
const VertexAttribP4uivIdx: u16 = 681;
const VertexAttribPointerIdx: u16 = 682;
const VertexBindingDivisorIdx: u16 = 683;
const VertexP2uiIdx: u16 = 684;
const VertexP2uivIdx: u16 = 685;
const VertexP3uiIdx: u16 = 686;
const VertexP3uivIdx: u16 = 687;
const VertexP4uiIdx: u16 = 688;
const VertexP4uivIdx: u16 = 689;
const ViewportIdx: u16 = 690;
const ViewportArrayvIdx: u16 = 691;
const ViewportIndexedfIdx: u16 = 692;
const ViewportIndexedfvIdx: u16 = 693;
const WaitSyncIdx: u16 = 694;

/// Fallbacks: EndTransformFeedbackEXT, EndTransformFeedbackNV
#[inline]
pub unsafe fn EndTransformFeedback() {
    unsafe {
        mem::transmute::<_, extern "system" fn() -> ()>(
            *GL_API.get_unchecked(EndTransformFeedbackIdx as usize),
        )()
    }
}
#[inline]
pub unsafe fn GetProgramResourceLocationIndex(
    program: GLuint,
    programInterface: GLenum,
    name: *const GLchar,
) -> GLint {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint>(
            *GL_API.get_unchecked(GetProgramResourceLocationIndexIdx as usize),
        )(program, programInterface, name)
    }
}
#[inline]
pub unsafe fn GetProgramResourceLocation(
    program: GLuint,
    programInterface: GLenum,
    name: *const GLchar,
) -> GLint {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint>(
            *GL_API.get_unchecked(GetProgramResourceLocationIdx as usize),
        )(program, programInterface, name)
    }
}
/// Fallbacks: VertexAttribL3dEXT
#[inline]
pub unsafe fn VertexAttribL3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble) -> ()>(
            *GL_API.get_unchecked(VertexAttribL3dIdx as usize),
        )(index, x, y, z)
    }
}
/// Fallbacks: ObjectPtrLabelKHR
#[inline]
pub unsafe fn ObjectPtrLabel(ptr: *const c_void, length: GLsizei, label: *const GLchar) {
    unsafe {
        mem::transmute::<_, extern "system" fn(*const c_void, GLsizei, *const GLchar) -> ()>(
            *GL_API.get_unchecked(ObjectPtrLabelIdx as usize),
        )(ptr, length, label)
    }
}
#[inline]
pub unsafe fn ActiveShaderProgram(pipeline: GLuint, program: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(ActiveShaderProgramIdx as usize),
        )(pipeline, program)
    }
}
#[inline]
pub unsafe fn BindProgramPipeline(pipeline: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
            *GL_API.get_unchecked(BindProgramPipelineIdx as usize),
        )(pipeline)
    }
}
#[inline]
pub unsafe fn CreateProgramPipelines(n: GLsizei, pipelines: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(CreateProgramPipelinesIdx as usize),
        )(n, pipelines)
    }
}
#[inline]
pub unsafe fn NormalP3ui(type_: GLenum, coords: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(NormalP3uiIdx as usize),
        )(type_, coords)
    }
}
#[inline]
pub unsafe fn UseProgramStages(pipeline: GLuint, stages: GLbitfield, program: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLbitfield, GLuint) -> ()>(
            *GL_API.get_unchecked(UseProgramStagesIdx as usize),
        )(pipeline, stages, program)
    }
}
/// Fallbacks: VertexAttribL2dEXT
#[inline]
pub unsafe fn VertexAttribL2d(index: GLuint, x: GLdouble, y: GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble) -> ()>(
            *GL_API.get_unchecked(VertexAttribL2dIdx as usize),
        )(index, x, y)
    }
}
#[inline]
pub unsafe fn GetnHistogram(
    target: GLenum,
    reset: GLboolean,
    format: GLenum,
    type_: GLenum,
    bufSize: GLsizei,
    values: *mut c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLboolean, GLenum, GLenum, GLsizei, *mut c_void) -> (),
        >(*GL_API.get_unchecked(GetnHistogramIdx as usize))(
            target, reset, format, type_, bufSize, values,
        )
    }
}
/// Fallbacks: ScissorArrayvNV
#[inline]
pub unsafe fn ScissorArrayv(first: GLuint, count: GLsizei, v: *const GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLint) -> ()>(
            *GL_API.get_unchecked(ScissorArrayvIdx as usize),
        )(first, count, v)
    }
}
/// Fallbacks: VertexAttribDivisorANGLE, VertexAttribDivisorARB, VertexAttribDivisorEXT, VertexAttribDivisorNV
#[inline]
pub unsafe fn VertexAttribDivisor(index: GLuint, divisor: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttribDivisorIdx as usize),
        )(index, divisor)
    }
}
#[inline]
pub unsafe fn GetTexImage(
    target: GLenum,
    level: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: *mut c_void,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLenum, *mut c_void) -> ()>(
            *GL_API.get_unchecked(GetTexImageIdx as usize),
        )(target, level, format, type_, pixels)
    }
}
#[inline]
pub unsafe fn SamplerParameteri(sampler: GLuint, pname: GLenum, param: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>(
            *GL_API.get_unchecked(SamplerParameteriIdx as usize),
        )(sampler, pname, param)
    }
}
#[inline]
pub unsafe fn TextureBarrier() {
    unsafe {
        mem::transmute::<_, extern "system" fn() -> ()>(
            *GL_API.get_unchecked(TextureBarrierIdx as usize),
        )()
    }
}
#[inline]
pub unsafe fn TextureParameteri(texture: GLuint, pname: GLenum, param: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>(
            *GL_API.get_unchecked(TextureParameteriIdx as usize),
        )(texture, pname, param)
    }
}
/// Fallbacks: GetObjectLabelKHR
#[inline]
pub unsafe fn GetObjectLabel(
    identifier: GLenum,
    name: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    label: *mut GLchar,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> (),
        >(*GL_API.get_unchecked(GetObjectLabelIdx as usize))(
            identifier, name, bufSize, length, label,
        )
    }
}
#[inline]
pub unsafe fn ReadBuffer(src: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(
            *GL_API.get_unchecked(ReadBufferIdx as usize),
        )(src)
    }
}
/// Fallbacks: StencilOpSeparateATI
#[inline]
pub unsafe fn StencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLenum) -> ()>(
            *GL_API.get_unchecked(StencilOpSeparateIdx as usize),
        )(face, sfail, dpfail, dppass)
    }
}
/// Fallbacks: TexSubImage2DEXT
#[inline]
pub unsafe fn TexSubImage2D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLint,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLenum,
                GLenum,
                *const c_void,
            ) -> (),
        >(*GL_API.get_unchecked(TexSubImage2DIdx as usize))(
            target, level, xoffset, yoffset, width, height, format, type_, pixels,
        )
    }
}
/// Fallbacks: GetTransformFeedbackVaryingEXT
#[inline]
pub unsafe fn GetTransformFeedbackVarying(
    program: GLuint,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    size: *mut GLsizei,
    type_: *mut GLenum,
    name: *mut GLchar,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLuint,
                GLsizei,
                *mut GLsizei,
                *mut GLsizei,
                *mut GLenum,
                *mut GLchar,
            ) -> (),
        >(*GL_API.get_unchecked(GetTransformFeedbackVaryingIdx as usize))(
            program, index, bufSize, length, size, type_, name,
        )
    }
}
#[inline]
pub unsafe fn MapNamedBufferRange(
    buffer: GLuint,
    offset: GLintptr,
    length: GLsizeiptr,
    access: GLbitfield,
) -> *mut c_void {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLintptr, GLsizeiptr, GLbitfield) -> *mut c_void,
        >(*GL_API.get_unchecked(MapNamedBufferRangeIdx as usize))(
            buffer, offset, length, access
        )
    }
}
#[inline]
pub unsafe fn SamplerParameteriv(sampler: GLuint, pname: GLenum, param: *const GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>(
            *GL_API.get_unchecked(SamplerParameterivIdx as usize),
        )(sampler, pname, param)
    }
}
/// Fallbacks: ProgramUniform4fvEXT
#[inline]
pub unsafe fn ProgramUniform4fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(ProgramUniform4fvIdx as usize),
        )(program, location, count, value)
    }
}
#[inline]
pub unsafe fn UniformMatrix4x3dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(UniformMatrix4x3dvIdx as usize),
        )(location, count, transpose, value)
    }
}
/// Fallbacks: ScissorIndexedvNV
#[inline]
pub unsafe fn ScissorIndexedv(index: GLuint, v: *const GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(
            *GL_API.get_unchecked(ScissorIndexedvIdx as usize),
        )(index, v)
    }
}
#[inline]
pub unsafe fn BindImageTexture(
    unit: GLuint,
    texture: GLuint,
    level: GLint,
    layered: GLboolean,
    layer: GLint,
    access: GLenum,
    format: GLenum,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLuint, GLint, GLboolean, GLint, GLenum, GLenum) -> (),
        >(*GL_API.get_unchecked(BindImageTextureIdx as usize))(
            unit, texture, level, layered, layer, access, format,
        )
    }
}
/// Fallbacks: BlendColorEXT
#[inline]
pub unsafe fn BlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(
            *GL_API.get_unchecked(BlendColorIdx as usize),
        )(red, green, blue, alpha)
    }
}
/// Fallbacks: GetPointervEXT, GetPointervKHR
#[inline]
pub unsafe fn GetPointerv(pname: GLenum, params: *const *mut c_void) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *const *mut c_void) -> ()>(
            *GL_API.get_unchecked(GetPointervIdx as usize),
        )(pname, params)
    }
}
/// Fallbacks: ProgramUniform2uivEXT
#[inline]
pub unsafe fn ProgramUniform2uiv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(ProgramUniform2uivIdx as usize),
        )(program, location, count, value)
    }
}
/// Fallbacks: DrawElementsInstancedBaseVertexBaseInstanceEXT
#[inline]
pub unsafe fn DrawElementsInstancedBaseVertexBaseInstance(
    mode: GLenum,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
    instancecount: GLsizei,
    basevertex: GLint,
    baseinstance: GLuint,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLsizei,
                GLenum,
                *const c_void,
                GLsizei,
                GLint,
                GLuint,
            ) -> (),
        >(*GL_API.get_unchecked(DrawElementsInstancedBaseVertexBaseInstanceIdx as usize))(
            mode,
            count,
            type_,
            indices,
            instancecount,
            basevertex,
            baseinstance,
        )
    }
}
/// Fallbacks: GetInteger64vAPPLE
#[inline]
pub unsafe fn GetInteger64v(pname: GLenum, data: *mut GLint64) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *mut GLint64) -> ()>(
            *GL_API.get_unchecked(GetInteger64vIdx as usize),
        )(pname, data)
    }
}
/// Fallbacks: VertexAttribI2uiEXT
#[inline]
pub unsafe fn VertexAttribI2ui(index: GLuint, x: GLuint, y: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttribI2uiIdx as usize),
        )(index, x, y)
    }
}
/// Fallbacks: ProgramUniform1iEXT
#[inline]
pub unsafe fn ProgramUniform1i(program: GLuint, location: GLint, v0: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint) -> ()>(
            *GL_API.get_unchecked(ProgramUniform1iIdx as usize),
        )(program, location, v0)
    }
}
/// Fallbacks: GetVertexAttribivARB, GetVertexAttribivNV
#[inline]
pub unsafe fn GetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetVertexAttribivIdx as usize),
        )(index, pname, params)
    }
}
/// Fallbacks: ProgramUniform4iEXT
#[inline]
pub unsafe fn ProgramUniform4i(
    program: GLuint,
    location: GLint,
    v0: GLint,
    v1: GLint,
    v2: GLint,
    v3: GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint) -> ()>(
            *GL_API.get_unchecked(ProgramUniform4iIdx as usize),
        )(program, location, v0, v1, v2, v3)
    }
}
#[inline]
pub unsafe fn VertexArrayAttribBinding(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexArrayAttribBindingIdx as usize),
        )(vaobj, attribindex, bindingindex)
    }
}
#[inline]
pub unsafe fn GetFloatv(pname: GLenum, data: *mut GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *mut GLfloat) -> ()>(
            *GL_API.get_unchecked(GetFloatvIdx as usize),
        )(pname, data)
    }
}
#[inline]
pub unsafe fn DispatchComputeIndirect(indirect: GLintptr) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLintptr) -> ()>(
            *GL_API.get_unchecked(DispatchComputeIndirectIdx as usize),
        )(indirect)
    }
}
#[inline]
pub unsafe fn Enable(cap: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(
            *GL_API.get_unchecked(EnableIdx as usize),
        )(cap)
    }
}
/// Fallbacks: BindBufferRangeEXT, BindBufferRangeNV
#[inline]
pub unsafe fn BindBufferRange(
    target: GLenum,
    index: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint, GLintptr, GLsizeiptr) -> ()>(
            *GL_API.get_unchecked(BindBufferRangeIdx as usize),
        )(target, index, buffer, offset, size)
    }
}
/// Fallbacks: ShaderSourceARB
#[inline]
pub unsafe fn ShaderSource(
    shader: GLuint,
    count: GLsizei,
    string: *const *const GLchar,
    length: *const GLint,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLsizei, *const *const GLchar, *const GLint) -> (),
        >(*GL_API.get_unchecked(ShaderSourceIdx as usize))(shader, count, string, length)
    }
}
#[inline]
pub unsafe fn VertexArrayAttribIFormat(
    vaobj: GLuint,
    attribindex: GLuint,
    size: GLint,
    type_: GLenum,
    relativeoffset: GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLint, GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexArrayAttribIFormatIdx as usize),
        )(vaobj, attribindex, size, type_, relativeoffset)
    }
}
/// Fallbacks: VertexAttribI4ubvEXT
#[inline]
pub unsafe fn VertexAttribI4ubv(index: GLuint, v: *const GLubyte) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLubyte) -> ()>(
            *GL_API.get_unchecked(VertexAttribI4ubvIdx as usize),
        )(index, v)
    }
}
/// Fallbacks: VertexAttrib1sARB, VertexAttrib1sNV
#[inline]
pub unsafe fn VertexAttrib1s(index: GLuint, x: GLshort) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLshort) -> ()>(
            *GL_API.get_unchecked(VertexAttrib1sIdx as usize),
        )(index, x)
    }
}
/// Fallbacks: VertexAttribI2ivEXT
#[inline]
pub unsafe fn VertexAttribI2iv(index: GLuint, v: *const GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(
            *GL_API.get_unchecked(VertexAttribI2ivIdx as usize),
        )(index, v)
    }
}
/// Fallbacks: GetObjectPtrLabelKHR
#[inline]
pub unsafe fn GetObjectPtrLabel(
    ptr: *const c_void,
    bufSize: GLsizei,
    length: *mut GLsizei,
    label: *mut GLchar,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(*const c_void, GLsizei, *mut GLsizei, *mut GLchar) -> (),
        >(*GL_API.get_unchecked(GetObjectPtrLabelIdx as usize))(ptr, bufSize, length, label)
    }
}
#[inline]
pub unsafe fn Uniform2d(location: GLint, x: GLdouble, y: GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLdouble, GLdouble) -> ()>(
            *GL_API.get_unchecked(Uniform2dIdx as usize),
        )(location, x, y)
    }
}
/// Fallbacks: MultiDrawArraysIndirectAMD, MultiDrawArraysIndirectEXT
#[inline]
pub unsafe fn MultiDrawArraysIndirect(
    mode: GLenum,
    indirect: *const c_void,
    drawcount: GLsizei,
    stride: GLsizei,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *const c_void, GLsizei, GLsizei) -> ()>(
            *GL_API.get_unchecked(MultiDrawArraysIndirectIdx as usize),
        )(mode, indirect, drawcount, stride)
    }
}
/// Fallbacks: DrawArraysInstancedANGLE, DrawArraysInstancedARB, DrawArraysInstancedEXT, DrawArraysInstancedNV
#[inline]
pub unsafe fn DrawArraysInstanced(
    mode: GLenum,
    first: GLint,
    count: GLsizei,
    instancecount: GLsizei,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei, GLsizei) -> ()>(
            *GL_API.get_unchecked(DrawArraysInstancedIdx as usize),
        )(mode, first, count, instancecount)
    }
}
#[inline]
pub unsafe fn GetVertexArrayIndexed64iv(
    vaobj: GLuint,
    index: GLuint,
    pname: GLenum,
    param: *mut GLint64,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, *mut GLint64) -> ()>(
            *GL_API.get_unchecked(GetVertexArrayIndexed64ivIdx as usize),
        )(vaobj, index, pname, param)
    }
}
#[inline]
pub unsafe fn GetQueryIndexediv(target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetQueryIndexedivIdx as usize),
        )(target, index, pname, params)
    }
}
/// Fallbacks: GetFragDataLocationEXT
#[inline]
pub unsafe fn GetFragDataLocation(program: GLuint, name: *const GLchar) -> GLint {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(
            *GL_API.get_unchecked(GetFragDataLocationIdx as usize),
        )(program, name)
    }
}
#[inline]
pub unsafe fn DispatchCompute(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(DispatchComputeIdx as usize),
        )(num_groups_x, num_groups_y, num_groups_z)
    }
}
#[inline]
pub unsafe fn CopyTextureSubImage2D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) -> (),
        >(*GL_API.get_unchecked(CopyTextureSubImage2DIdx as usize))(
            texture, level, xoffset, yoffset, x, y, width, height,
        )
    }
}
#[inline]
pub unsafe fn ClearTexImage(
    texture: GLuint,
    level: GLint,
    format: GLenum,
    type_: GLenum,
    data: *const c_void,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLenum, *const c_void) -> ()>(
            *GL_API.get_unchecked(ClearTexImageIdx as usize),
        )(texture, level, format, type_, data)
    }
}
/// Fallbacks: VertexAttribI4uiEXT
#[inline]
pub unsafe fn VertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttribI4uiIdx as usize),
        )(index, x, y, z, w)
    }
}
/// Fallbacks: VertexAttrib4NsvARB
#[inline]
pub unsafe fn VertexAttrib4Nsv(index: GLuint, v: *const GLshort) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(
            *GL_API.get_unchecked(VertexAttrib4NsvIdx as usize),
        )(index, v)
    }
}
/// Fallbacks: VertexAttribI3iEXT
#[inline]
pub unsafe fn VertexAttribI3i(index: GLuint, x: GLint, y: GLint, z: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint) -> ()>(
            *GL_API.get_unchecked(VertexAttribI3iIdx as usize),
        )(index, x, y, z)
    }
}
#[inline]
pub unsafe fn VertexAttribP4uiv(
    index: GLuint,
    type_: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttribP4uivIdx as usize),
        )(index, type_, normalized, value)
    }
}
#[inline]
pub unsafe fn VertexAttribP2uiv(
    index: GLuint,
    type_: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttribP2uivIdx as usize),
        )(index, type_, normalized, value)
    }
}
/// Fallbacks: ProgramUniform2uiEXT
#[inline]
pub unsafe fn ProgramUniform2ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(ProgramUniform2uiIdx as usize),
        )(program, location, v0, v1)
    }
}
#[inline]
pub unsafe fn Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> ()>(
            *GL_API.get_unchecked(ViewportIdx as usize),
        )(x, y, width, height)
    }
}
#[inline]
pub unsafe fn GetError() -> GLenum {
    unsafe {
        mem::transmute::<_, extern "system" fn() -> GLenum>(
            *GL_API.get_unchecked(GetErrorIdx as usize),
        )()
    }
}
/// Fallbacks: DrawBuffersARB, DrawBuffersATI, DrawBuffersEXT
#[inline]
pub unsafe fn DrawBuffers(n: GLsizei, bufs: *const GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *const GLenum) -> ()>(
            *GL_API.get_unchecked(DrawBuffersIdx as usize),
        )(n, bufs)
    }
}
#[inline]
pub unsafe fn GetTextureLevelParameterfv(
    texture: GLuint,
    level: GLint,
    pname: GLenum,
    params: *mut GLfloat,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, *mut GLfloat) -> ()>(
            *GL_API.get_unchecked(GetTextureLevelParameterfvIdx as usize),
        )(texture, level, pname, params)
    }
}
/// Fallbacks: NamedBufferStorageEXT
#[inline]
pub unsafe fn NamedBufferStorage(
    buffer: GLuint,
    size: GLsizeiptr,
    data: *const c_void,
    flags: GLbitfield,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizeiptr, *const c_void, GLbitfield) -> ()>(
            *GL_API.get_unchecked(NamedBufferStorageIdx as usize),
        )(buffer, size, data, flags)
    }
}
/// Fallbacks: DrawRangeElementsBaseVertexEXT, DrawRangeElementsBaseVertexOES
#[inline]
pub unsafe fn DrawRangeElementsBaseVertex(
    mode: GLenum,
    start: GLuint,
    end: GLuint,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
    basevertex: GLint,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const c_void, GLint) -> (),
        >(*GL_API.get_unchecked(DrawRangeElementsBaseVertexIdx as usize))(
            mode, start, end, count, type_, indices, basevertex,
        )
    }
}
#[inline]
pub unsafe fn ProgramUniformMatrix2dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> (),
        >(*GL_API.get_unchecked(ProgramUniformMatrix2dvIdx as usize))(
            program, location, count, transpose, value,
        )
    }
}
/// Fallbacks: GetVertexAttribdvARB, GetVertexAttribdvNV
#[inline]
pub unsafe fn GetVertexAttribdv(index: GLuint, pname: GLenum, params: *mut GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLdouble) -> ()>(
            *GL_API.get_unchecked(GetVertexAttribdvIdx as usize),
        )(index, pname, params)
    }
}
#[inline]
pub unsafe fn GetnUniformdv(
    program: GLuint,
    location: GLint,
    bufSize: GLsizei,
    params: *mut GLdouble,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *mut GLdouble) -> ()>(
            *GL_API.get_unchecked(GetnUniformdvIdx as usize),
        )(program, location, bufSize, params)
    }
}
#[inline]
pub unsafe fn ClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLuint) -> ()>(
            *GL_API.get_unchecked(ClearBufferuivIdx as usize),
        )(buffer, drawbuffer, value)
    }
}
#[inline]
pub unsafe fn IsEnabled(cap: GLenum) -> GLboolean {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum) -> GLboolean>(
            *GL_API.get_unchecked(IsEnabledIdx as usize),
        )(cap)
    }
}
/// Fallbacks: DrawTransformFeedbackNV
#[inline]
pub unsafe fn DrawTransformFeedback(mode: GLenum, id: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(DrawTransformFeedbackIdx as usize),
        )(mode, id)
    }
}
/// Fallbacks: VertexAttribL2dvEXT
#[inline]
pub unsafe fn VertexAttribL2dv(index: GLuint, v: *const GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(VertexAttribL2dvIdx as usize),
        )(index, v)
    }
}
#[inline]
pub unsafe fn DepthFunc(func: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(
            *GL_API.get_unchecked(DepthFuncIdx as usize),
        )(func)
    }
}
/// Fallbacks: MultiDrawElementsEXT
#[inline]
pub unsafe fn MultiDrawElements(
    mode: GLenum,
    count: *const GLsizei,
    type_: GLenum,
    indices: *const *const c_void,
    drawcount: GLsizei,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, *const GLsizei, GLenum, *const *const c_void, GLsizei) -> (),
        >(*GL_API.get_unchecked(MultiDrawElementsIdx as usize))(
            mode, count, type_, indices, drawcount,
        )
    }
}
#[inline]
pub unsafe fn Flush() {
    unsafe {
        mem::transmute::<_, extern "system" fn() -> ()>(*GL_API.get_unchecked(FlushIdx as usize))()
    }
}
/// Fallbacks: GetUniformfvARB
#[inline]
pub unsafe fn GetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLfloat) -> ()>(
            *GL_API.get_unchecked(GetUniformfvIdx as usize),
        )(program, location, params)
    }
}
#[inline]
pub unsafe fn GetnPixelMapuiv(map: GLenum, bufSize: GLsizei, values: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(GetnPixelMapuivIdx as usize),
        )(map, bufSize, values)
    }
}
/// Fallbacks: GetQueryObjecti64vEXT
#[inline]
pub unsafe fn GetQueryObjecti64v(id: GLuint, pname: GLenum, params: *mut GLint64) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint64) -> ()>(
            *GL_API.get_unchecked(GetQueryObjecti64vIdx as usize),
        )(id, pname, params)
    }
}
/// Fallbacks: GenerateMipmapEXT
#[inline]
pub unsafe fn GenerateMipmap(target: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(
            *GL_API.get_unchecked(GenerateMipmapIdx as usize),
        )(target)
    }
}
#[inline]
pub unsafe fn DrawTransformFeedbackStream(mode: GLenum, id: GLuint, stream: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(DrawTransformFeedbackStreamIdx as usize),
        )(mode, id, stream)
    }
}
#[inline]
pub unsafe fn GetTexLevelParameterfv(
    target: GLenum,
    level: GLint,
    pname: GLenum,
    params: *mut GLfloat,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, *mut GLfloat) -> ()>(
            *GL_API.get_unchecked(GetTexLevelParameterfvIdx as usize),
        )(target, level, pname, params)
    }
}
/// Fallbacks: VertexAttrib4uivARB
#[inline]
pub unsafe fn VertexAttrib4uiv(index: GLuint, v: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttrib4uivIdx as usize),
        )(index, v)
    }
}
#[inline]
pub unsafe fn UniformMatrix4dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(UniformMatrix4dvIdx as usize),
        )(location, count, transpose, value)
    }
}
/// Fallbacks: VertexAttrib4dARB, VertexAttrib4dNV
#[inline]
pub unsafe fn VertexAttrib4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(
            *GL_API.get_unchecked(VertexAttrib4dIdx as usize),
        )(index, x, y, z, w)
    }
}
#[inline]
pub unsafe fn DepthMask(flag: GLboolean) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLboolean) -> ()>(
            *GL_API.get_unchecked(DepthMaskIdx as usize),
        )(flag)
    }
}
/// Fallbacks: VertexAttribL4dEXT
#[inline]
pub unsafe fn VertexAttribL4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(
            *GL_API.get_unchecked(VertexAttribL4dIdx as usize),
        )(index, x, y, z, w)
    }
}
/// Fallbacks: CopyTexSubImage1DEXT
#[inline]
pub unsafe fn CopyTexSubImage1D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei) -> ()>(
            *GL_API.get_unchecked(CopyTexSubImage1DIdx as usize),
        )(target, level, xoffset, x, y, width)
    }
}
/// Fallbacks: Uniform1uiEXT
#[inline]
pub unsafe fn Uniform1ui(location: GLint, v0: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLuint) -> ()>(
            *GL_API.get_unchecked(Uniform1uiIdx as usize),
        )(location, v0)
    }
}
/// Fallbacks: VertexAttrib4NubvARB, VertexAttrib4ubvNV
#[inline]
pub unsafe fn VertexAttrib4Nubv(index: GLuint, v: *const GLubyte) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLubyte) -> ()>(
            *GL_API.get_unchecked(VertexAttrib4NubvIdx as usize),
        )(index, v)
    }
}
#[inline]
pub unsafe fn UniformSubroutinesuiv(shadertype: GLenum, count: GLsizei, indices: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(UniformSubroutinesuivIdx as usize),
        )(shadertype, count, indices)
    }
}
#[inline]
pub unsafe fn Scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> ()>(
            *GL_API.get_unchecked(ScissorIdx as usize),
        )(x, y, width, height)
    }
}
#[inline]
pub unsafe fn TextureStorage3DMultisample(
    texture: GLuint,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    fixedsamplelocations: GLboolean,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean) -> (),
        >(*GL_API.get_unchecked(TextureStorage3DMultisampleIdx as usize))(
            texture,
            samples,
            internalformat,
            width,
            height,
            depth,
            fixedsamplelocations,
        )
    }
}
#[inline]
pub unsafe fn StencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint, GLuint) -> ()>(
            *GL_API.get_unchecked(StencilFuncSeparateIdx as usize),
        )(face, func, ref_, mask)
    }
}
#[inline]
pub unsafe fn TexCoordP3uiv(type_: GLenum, coords: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(
            *GL_API.get_unchecked(TexCoordP3uivIdx as usize),
        )(type_, coords)
    }
}
/// Fallbacks: ValidateProgramARB
#[inline]
pub unsafe fn ValidateProgram(program: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
            *GL_API.get_unchecked(ValidateProgramIdx as usize),
        )(program)
    }
}
#[inline]
pub unsafe fn InvalidateSubFramebuffer(
    target: GLenum,
    numAttachments: GLsizei,
    attachments: *const GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLsizei,
                *const GLenum,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
            ) -> (),
        >(*GL_API.get_unchecked(InvalidateSubFramebufferIdx as usize))(
            target,
            numAttachments,
            attachments,
            x,
            y,
            width,
            height,
        )
    }
}
/// Fallbacks: VertexAttrib3fvARB, VertexAttrib3fvNV
#[inline]
pub unsafe fn VertexAttrib3fv(index: GLuint, v: *const GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(VertexAttrib3fvIdx as usize),
        )(index, v)
    }
}
/// Fallbacks: DeleteVertexArraysAPPLE, DeleteVertexArraysOES
#[inline]
pub unsafe fn DeleteVertexArrays(n: GLsizei, arrays: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(DeleteVertexArraysIdx as usize),
        )(n, arrays)
    }
}
/// Fallbacks: VertexAttribI4uivEXT
#[inline]
pub unsafe fn VertexAttribI4uiv(index: GLuint, v: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttribI4uivIdx as usize),
        )(index, v)
    }
}
/// Fallbacks: VertexAttrib4svARB, VertexAttrib4svNV
#[inline]
pub unsafe fn VertexAttrib4sv(index: GLuint, v: *const GLshort) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(
            *GL_API.get_unchecked(VertexAttrib4svIdx as usize),
        )(index, v)
    }
}
#[inline]
pub unsafe fn SamplerParameterf(sampler: GLuint, pname: GLenum, param: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLfloat) -> ()>(
            *GL_API.get_unchecked(SamplerParameterfIdx as usize),
        )(sampler, pname, param)
    }
}
/// Fallbacks: VertexAttribI1ivEXT
#[inline]
pub unsafe fn VertexAttribI1iv(index: GLuint, v: *const GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(
            *GL_API.get_unchecked(VertexAttribI1ivIdx as usize),
        )(index, v)
    }
}
#[inline]
pub unsafe fn TexParameteriv(target: GLenum, pname: GLenum, params: *const GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(
            *GL_API.get_unchecked(TexParameterivIdx as usize),
        )(target, pname, params)
    }
}
/// Fallbacks: Uniform4iARB
#[inline]
pub unsafe fn Uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint, GLint) -> ()>(
            *GL_API.get_unchecked(Uniform4iIdx as usize),
        )(location, v0, v1, v2, v3)
    }
}
#[inline]
pub unsafe fn TexCoordP1ui(type_: GLenum, coords: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(TexCoordP1uiIdx as usize),
        )(type_, coords)
    }
}
/// Fallbacks: IsFramebufferEXT
#[inline]
pub unsafe fn IsFramebuffer(framebuffer: GLuint) -> GLboolean {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
            *GL_API.get_unchecked(IsFramebufferIdx as usize),
        )(framebuffer)
    }
}
#[inline]
pub unsafe fn IsTexture(texture: GLuint) -> GLboolean {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
            *GL_API.get_unchecked(IsTextureIdx as usize),
        )(texture)
    }
}
#[inline]
pub unsafe fn BlendFunc(sfactor: GLenum, dfactor: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(
            *GL_API.get_unchecked(BlendFuncIdx as usize),
        )(sfactor, dfactor)
    }
}
/// Fallbacks: ProgramUniform4uiEXT
#[inline]
pub unsafe fn ProgramUniform4ui(
    program: GLuint,
    location: GLint,
    v0: GLuint,
    v1: GLuint,
    v2: GLuint,
    v3: GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(ProgramUniform4uiIdx as usize),
        )(program, location, v0, v1, v2, v3)
    }
}
#[inline]
pub unsafe fn UniformMatrix2dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(UniformMatrix2dvIdx as usize),
        )(location, count, transpose, value)
    }
}
#[inline]
pub unsafe fn VertexArrayElementBuffer(vaobj: GLuint, buffer: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexArrayElementBufferIdx as usize),
        )(vaobj, buffer)
    }
}
#[inline]
pub unsafe fn GenProgramPipelines(n: GLsizei, pipelines: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(GenProgramPipelinesIdx as usize),
        )(n, pipelines)
    }
}
#[inline]
pub unsafe fn NamedFramebufferReadBuffer(framebuffer: GLuint, src: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>(
            *GL_API.get_unchecked(NamedFramebufferReadBufferIdx as usize),
        )(framebuffer, src)
    }
}
#[inline]
pub unsafe fn DrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const c_void) -> ()>(
            *GL_API.get_unchecked(DrawElementsIdx as usize),
        )(mode, count, type_, indices)
    }
}
#[inline]
pub unsafe fn TextureParameteriv(texture: GLuint, pname: GLenum, param: *const GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>(
            *GL_API.get_unchecked(TextureParameterivIdx as usize),
        )(texture, pname, param)
    }
}
#[inline]
pub unsafe fn StencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum) -> ()>(
            *GL_API.get_unchecked(StencilOpIdx as usize),
        )(fail, zfail, zpass)
    }
}
#[inline]
pub unsafe fn BindVertexBuffers(
    first: GLuint,
    count: GLsizei,
    buffers: *const GLuint,
    offsets: *const GLintptr,
    strides: *const GLsizei,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLsizei,
                *const GLuint,
                *const GLintptr,
                *const GLsizei,
            ) -> (),
        >(*GL_API.get_unchecked(BindVertexBuffersIdx as usize))(
            first, count, buffers, offsets, strides,
        )
    }
}
/// Fallbacks: PopDebugGroupKHR
#[inline]
pub unsafe fn PopDebugGroup() {
    unsafe {
        mem::transmute::<_, extern "system" fn() -> ()>(
            *GL_API.get_unchecked(PopDebugGroupIdx as usize),
        )()
    }
}
/// Fallbacks: Uniform2uiEXT
#[inline]
pub unsafe fn Uniform2ui(location: GLint, v0: GLuint, v1: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(Uniform2uiIdx as usize),
        )(location, v0, v1)
    }
}
#[inline]
pub unsafe fn SecondaryColorP3uiv(type_: GLenum, color: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(
            *GL_API.get_unchecked(SecondaryColorP3uivIdx as usize),
        )(type_, color)
    }
}
#[inline]
pub unsafe fn BindSampler(unit: GLuint, sampler: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(BindSamplerIdx as usize),
        )(unit, sampler)
    }
}
#[inline]
pub unsafe fn Uniform1dv(location: GLint, count: GLsizei, value: *const GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(Uniform1dvIdx as usize),
        )(location, count, value)
    }
}
/// Fallbacks: VertexAttrib3dARB, VertexAttrib3dNV
#[inline]
pub unsafe fn VertexAttrib3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble) -> ()>(
            *GL_API.get_unchecked(VertexAttrib3dIdx as usize),
        )(index, x, y, z)
    }
}
#[inline]
pub unsafe fn GetNamedBufferPointerv(buffer: GLuint, pname: GLenum, params: *const *mut c_void) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const *mut c_void) -> ()>(
            *GL_API.get_unchecked(GetNamedBufferPointervIdx as usize),
        )(buffer, pname, params)
    }
}
#[inline]
pub unsafe fn CreateSamplers(n: GLsizei, samplers: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(CreateSamplersIdx as usize),
        )(n, samplers)
    }
}
#[inline]
pub unsafe fn EndQueryIndexed(target: GLenum, index: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(EndQueryIndexedIdx as usize),
        )(target, index)
    }
}
#[inline]
pub unsafe fn ClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(ClearBufferfvIdx as usize),
        )(buffer, drawbuffer, value)
    }
}
/// Fallbacks: UniformMatrix4x2fvNV
#[inline]
pub unsafe fn UniformMatrix4x2fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(UniformMatrix4x2fvIdx as usize),
        )(location, count, transpose, value)
    }
}
#[inline]
pub unsafe fn StencilMask(mask: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
            *GL_API.get_unchecked(StencilMaskIdx as usize),
        )(mask)
    }
}
/// Fallbacks: UniformMatrix4fvARB
#[inline]
pub unsafe fn UniformMatrix4fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(UniformMatrix4fvIdx as usize),
        )(location, count, transpose, value)
    }
}
/// Fallbacks: PolygonModeNV
#[inline]
pub unsafe fn PolygonMode(face: GLenum, mode: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(
            *GL_API.get_unchecked(PolygonModeIdx as usize),
        )(face, mode)
    }
}
/// Fallbacks: CompressedTexSubImage3DARB, CompressedTexSubImage3DOES
#[inline]
pub unsafe fn CompressedTexSubImage3D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLint,
                GLint,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLsizei,
                GLenum,
                GLsizei,
                *const c_void,
            ) -> (),
        >(*GL_API.get_unchecked(CompressedTexSubImage3DIdx as usize))(
            target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data,
        )
    }
}
#[inline]
pub unsafe fn VertexAttribP4ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttribP4uiIdx as usize),
        )(index, type_, normalized, value)
    }
}
/// Fallbacks: VertexAttribIPointerEXT
#[inline]
pub unsafe fn VertexAttribIPointer(
    index: GLuint,
    size: GLint,
    type_: GLenum,
    stride: GLsizei,
    pointer: *const c_void,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const c_void) -> ()>(
            *GL_API.get_unchecked(VertexAttribIPointerIdx as usize),
        )(index, size, type_, stride, pointer)
    }
}
#[inline]
pub unsafe fn NamedFramebufferTextureLayer(
    framebuffer: GLuint,
    attachment: GLenum,
    texture: GLuint,
    level: GLint,
    layer: GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLint, GLint) -> ()>(
            *GL_API.get_unchecked(NamedFramebufferTextureLayerIdx as usize),
        )(framebuffer, attachment, texture, level, layer)
    }
}
/// Fallbacks: DeleteFramebuffersEXT
#[inline]
pub unsafe fn DeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(DeleteFramebuffersIdx as usize),
        )(n, framebuffers)
    }
}
#[inline]
pub unsafe fn Disable(cap: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(
            *GL_API.get_unchecked(DisableIdx as usize),
        )(cap)
    }
}
#[inline]
pub unsafe fn GetShaderInfoLog(
    shader: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    infoLog: *mut GLchar,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(
            *GL_API.get_unchecked(GetShaderInfoLogIdx as usize),
        )(shader, bufSize, length, infoLog)
    }
}
#[inline]
pub unsafe fn Uniform3d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLdouble, GLdouble, GLdouble) -> ()>(
            *GL_API.get_unchecked(Uniform3dIdx as usize),
        )(location, x, y, z)
    }
}
#[inline]
pub unsafe fn CopyTextureSubImage3D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
            ) -> (),
        >(*GL_API.get_unchecked(CopyTextureSubImage3DIdx as usize))(
            texture, level, xoffset, yoffset, zoffset, x, y, width, height,
        )
    }
}
#[inline]
pub unsafe fn InvalidateBufferData(buffer: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
            *GL_API.get_unchecked(InvalidateBufferDataIdx as usize),
        )(buffer)
    }
}
/// Fallbacks: EndConditionalRenderNV, EndConditionalRenderNVX
#[inline]
pub unsafe fn EndConditionalRender() {
    unsafe {
        mem::transmute::<_, extern "system" fn() -> ()>(
            *GL_API.get_unchecked(EndConditionalRenderIdx as usize),
        )()
    }
}
#[inline]
pub unsafe fn ReleaseShaderCompiler() {
    unsafe {
        mem::transmute::<_, extern "system" fn() -> ()>(
            *GL_API.get_unchecked(ReleaseShaderCompilerIdx as usize),
        )()
    }
}
/// Fallbacks: NamedBufferSubDataEXT
#[inline]
pub unsafe fn NamedBufferSubData(
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *const c_void,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLintptr, GLsizeiptr, *const c_void) -> ()>(
            *GL_API.get_unchecked(NamedBufferSubDataIdx as usize),
        )(buffer, offset, size, data)
    }
}
#[inline]
pub unsafe fn GetnPixelMapfv(map: GLenum, bufSize: GLsizei, values: *mut GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *mut GLfloat) -> ()>(
            *GL_API.get_unchecked(GetnPixelMapfvIdx as usize),
        )(map, bufSize, values)
    }
}
/// Fallbacks: UniformMatrix3x2fvNV
#[inline]
pub unsafe fn UniformMatrix3x2fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(UniformMatrix3x2fvIdx as usize),
        )(location, count, transpose, value)
    }
}
#[inline]
pub unsafe fn CopyNamedBufferSubData(
    readBuffer: GLuint,
    writeBuffer: GLuint,
    readOffset: GLintptr,
    writeOffset: GLintptr,
    size: GLsizeiptr,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLintptr, GLintptr, GLsizeiptr) -> ()>(
            *GL_API.get_unchecked(CopyNamedBufferSubDataIdx as usize),
        )(readBuffer, writeBuffer, readOffset, writeOffset, size)
    }
}
#[inline]
pub unsafe fn ProgramUniformMatrix4x2dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> (),
        >(*GL_API.get_unchecked(ProgramUniformMatrix4x2dvIdx as usize))(
            program, location, count, transpose, value,
        )
    }
}
#[inline]
pub unsafe fn GetDoublev(pname: GLenum, data: *mut GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *mut GLdouble) -> ()>(
            *GL_API.get_unchecked(GetDoublevIdx as usize),
        )(pname, data)
    }
}
/// Fallbacks: DisableVertexAttribArrayARB
#[inline]
pub unsafe fn DisableVertexAttribArray(index: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
            *GL_API.get_unchecked(DisableVertexAttribArrayIdx as usize),
        )(index)
    }
}
#[inline]
pub unsafe fn BindBuffersRange(
    target: GLenum,
    first: GLuint,
    count: GLsizei,
    buffers: *const GLuint,
    offsets: *const GLintptr,
    sizes: *const GLsizeiptr,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLuint,
                GLsizei,
                *const GLuint,
                *const GLintptr,
                *const GLsizeiptr,
            ) -> (),
        >(*GL_API.get_unchecked(BindBuffersRangeIdx as usize))(
            target, first, count, buffers, offsets, sizes,
        )
    }
}
/// Fallbacks: ProgramUniform4uivEXT
#[inline]
pub unsafe fn ProgramUniform4uiv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(ProgramUniform4uivIdx as usize),
        )(program, location, count, value)
    }
}
/// Fallbacks: ActiveTextureARB
#[inline]
pub unsafe fn ActiveTexture(texture: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(
            *GL_API.get_unchecked(ActiveTextureIdx as usize),
        )(texture)
    }
}
#[inline]
pub unsafe fn GetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetProgramivIdx as usize),
        )(program, pname, params)
    }
}
#[inline]
pub unsafe fn VertexAttribIFormat(
    attribindex: GLuint,
    size: GLint,
    type_: GLenum,
    relativeoffset: GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttribIFormatIdx as usize),
        )(attribindex, size, type_, relativeoffset)
    }
}
/// Fallbacks: CopyTexSubImage3DEXT, CopyTexSubImage3DOES
#[inline]
pub unsafe fn CopyTexSubImage3D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
            ) -> (),
        >(*GL_API.get_unchecked(CopyTexSubImage3DIdx as usize))(
            target, level, xoffset, yoffset, zoffset, x, y, width, height,
        )
    }
}
#[inline]
pub unsafe fn GetActiveAtomicCounterBufferiv(
    program: GLuint,
    bufferIndex: GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetActiveAtomicCounterBufferivIdx as usize),
        )(program, bufferIndex, pname, params)
    }
}
#[inline]
pub unsafe fn DrawElementsIndirect(mode: GLenum, type_: GLenum, indirect: *const c_void) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const c_void) -> ()>(
            *GL_API.get_unchecked(DrawElementsIndirectIdx as usize),
        )(mode, type_, indirect)
    }
}
/// Fallbacks: ViewportIndexedfNV
#[inline]
pub unsafe fn ViewportIndexedf(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(
            *GL_API.get_unchecked(ViewportIndexedfIdx as usize),
        )(index, x, y, w, h)
    }
}
/// Fallbacks: VertexAttrib4ubvARB
#[inline]
pub unsafe fn VertexAttrib4ubv(index: GLuint, v: *const GLubyte) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLubyte) -> ()>(
            *GL_API.get_unchecked(VertexAttrib4ubvIdx as usize),
        )(index, v)
    }
}
#[inline]
pub unsafe fn ClearBufferfi(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, GLfloat, GLint) -> ()>(
            *GL_API.get_unchecked(ClearBufferfiIdx as usize),
        )(buffer, drawbuffer, depth, stencil)
    }
}
/// Fallbacks: VertexAttribI1uivEXT
#[inline]
pub unsafe fn VertexAttribI1uiv(index: GLuint, v: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttribI1uivIdx as usize),
        )(index, v)
    }
}
/// Fallbacks: AttachObjectARB
#[inline]
pub unsafe fn AttachShader(program: GLuint, shader: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(AttachShaderIdx as usize),
        )(program, shader)
    }
}
/// Fallbacks: VertexAttrib3svARB, VertexAttrib3svNV
#[inline]
pub unsafe fn VertexAttrib3sv(index: GLuint, v: *const GLshort) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(
            *GL_API.get_unchecked(VertexAttrib3svIdx as usize),
        )(index, v)
    }
}
#[inline]
pub unsafe fn BindTransformFeedback(target: GLenum, id: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(BindTransformFeedbackIdx as usize),
        )(target, id)
    }
}
/// Fallbacks: ProgramUniform3iEXT
#[inline]
pub unsafe fn ProgramUniform3i(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint) -> ()>(
            *GL_API.get_unchecked(ProgramUniform3iIdx as usize),
        )(program, location, v0, v1, v2)
    }
}
#[inline]
pub unsafe fn ClearBufferiv(buffer: GLenum, drawbuffer: GLint, value: *const GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, *const GLint) -> ()>(
            *GL_API.get_unchecked(ClearBufferivIdx as usize),
        )(buffer, drawbuffer, value)
    }
}
/// Fallbacks: ProgramUniform3ivEXT
#[inline]
pub unsafe fn ProgramUniform3iv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> ()>(
            *GL_API.get_unchecked(ProgramUniform3ivIdx as usize),
        )(program, location, count, value)
    }
}
/// Fallbacks: GetCompressedTexImageARB
#[inline]
pub unsafe fn GetCompressedTexImage(target: GLenum, level: GLint, img: *mut c_void) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, *mut c_void) -> ()>(
            *GL_API.get_unchecked(GetCompressedTexImageIdx as usize),
        )(target, level, img)
    }
}
#[inline]
pub unsafe fn GetQueryBufferObjecti64v(
    id: GLuint,
    buffer: GLuint,
    pname: GLenum,
    offset: GLintptr,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, GLintptr) -> ()>(
            *GL_API.get_unchecked(GetQueryBufferObjecti64vIdx as usize),
        )(id, buffer, pname, offset)
    }
}
#[inline]
pub unsafe fn ProgramUniform4dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(ProgramUniform4dvIdx as usize),
        )(program, location, count, value)
    }
}
#[inline]
pub unsafe fn VertexArrayVertexBuffer(
    vaobj: GLuint,
    bindingindex: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    stride: GLsizei,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, GLintptr, GLsizei) -> ()>(
            *GL_API.get_unchecked(VertexArrayVertexBufferIdx as usize),
        )(vaobj, bindingindex, buffer, offset, stride)
    }
}
/// Fallbacks: Uniform2fARB
#[inline]
pub unsafe fn Uniform2f(location: GLint, v0: GLfloat, v1: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat) -> ()>(
            *GL_API.get_unchecked(Uniform2fIdx as usize),
        )(location, v0, v1)
    }
}
#[inline]
pub unsafe fn GetNamedRenderbufferParameteriv(
    renderbuffer: GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetNamedRenderbufferParameterivIdx as usize),
        )(renderbuffer, pname, params)
    }
}
/// Fallbacks: VertexAttrib2svARB, VertexAttrib2svNV
#[inline]
pub unsafe fn VertexAttrib2sv(index: GLuint, v: *const GLshort) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(
            *GL_API.get_unchecked(VertexAttrib2svIdx as usize),
        )(index, v)
    }
}
#[inline]
pub unsafe fn GetTextureSubImage(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    type_: GLenum,
    bufSize: GLsizei,
    pixels: *mut c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLsizei,
                GLenum,
                GLenum,
                GLsizei,
                *mut c_void,
            ) -> (),
        >(*GL_API.get_unchecked(GetTextureSubImageIdx as usize))(
            texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_,
            bufSize, pixels,
        )
    }
}
/// Fallbacks: VertexAttribI3uiEXT
#[inline]
pub unsafe fn VertexAttribI3ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttribI3uiIdx as usize),
        )(index, x, y, z)
    }
}
/// Fallbacks: GetQueryivARB
#[inline]
pub unsafe fn GetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetQueryivIdx as usize),
        )(target, pname, params)
    }
}
#[inline]
pub unsafe fn MemoryBarrierByRegion(barriers: GLbitfield) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(
            *GL_API.get_unchecked(MemoryBarrierByRegionIdx as usize),
        )(barriers)
    }
}
/// Fallbacks: ProgramUniformMatrix3fvEXT
#[inline]
pub unsafe fn ProgramUniformMatrix3fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
        >(*GL_API.get_unchecked(ProgramUniformMatrix3fvIdx as usize))(
            program, location, count, transpose, value,
        )
    }
}
/// Fallbacks: VertexAttrib1svARB, VertexAttrib1svNV
#[inline]
pub unsafe fn VertexAttrib1sv(index: GLuint, v: *const GLshort) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(
            *GL_API.get_unchecked(VertexAttrib1svIdx as usize),
        )(index, v)
    }
}
/// Fallbacks: BindTextureEXT
#[inline]
pub unsafe fn BindTexture(target: GLenum, texture: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(BindTextureIdx as usize),
        )(target, texture)
    }
}
#[inline]
pub unsafe fn TextureBufferRange(
    texture: GLuint,
    internalformat: GLenum,
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLintptr, GLsizeiptr) -> ()>(
            *GL_API.get_unchecked(TextureBufferRangeIdx as usize),
        )(texture, internalformat, buffer, offset, size)
    }
}
/// Fallbacks: Uniform4fARB
#[inline]
pub unsafe fn Uniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(
            *GL_API.get_unchecked(Uniform4fIdx as usize),
        )(location, v0, v1, v2, v3)
    }
}
#[inline]
pub unsafe fn ClearDepth(depth: GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLdouble) -> ()>(
            *GL_API.get_unchecked(ClearDepthIdx as usize),
        )(depth)
    }
}
#[inline]
pub unsafe fn FrontFace(mode: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(
            *GL_API.get_unchecked(FrontFaceIdx as usize),
        )(mode)
    }
}
#[inline]
pub unsafe fn GetTextureParameterfv(texture: GLuint, pname: GLenum, params: *mut GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLfloat) -> ()>(
            *GL_API.get_unchecked(GetTextureParameterfvIdx as usize),
        )(texture, pname, params)
    }
}
/// Fallbacks: MemoryBarrierEXT
#[inline]
pub unsafe fn MemoryBarrier(barriers: GLbitfield) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(
            *GL_API.get_unchecked(MemoryBarrierIdx as usize),
        )(barriers)
    }
}
/// Fallbacks: ViewportArrayvNV
#[inline]
pub unsafe fn ViewportArrayv(first: GLuint, count: GLsizei, v: *const GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(ViewportArrayvIdx as usize),
        )(first, count, v)
    }
}
#[inline]
pub unsafe fn BeginQueryIndexed(target: GLenum, index: GLuint, id: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(BeginQueryIndexedIdx as usize),
        )(target, index, id)
    }
}
#[inline]
pub unsafe fn PatchParameterfv(pname: GLenum, values: *const GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(PatchParameterfvIdx as usize),
        )(pname, values)
    }
}
#[inline]
pub unsafe fn BindTextures(first: GLuint, count: GLsizei, textures: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(BindTexturesIdx as usize),
        )(first, count, textures)
    }
}
#[inline]
pub unsafe fn GetProgramPipelineInfoLog(
    pipeline: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    infoLog: *mut GLchar,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(
            *GL_API.get_unchecked(GetProgramPipelineInfoLogIdx as usize),
        )(pipeline, bufSize, length, infoLog)
    }
}
/// Fallbacks: GetUniformuivEXT
#[inline]
pub unsafe fn GetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(GetUniformuivIdx as usize),
        )(program, location, params)
    }
}
/// Fallbacks: MultiDrawArraysEXT
#[inline]
pub unsafe fn MultiDrawArrays(
    mode: GLenum,
    first: *const GLint,
    count: *const GLsizei,
    drawcount: GLsizei,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *const GLint, *const GLsizei, GLsizei) -> ()>(
            *GL_API.get_unchecked(MultiDrawArraysIdx as usize),
        )(mode, first, count, drawcount)
    }
}
/// Fallbacks: ProgramUniform1uiEXT
#[inline]
pub unsafe fn ProgramUniform1ui(program: GLuint, location: GLint, v0: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint) -> ()>(
            *GL_API.get_unchecked(ProgramUniform1uiIdx as usize),
        )(program, location, v0)
    }
}
#[inline]
pub unsafe fn GetStringi(name: GLenum, index: GLuint) -> *const GLubyte {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> *const GLubyte>(
            *GL_API.get_unchecked(GetStringiIdx as usize),
        )(name, index)
    }
}
/// Fallbacks: GetShaderSourceARB
#[inline]
pub unsafe fn GetShaderSource(
    shader: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    source: *mut GLchar,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(
            *GL_API.get_unchecked(GetShaderSourceIdx as usize),
        )(shader, bufSize, length, source)
    }
}
/// Fallbacks: MapBufferRangeEXT
#[inline]
pub unsafe fn MapBufferRange(
    target: GLenum,
    offset: GLintptr,
    length: GLsizeiptr,
    access: GLbitfield,
) -> *mut c_void {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLintptr, GLsizeiptr, GLbitfield) -> *mut c_void,
        >(*GL_API.get_unchecked(MapBufferRangeIdx as usize))(target, offset, length, access)
    }
}
/// Fallbacks: VertexAttrib4NuivARB
#[inline]
pub unsafe fn VertexAttrib4Nuiv(index: GLuint, v: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttrib4NuivIdx as usize),
        )(index, v)
    }
}
#[inline]
pub unsafe fn ClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(
            *GL_API.get_unchecked(ClearColorIdx as usize),
        )(red, green, blue, alpha)
    }
}
/// Fallbacks: Uniform3uiEXT
#[inline]
pub unsafe fn Uniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(Uniform3uiIdx as usize),
        )(location, v0, v1, v2)
    }
}
/// Fallbacks: CreateProgramObjectARB
#[inline]
pub unsafe fn CreateProgram() -> GLuint {
    unsafe {
        mem::transmute::<_, extern "system" fn() -> GLuint>(
            *GL_API.get_unchecked(CreateProgramIdx as usize),
        )()
    }
}
#[inline]
pub unsafe fn IsProgramPipeline(pipeline: GLuint) -> GLboolean {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
            *GL_API.get_unchecked(IsProgramPipelineIdx as usize),
        )(pipeline)
    }
}
/// Fallbacks: Uniform3fARB
#[inline]
pub unsafe fn Uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat, GLfloat) -> ()>(
            *GL_API.get_unchecked(Uniform3fIdx as usize),
        )(location, v0, v1, v2)
    }
}
#[inline]
pub unsafe fn CreateQueries(target: GLenum, n: GLsizei, ids: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(CreateQueriesIdx as usize),
        )(target, n, ids)
    }
}
#[inline]
pub unsafe fn GetNamedBufferParameteriv(buffer: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetNamedBufferParameterivIdx as usize),
        )(buffer, pname, params)
    }
}
#[inline]
pub unsafe fn GetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetShaderivIdx as usize),
        )(shader, pname, params)
    }
}
#[inline]
pub unsafe fn PointSize(size: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(
            *GL_API.get_unchecked(PointSizeIdx as usize),
        )(size)
    }
}
#[inline]
pub unsafe fn DrawTransformFeedbackInstanced(mode: GLenum, id: GLuint, instancecount: GLsizei) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei) -> ()>(
            *GL_API.get_unchecked(DrawTransformFeedbackInstancedIdx as usize),
        )(mode, id, instancecount)
    }
}
/// Fallbacks: IsVertexArrayAPPLE, IsVertexArrayOES
#[inline]
pub unsafe fn IsVertexArray(array: GLuint) -> GLboolean {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
            *GL_API.get_unchecked(IsVertexArrayIdx as usize),
        )(array)
    }
}
#[inline]
pub unsafe fn GetCompressedTextureSubImage(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    bufSize: GLsizei,
    pixels: *mut c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLsizei,
                GLsizei,
                *mut c_void,
            ) -> (),
        >(*GL_API.get_unchecked(GetCompressedTextureSubImageIdx as usize))(
            texture, level, xoffset, yoffset, zoffset, width, height, depth, bufSize, pixels,
        )
    }
}
#[inline]
pub unsafe fn GetnPixelMapusv(map: GLenum, bufSize: GLsizei, values: *mut GLushort) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *mut GLushort) -> ()>(
            *GL_API.get_unchecked(GetnPixelMapusvIdx as usize),
        )(map, bufSize, values)
    }
}
/// Fallbacks: BeginTransformFeedbackEXT, BeginTransformFeedbackNV
#[inline]
pub unsafe fn BeginTransformFeedback(primitiveMode: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(
            *GL_API.get_unchecked(BeginTransformFeedbackIdx as usize),
        )(primitiveMode)
    }
}
/// Fallbacks: GetGraphicsResetStatusKHR
#[inline]
pub unsafe fn GetGraphicsResetStatus() -> GLenum {
    unsafe {
        mem::transmute::<_, extern "system" fn() -> GLenum>(
            *GL_API.get_unchecked(GetGraphicsResetStatusIdx as usize),
        )()
    }
}
#[inline]
pub unsafe fn Clear(mask: GLbitfield) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(
            *GL_API.get_unchecked(ClearIdx as usize),
        )(mask)
    }
}
#[inline]
pub unsafe fn ColorP3ui(type_: GLenum, color: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(ColorP3uiIdx as usize),
        )(type_, color)
    }
}
#[inline]
pub unsafe fn CreateBuffers(n: GLsizei, buffers: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(CreateBuffersIdx as usize),
        )(n, buffers)
    }
}
#[inline]
pub unsafe fn TexParameteri(target: GLenum, pname: GLenum, param: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(
            *GL_API.get_unchecked(TexParameteriIdx as usize),
        )(target, pname, param)
    }
}
/// Fallbacks: Uniform2iARB
#[inline]
pub unsafe fn Uniform2i(location: GLint, v0: GLint, v1: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLint, GLint) -> ()>(
            *GL_API.get_unchecked(Uniform2iIdx as usize),
        )(location, v0, v1)
    }
}
#[inline]
pub unsafe fn IsShader(shader: GLuint) -> GLboolean {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
            *GL_API.get_unchecked(IsShaderIdx as usize),
        )(shader)
    }
}
/// Fallbacks: GetBufferParameterivARB
#[inline]
pub unsafe fn GetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetBufferParameterivIdx as usize),
        )(target, pname, params)
    }
}
#[inline]
pub unsafe fn GetCompressedTextureImage(
    texture: GLuint,
    level: GLint,
    bufSize: GLsizei,
    pixels: *mut c_void,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *mut c_void) -> ()>(
            *GL_API.get_unchecked(GetCompressedTextureImageIdx as usize),
        )(texture, level, bufSize, pixels)
    }
}
/// Fallbacks: Uniform1fARB
#[inline]
pub unsafe fn Uniform1f(location: GLint, v0: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLfloat) -> ()>(
            *GL_API.get_unchecked(Uniform1fIdx as usize),
        )(location, v0)
    }
}
#[inline]
pub unsafe fn ClearNamedFramebufferuiv(
    framebuffer: GLuint,
    buffer: GLenum,
    drawbuffer: GLint,
    value: *const GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint, *const GLuint) -> ()>(
            *GL_API.get_unchecked(ClearNamedFramebufferuivIdx as usize),
        )(framebuffer, buffer, drawbuffer, value)
    }
}
/// Fallbacks: BlendEquationIndexedAMD, BlendEquationiARB, BlendEquationiEXT, BlendEquationiOES
#[inline]
pub unsafe fn BlendEquationi(buf: GLuint, mode: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>(
            *GL_API.get_unchecked(BlendEquationiIdx as usize),
        )(buf, mode)
    }
}
/// Fallbacks: CopyBufferSubDataNV
#[inline]
pub unsafe fn CopyBufferSubData(
    readTarget: GLenum,
    writeTarget: GLenum,
    readOffset: GLintptr,
    writeOffset: GLintptr,
    size: GLsizeiptr,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLintptr, GLintptr, GLsizeiptr) -> ()>(
            *GL_API.get_unchecked(CopyBufferSubDataIdx as usize),
        )(readTarget, writeTarget, readOffset, writeOffset, size)
    }
}
/// Fallbacks: PointParameterivNV
#[inline]
pub unsafe fn PointParameteriv(pname: GLenum, params: *const GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *const GLint) -> ()>(
            *GL_API.get_unchecked(PointParameterivIdx as usize),
        )(pname, params)
    }
}
/// Fallbacks: GetnUniformivKHR
#[inline]
pub unsafe fn GetnUniformiv(
    program: GLuint,
    location: GLint,
    bufSize: GLsizei,
    params: *mut GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetnUniformivIdx as usize),
        )(program, location, bufSize, params)
    }
}
#[inline]
pub unsafe fn GetActiveUniformsiv(
    program: GLuint,
    uniformCount: GLsizei,
    uniformIndices: *const GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLsizei, *const GLuint, GLenum, *mut GLint) -> (),
        >(*GL_API.get_unchecked(GetActiveUniformsivIdx as usize))(
            program,
            uniformCount,
            uniformIndices,
            pname,
            params,
        )
    }
}
/// Fallbacks: BindBufferARB
#[inline]
pub unsafe fn BindBuffer(target: GLenum, buffer: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(BindBufferIdx as usize),
        )(target, buffer)
    }
}
#[inline]
pub unsafe fn DeleteProgram(program: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
            *GL_API.get_unchecked(DeleteProgramIdx as usize),
        )(program)
    }
}
/// Fallbacks: VertexAttrib2dvARB, VertexAttrib2dvNV
#[inline]
pub unsafe fn VertexAttrib2dv(index: GLuint, v: *const GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(VertexAttrib2dvIdx as usize),
        )(index, v)
    }
}
/// Fallbacks: ProgramUniformMatrix2x3fvEXT
#[inline]
pub unsafe fn ProgramUniformMatrix2x3fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
        >(*GL_API.get_unchecked(ProgramUniformMatrix2x3fvIdx as usize))(
            program, location, count, transpose, value,
        )
    }
}
/// Fallbacks: BindAttribLocationARB
#[inline]
pub unsafe fn BindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, *const GLchar) -> ()>(
            *GL_API.get_unchecked(BindAttribLocationIdx as usize),
        )(program, index, name)
    }
}
/// Fallbacks: ProvokingVertexEXT
#[inline]
pub unsafe fn ProvokingVertex(mode: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(
            *GL_API.get_unchecked(ProvokingVertexIdx as usize),
        )(mode)
    }
}
#[inline]
pub unsafe fn GetTransformFeedbacki_v(
    xfb: GLuint,
    pname: GLenum,
    index: GLuint,
    param: *mut GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetTransformFeedbacki_vIdx as usize),
        )(xfb, pname, index, param)
    }
}
/// Fallbacks: ProgramUniform4fEXT
#[inline]
pub unsafe fn ProgramUniform4f(
    program: GLuint,
    location: GLint,
    v0: GLfloat,
    v1: GLfloat,
    v2: GLfloat,
    v3: GLfloat,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat, GLfloat) -> (),
        >(*GL_API.get_unchecked(ProgramUniform4fIdx as usize))(
            program, location, v0, v1, v2, v3
        )
    }
}
#[inline]
pub unsafe fn CompressedTextureSubImage1D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    width: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLint, GLsizei, GLenum, GLsizei, *const c_void) -> (),
        >(*GL_API.get_unchecked(CompressedTextureSubImage1DIdx as usize))(
            texture, level, xoffset, width, format, imageSize, data,
        )
    }
}
/// Fallbacks: TexStorage1DEXT
#[inline]
pub unsafe fn TexStorage1D(
    target: GLenum,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei) -> ()>(
            *GL_API.get_unchecked(TexStorage1DIdx as usize),
        )(target, levels, internalformat, width)
    }
}
/// Fallbacks: VertexAttribI4usvEXT
#[inline]
pub unsafe fn VertexAttribI4usv(index: GLuint, v: *const GLushort) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLushort) -> ()>(
            *GL_API.get_unchecked(VertexAttribI4usvIdx as usize),
        )(index, v)
    }
}
/// Fallbacks: IsRenderbufferEXT
#[inline]
pub unsafe fn IsRenderbuffer(renderbuffer: GLuint) -> GLboolean {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
            *GL_API.get_unchecked(IsRenderbufferIdx as usize),
        )(renderbuffer)
    }
}
#[inline]
pub unsafe fn VertexAttribP1ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttribP1uiIdx as usize),
        )(index, type_, normalized, value)
    }
}
/// Fallbacks: Uniform3uivEXT
#[inline]
pub unsafe fn Uniform3uiv(location: GLint, count: GLsizei, value: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(Uniform3uivIdx as usize),
        )(location, count, value)
    }
}
/// Fallbacks: ProgramUniformMatrix4x3fvEXT
#[inline]
pub unsafe fn ProgramUniformMatrix4x3fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
        >(*GL_API.get_unchecked(ProgramUniformMatrix4x3fvIdx as usize))(
            program, location, count, transpose, value,
        )
    }
}
#[inline]
pub unsafe fn GetUniformIndices(
    program: GLuint,
    uniformCount: GLsizei,
    uniformNames: *const *const GLchar,
    uniformIndices: *mut GLuint,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLsizei, *const *const GLchar, *mut GLuint) -> (),
        >(*GL_API.get_unchecked(GetUniformIndicesIdx as usize))(
            program,
            uniformCount,
            uniformNames,
            uniformIndices,
        )
    }
}
#[inline]
pub unsafe fn GenSamplers(count: GLsizei, samplers: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(GenSamplersIdx as usize),
        )(count, samplers)
    }
}
/// Fallbacks: ProgramUniformMatrix4fvEXT
#[inline]
pub unsafe fn ProgramUniformMatrix4fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
        >(*GL_API.get_unchecked(ProgramUniformMatrix4fvIdx as usize))(
            program, location, count, transpose, value,
        )
    }
}
#[inline]
pub unsafe fn VertexArrayBindingDivisor(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexArrayBindingDivisorIdx as usize),
        )(vaobj, bindingindex, divisor)
    }
}
#[inline]
pub unsafe fn VertexP2uiv(type_: GLenum, value: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(
            *GL_API.get_unchecked(VertexP2uivIdx as usize),
        )(type_, value)
    }
}
/// Fallbacks: VertexAttrib4sARB, VertexAttrib4sNV
#[inline]
pub unsafe fn VertexAttrib4s(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLshort, GLshort, GLshort, GLshort) -> ()>(
            *GL_API.get_unchecked(VertexAttrib4sIdx as usize),
        )(index, x, y, z, w)
    }
}
#[inline]
pub unsafe fn DeleteTextures(n: GLsizei, textures: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(DeleteTexturesIdx as usize),
        )(n, textures)
    }
}
#[inline]
pub unsafe fn BindImageTextures(first: GLuint, count: GLsizei, textures: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(BindImageTexturesIdx as usize),
        )(first, count, textures)
    }
}
/// Fallbacks: WaitSyncAPPLE
#[inline]
pub unsafe fn WaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsync, GLbitfield, GLuint64) -> ()>(
            *GL_API.get_unchecked(WaitSyncIdx as usize),
        )(sync, flags, timeout)
    }
}
/// Fallbacks: BindVertexArrayOES
#[inline]
pub unsafe fn BindVertexArray(array: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
            *GL_API.get_unchecked(BindVertexArrayIdx as usize),
        )(array)
    }
}
/// Fallbacks: GetActiveAttribARB
#[inline]
pub unsafe fn GetActiveAttrib(
    program: GLuint,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    size: *mut GLint,
    type_: *mut GLenum,
    name: *mut GLchar,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLuint,
                GLsizei,
                *mut GLsizei,
                *mut GLint,
                *mut GLenum,
                *mut GLchar,
            ) -> (),
        >(*GL_API.get_unchecked(GetActiveAttribIdx as usize))(
            program, index, bufSize, length, size, type_, name,
        )
    }
}
#[inline]
pub unsafe fn TextureStorage2DMultisample(
    texture: GLuint,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    fixedsamplelocations: GLboolean,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLboolean) -> (),
        >(*GL_API.get_unchecked(TextureStorage2DMultisampleIdx as usize))(
            texture,
            samples,
            internalformat,
            width,
            height,
            fixedsamplelocations,
        )
    }
}
/// Fallbacks: DebugMessageInsertARB, DebugMessageInsertKHR
#[inline]
pub unsafe fn DebugMessageInsert(
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    buf: *const GLchar,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar) -> (),
        >(*GL_API.get_unchecked(DebugMessageInsertIdx as usize))(
            source, type_, id, severity, length, buf,
        )
    }
}
/// Fallbacks: DeleteTransformFeedbacksNV
#[inline]
pub unsafe fn DeleteTransformFeedbacks(n: GLsizei, ids: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(DeleteTransformFeedbacksIdx as usize),
        )(n, ids)
    }
}
#[inline]
pub unsafe fn TextureSubImage1D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    width: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLint, GLsizei, GLenum, GLenum, *const c_void) -> (),
        >(*GL_API.get_unchecked(TextureSubImage1DIdx as usize))(
            texture, level, xoffset, width, format, type_, pixels,
        )
    }
}
/// Fallbacks: VertexAttribL1dvEXT
#[inline]
pub unsafe fn VertexAttribL1dv(index: GLuint, v: *const GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(VertexAttribL1dvIdx as usize),
        )(index, v)
    }
}
/// Fallbacks: VertexAttrib1fvARB, VertexAttrib1fvNV
#[inline]
pub unsafe fn VertexAttrib1fv(index: GLuint, v: *const GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(VertexAttrib1fvIdx as usize),
        )(index, v)
    }
}
#[inline]
pub unsafe fn GetBufferParameteri64v(target: GLenum, pname: GLenum, params: *mut GLint64) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint64) -> ()>(
            *GL_API.get_unchecked(GetBufferParameteri64vIdx as usize),
        )(target, pname, params)
    }
}
/// Fallbacks: DeleteRenderbuffersEXT
#[inline]
pub unsafe fn DeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(DeleteRenderbuffersIdx as usize),
        )(n, renderbuffers)
    }
}
/// Fallbacks: GetRenderbufferParameterivEXT
#[inline]
pub unsafe fn GetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetRenderbufferParameterivIdx as usize),
        )(target, pname, params)
    }
}
#[inline]
pub unsafe fn TextureParameterfv(texture: GLuint, pname: GLenum, param: *const GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(TextureParameterfvIdx as usize),
        )(texture, pname, param)
    }
}
/// Fallbacks: TexBufferRangeEXT, TexBufferRangeOES
#[inline]
pub unsafe fn TexBufferRange(
    target: GLenum,
    internalformat: GLenum,
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint, GLintptr, GLsizeiptr) -> ()>(
            *GL_API.get_unchecked(TexBufferRangeIdx as usize),
        )(target, internalformat, buffer, offset, size)
    }
}
#[inline]
pub unsafe fn NamedBufferData(
    buffer: GLuint,
    size: GLsizeiptr,
    data: *const c_void,
    usage: GLenum,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizeiptr, *const c_void, GLenum) -> ()>(
            *GL_API.get_unchecked(NamedBufferDataIdx as usize),
        )(buffer, size, data, usage)
    }
}
#[inline]
pub unsafe fn PixelStorei(pname: GLenum, param: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(
            *GL_API.get_unchecked(PixelStoreiIdx as usize),
        )(pname, param)
    }
}
#[inline]
pub unsafe fn GetActiveSubroutineUniformName(
    program: GLuint,
    shadertype: GLenum,
    index: GLuint,
    bufsize: GLsizei,
    length: *mut GLsizei,
    name: *mut GLchar,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> (),
        >(*GL_API.get_unchecked(GetActiveSubroutineUniformNameIdx as usize))(
            program, shadertype, index, bufsize, length, name,
        )
    }
}
/// Fallbacks: BlendEquationEXT
#[inline]
pub unsafe fn BlendEquation(mode: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(
            *GL_API.get_unchecked(BlendEquationIdx as usize),
        )(mode)
    }
}
/// Fallbacks: BufferDataARB
#[inline]
pub unsafe fn BufferData(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLsizeiptr, *const c_void, GLenum) -> ()>(
            *GL_API.get_unchecked(BufferDataIdx as usize),
        )(target, size, data, usage)
    }
}
/// Fallbacks: CompressedTexSubImage2DARB
#[inline]
pub unsafe fn CompressedTexSubImage2D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLint,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLenum,
                GLsizei,
                *const c_void,
            ) -> (),
        >(*GL_API.get_unchecked(CompressedTexSubImage2DIdx as usize))(
            target, level, xoffset, yoffset, width, height, format, imageSize, data,
        )
    }
}
/// Fallbacks: FramebufferTexture3DEXT, FramebufferTexture3DOES
#[inline]
pub unsafe fn FramebufferTexture3D(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
    zoffset: GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint, GLint) -> ()>(
            *GL_API.get_unchecked(FramebufferTexture3DIdx as usize),
        )(target, attachment, textarget, texture, level, zoffset)
    }
}
#[inline]
pub unsafe fn ProgramUniformMatrix4x3dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> (),
        >(*GL_API.get_unchecked(ProgramUniformMatrix4x3dvIdx as usize))(
            program, location, count, transpose, value,
        )
    }
}
#[inline]
pub unsafe fn GetnCompressedTexImage(
    target: GLenum,
    lod: GLint,
    bufSize: GLsizei,
    pixels: *mut c_void,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei, *mut c_void) -> ()>(
            *GL_API.get_unchecked(GetnCompressedTexImageIdx as usize),
        )(target, lod, bufSize, pixels)
    }
}
#[inline]
pub unsafe fn GetProgramStageiv(
    program: GLuint,
    shadertype: GLenum,
    pname: GLenum,
    values: *mut GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetProgramStageivIdx as usize),
        )(program, shadertype, pname, values)
    }
}
/// Fallbacks: ClampColorARB
#[inline]
pub unsafe fn ClampColor(target: GLenum, clamp: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(
            *GL_API.get_unchecked(ClampColorIdx as usize),
        )(target, clamp)
    }
}
#[inline]
pub unsafe fn ValidateProgramPipeline(pipeline: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
            *GL_API.get_unchecked(ValidateProgramPipelineIdx as usize),
        )(pipeline)
    }
}
/// Fallbacks: GetVertexAttribfvARB, GetVertexAttribfvNV
#[inline]
pub unsafe fn GetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLfloat) -> ()>(
            *GL_API.get_unchecked(GetVertexAttribfvIdx as usize),
        )(index, pname, params)
    }
}
#[inline]
pub unsafe fn ProgramUniformMatrix2x4dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> (),
        >(*GL_API.get_unchecked(ProgramUniformMatrix2x4dvIdx as usize))(
            program, location, count, transpose, value,
        )
    }
}
/// Fallbacks: UniformMatrix4x3fvNV
#[inline]
pub unsafe fn UniformMatrix4x3fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(UniformMatrix4x3fvIdx as usize),
        )(location, count, transpose, value)
    }
}
#[inline]
pub unsafe fn MultiTexCoordP2uiv(texture: GLenum, type_: GLenum, coords: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLuint) -> ()>(
            *GL_API.get_unchecked(MultiTexCoordP2uivIdx as usize),
        )(texture, type_, coords)
    }
}
#[inline]
pub unsafe fn DeleteShader(shader: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
            *GL_API.get_unchecked(DeleteShaderIdx as usize),
        )(shader)
    }
}
#[inline]
pub unsafe fn NamedFramebufferRenderbuffer(
    framebuffer: GLuint,
    attachment: GLenum,
    renderbuffertarget: GLenum,
    renderbuffer: GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(NamedFramebufferRenderbufferIdx as usize),
        )(framebuffer, attachment, renderbuffertarget, renderbuffer)
    }
}
/// Fallbacks: GetAttribLocationARB
#[inline]
pub unsafe fn GetAttribLocation(program: GLuint, name: *const GLchar) -> GLint {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(
            *GL_API.get_unchecked(GetAttribLocationIdx as usize),
        )(program, name)
    }
}
#[inline]
pub unsafe fn GetInteger64i_v(target: GLenum, index: GLuint, data: *mut GLint64) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLint64) -> ()>(
            *GL_API.get_unchecked(GetInteger64i_vIdx as usize),
        )(target, index, data)
    }
}
/// Fallbacks: CopyTexImage1DEXT
#[inline]
pub unsafe fn CopyTexImage1D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    border: GLint,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLint) -> (),
        >(*GL_API.get_unchecked(CopyTexImage1DIdx as usize))(
            target,
            level,
            internalformat,
            x,
            y,
            width,
            border,
        )
    }
}
/// Fallbacks: VertexAttrib2fARB, VertexAttrib2fNV
#[inline]
pub unsafe fn VertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat) -> ()>(
            *GL_API.get_unchecked(VertexAttrib2fIdx as usize),
        )(index, x, y)
    }
}
/// Fallbacks: VertexAttribI4ivEXT
#[inline]
pub unsafe fn VertexAttribI4iv(index: GLuint, v: *const GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(
            *GL_API.get_unchecked(VertexAttribI4ivIdx as usize),
        )(index, v)
    }
}
/// Fallbacks: ClearDepthfOES
#[inline]
pub unsafe fn ClearDepthf(d: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(
            *GL_API.get_unchecked(ClearDepthfIdx as usize),
        )(d)
    }
}
#[inline]
pub unsafe fn UniformMatrix2x3dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(UniformMatrix2x3dvIdx as usize),
        )(location, count, transpose, value)
    }
}
#[inline]
pub unsafe fn GetTexLevelParameteriv(
    target: GLenum,
    level: GLint,
    pname: GLenum,
    params: *mut GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetTexLevelParameterivIdx as usize),
        )(target, level, pname, params)
    }
}
/// Fallbacks: ReadnPixelsARB, ReadnPixelsEXT, ReadnPixelsKHR
#[inline]
pub unsafe fn ReadnPixels(
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    type_: GLenum,
    bufSize: GLsizei,
    data: *mut c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLenum,
                GLenum,
                GLsizei,
                *mut c_void,
            ) -> (),
        >(*GL_API.get_unchecked(ReadnPixelsIdx as usize))(
            x, y, width, height, format, type_, bufSize, data,
        )
    }
}
/// Fallbacks: LinkProgramARB
#[inline]
pub unsafe fn LinkProgram(program: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
            *GL_API.get_unchecked(LinkProgramIdx as usize),
        )(program)
    }
}
#[inline]
pub unsafe fn EnableVertexArrayAttrib(vaobj: GLuint, index: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(EnableVertexArrayAttribIdx as usize),
        )(vaobj, index)
    }
}
/// Fallbacks: VertexAttribLPointerEXT
#[inline]
pub unsafe fn VertexAttribLPointer(
    index: GLuint,
    size: GLint,
    type_: GLenum,
    stride: GLsizei,
    pointer: *const c_void,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const c_void) -> ()>(
            *GL_API.get_unchecked(VertexAttribLPointerIdx as usize),
        )(index, size, type_, stride, pointer)
    }
}
/// Fallbacks: TextureViewEXT, TextureViewOES
#[inline]
pub unsafe fn TextureView(
    texture: GLuint,
    target: GLenum,
    origtexture: GLuint,
    internalformat: GLenum,
    minlevel: GLuint,
    numlevels: GLuint,
    minlayer: GLuint,
    numlayers: GLuint,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLenum,
                GLuint,
                GLenum,
                GLuint,
                GLuint,
                GLuint,
                GLuint,
            ) -> (),
        >(*GL_API.get_unchecked(TextureViewIdx as usize))(
            texture,
            target,
            origtexture,
            internalformat,
            minlevel,
            numlevels,
            minlayer,
            numlayers,
        )
    }
}
#[inline]
pub unsafe fn GetActiveSubroutineUniformiv(
    program: GLuint,
    shadertype: GLenum,
    index: GLuint,
    pname: GLenum,
    values: *mut GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetActiveSubroutineUniformivIdx as usize),
        )(program, shadertype, index, pname, values)
    }
}
#[inline]
pub unsafe fn GetQueryBufferObjectui64v(
    id: GLuint,
    buffer: GLuint,
    pname: GLenum,
    offset: GLintptr,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, GLintptr) -> ()>(
            *GL_API.get_unchecked(GetQueryBufferObjectui64vIdx as usize),
        )(id, buffer, pname, offset)
    }
}
/// Fallbacks: CompileShaderARB
#[inline]
pub unsafe fn CompileShader(shader: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
            *GL_API.get_unchecked(CompileShaderIdx as usize),
        )(shader)
    }
}
/// Fallbacks: Uniform2fvARB
#[inline]
pub unsafe fn Uniform2fv(location: GLint, count: GLsizei, value: *const GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(Uniform2fvIdx as usize),
        )(location, count, value)
    }
}
/// Fallbacks: TexSubImage3DEXT, TexSubImage3DOES
#[inline]
pub unsafe fn TexSubImage3D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLint,
                GLint,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLsizei,
                GLenum,
                GLenum,
                *const c_void,
            ) -> (),
        >(*GL_API.get_unchecked(TexSubImage3DIdx as usize))(
            target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels,
        )
    }
}
#[inline]
pub unsafe fn TexImage2DMultisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    fixedsamplelocations: GLboolean,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean) -> (),
        >(*GL_API.get_unchecked(TexImage2DMultisampleIdx as usize))(
            target,
            samples,
            internalformat,
            width,
            height,
            fixedsamplelocations,
        )
    }
}
#[inline]
pub unsafe fn Uniform4d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(
            *GL_API.get_unchecked(Uniform4dIdx as usize),
        )(location, x, y, z, w)
    }
}
#[inline]
pub unsafe fn GetTransformFeedbacki64_v(
    xfb: GLuint,
    pname: GLenum,
    index: GLuint,
    param: *mut GLint64,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, *mut GLint64) -> ()>(
            *GL_API.get_unchecked(GetTransformFeedbacki64_vIdx as usize),
        )(xfb, pname, index, param)
    }
}
/// Fallbacks: ProgramUniformMatrix3x2fvEXT
#[inline]
pub unsafe fn ProgramUniformMatrix3x2fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
        >(*GL_API.get_unchecked(ProgramUniformMatrix3x2fvIdx as usize))(
            program, location, count, transpose, value,
        )
    }
}
/// Fallbacks: ProgramUniformMatrix2fvEXT
#[inline]
pub unsafe fn ProgramUniformMatrix2fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
        >(*GL_API.get_unchecked(ProgramUniformMatrix2fvIdx as usize))(
            program, location, count, transpose, value,
        )
    }
}
#[inline]
pub unsafe fn CreateVertexArrays(n: GLsizei, arrays: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(CreateVertexArraysIdx as usize),
        )(n, arrays)
    }
}
/// Fallbacks: BindBufferBaseEXT, BindBufferBaseNV
#[inline]
pub unsafe fn BindBufferBase(target: GLenum, index: GLuint, buffer: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(BindBufferBaseIdx as usize),
        )(target, index, buffer)
    }
}
#[inline]
pub unsafe fn GetSamplerParameteriv(sampler: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetSamplerParameterivIdx as usize),
        )(sampler, pname, params)
    }
}
#[inline]
pub unsafe fn ReadPixels(
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *mut c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *mut c_void) -> (),
        >(*GL_API.get_unchecked(ReadPixelsIdx as usize))(
            x, y, width, height, format, type_, pixels
        )
    }
}
#[inline]
pub unsafe fn VertexAttribLFormat(
    attribindex: GLuint,
    size: GLint,
    type_: GLenum,
    relativeoffset: GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttribLFormatIdx as usize),
        )(attribindex, size, type_, relativeoffset)
    }
}
#[inline]
pub unsafe fn GetQueryBufferObjectuiv(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, GLintptr) -> ()>(
            *GL_API.get_unchecked(GetQueryBufferObjectuivIdx as usize),
        )(id, buffer, pname, offset)
    }
}
/// Fallbacks: FramebufferTextureARB, FramebufferTextureEXT, FramebufferTextureOES
#[inline]
pub unsafe fn FramebufferTexture(
    target: GLenum,
    attachment: GLenum,
    texture: GLuint,
    level: GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint, GLint) -> ()>(
            *GL_API.get_unchecked(FramebufferTextureIdx as usize),
        )(target, attachment, texture, level)
    }
}
#[inline]
pub unsafe fn TexParameterf(target: GLenum, pname: GLenum, param: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLfloat) -> ()>(
            *GL_API.get_unchecked(TexParameterfIdx as usize),
        )(target, pname, param)
    }
}
#[inline]
pub unsafe fn FramebufferParameteri(target: GLenum, pname: GLenum, param: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(
            *GL_API.get_unchecked(FramebufferParameteriIdx as usize),
        )(target, pname, param)
    }
}
#[inline]
pub unsafe fn TextureParameterIiv(texture: GLuint, pname: GLenum, params: *const GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>(
            *GL_API.get_unchecked(TextureParameterIivIdx as usize),
        )(texture, pname, params)
    }
}
#[inline]
pub unsafe fn BindBuffersBase(
    target: GLenum,
    first: GLuint,
    count: GLsizei,
    buffers: *const GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(BindBuffersBaseIdx as usize),
        )(target, first, count, buffers)
    }
}
/// Fallbacks: TexStorage3DMultisampleOES
#[inline]
pub unsafe fn TexStorage3DMultisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    fixedsamplelocations: GLboolean,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean) -> (),
        >(*GL_API.get_unchecked(TexStorage3DMultisampleIdx as usize))(
            target,
            samples,
            internalformat,
            width,
            height,
            depth,
            fixedsamplelocations,
        )
    }
}
/// Fallbacks: VertexAttribI4iEXT
#[inline]
pub unsafe fn VertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint) -> ()>(
            *GL_API.get_unchecked(VertexAttribI4iIdx as usize),
        )(index, x, y, z, w)
    }
}
/// Fallbacks: DrawRangeElementsEXT
#[inline]
pub unsafe fn DrawRangeElements(
    mode: GLenum,
    start: GLuint,
    end: GLuint,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const c_void) -> (),
        >(*GL_API.get_unchecked(DrawRangeElementsIdx as usize))(
            mode, start, end, count, type_, indices,
        )
    }
}
/// Fallbacks: TexImage3DEXT, TexImage3DOES
#[inline]
pub unsafe fn TexImage3D(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLsizei,
                GLint,
                GLenum,
                GLenum,
                *const c_void,
            ) -> (),
        >(*GL_API.get_unchecked(TexImage3DIdx as usize))(
            target,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            format,
            type_,
            pixels,
        )
    }
}
#[inline]
pub unsafe fn TextureStorage2D(
    texture: GLuint,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei) -> ()>(
            *GL_API.get_unchecked(TextureStorage2DIdx as usize),
        )(texture, levels, internalformat, width, height)
    }
}
#[inline]
pub unsafe fn TransformFeedbackBufferRange(
    xfb: GLuint,
    index: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, GLintptr, GLsizeiptr) -> ()>(
            *GL_API.get_unchecked(TransformFeedbackBufferRangeIdx as usize),
        )(xfb, index, buffer, offset, size)
    }
}
#[inline]
pub unsafe fn VertexP4ui(type_: GLenum, value: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexP4uiIdx as usize),
        )(type_, value)
    }
}
/// Fallbacks: BlendFuncSeparateEXT, BlendFuncSeparateINGR
#[inline]
pub unsafe fn BlendFuncSeparate(
    sfactorRGB: GLenum,
    dfactorRGB: GLenum,
    sfactorAlpha: GLenum,
    dfactorAlpha: GLenum,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLenum) -> ()>(
            *GL_API.get_unchecked(BlendFuncSeparateIdx as usize),
        )(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha)
    }
}
/// Fallbacks: Uniform4fvARB
#[inline]
pub unsafe fn Uniform4fv(location: GLint, count: GLsizei, value: *const GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(Uniform4fvIdx as usize),
        )(location, count, value)
    }
}
#[inline]
pub unsafe fn CreateShaderProgramv(
    type_: GLenum,
    count: GLsizei,
    strings: *const *const GLchar,
) -> GLuint {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const *const GLchar) -> GLuint>(
            *GL_API.get_unchecked(CreateShaderProgramvIdx as usize),
        )(type_, count, strings)
    }
}
#[inline]
pub unsafe fn BindVertexBuffer(
    bindingindex: GLuint,
    buffer: GLuint,
    offset: GLintptr,
    stride: GLsizei,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLintptr, GLsizei) -> ()>(
            *GL_API.get_unchecked(BindVertexBufferIdx as usize),
        )(bindingindex, buffer, offset, stride)
    }
}
#[inline]
pub unsafe fn TexStorage2DMultisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    fixedsamplelocations: GLboolean,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean) -> (),
        >(*GL_API.get_unchecked(TexStorage2DMultisampleIdx as usize))(
            target,
            samples,
            internalformat,
            width,
            height,
            fixedsamplelocations,
        )
    }
}
#[inline]
pub unsafe fn ShaderStorageBlockBinding(
    program: GLuint,
    storageBlockIndex: GLuint,
    storageBlockBinding: GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(ShaderStorageBlockBindingIdx as usize),
        )(program, storageBlockIndex, storageBlockBinding)
    }
}
#[inline]
pub unsafe fn NamedRenderbufferStorageMultisample(
    renderbuffer: GLuint,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei) -> ()>(
            *GL_API.get_unchecked(NamedRenderbufferStorageMultisampleIdx as usize),
        )(renderbuffer, samples, internalformat, width, height)
    }
}
#[inline]
pub unsafe fn GetProgramResourceiv(
    program: GLuint,
    programInterface: GLenum,
    index: GLuint,
    propCount: GLsizei,
    props: *const GLenum,
    bufSize: GLsizei,
    length: *mut GLsizei,
    params: *mut GLint,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLenum,
                GLuint,
                GLsizei,
                *const GLenum,
                GLsizei,
                *mut GLsizei,
                *mut GLint,
            ) -> (),
        >(*GL_API.get_unchecked(GetProgramResourceivIdx as usize))(
            program,
            programInterface,
            index,
            propCount,
            props,
            bufSize,
            length,
            params,
        )
    }
}
/// Fallbacks: EnableVertexAttribArrayARB
#[inline]
pub unsafe fn EnableVertexAttribArray(index: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
            *GL_API.get_unchecked(EnableVertexAttribArrayIdx as usize),
        )(index)
    }
}
#[inline]
pub unsafe fn TexCoordP2ui(type_: GLenum, coords: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(TexCoordP2uiIdx as usize),
        )(type_, coords)
    }
}
/// Fallbacks: TexStorage2DEXT
#[inline]
pub unsafe fn TexStorage2D(
    target: GLenum,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei) -> ()>(
            *GL_API.get_unchecked(TexStorage2DIdx as usize),
        )(target, levels, internalformat, width, height)
    }
}
/// Fallbacks: VertexAttrib4NivARB
#[inline]
pub unsafe fn VertexAttrib4Niv(index: GLuint, v: *const GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(
            *GL_API.get_unchecked(VertexAttrib4NivIdx as usize),
        )(index, v)
    }
}
#[inline]
pub unsafe fn VertexArrayVertexBuffers(
    vaobj: GLuint,
    first: GLuint,
    count: GLsizei,
    buffers: *const GLuint,
    offsets: *const GLintptr,
    strides: *const GLsizei,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLuint,
                GLsizei,
                *const GLuint,
                *const GLintptr,
                *const GLsizei,
            ) -> (),
        >(*GL_API.get_unchecked(VertexArrayVertexBuffersIdx as usize))(
            vaobj, first, count, buffers, offsets, strides,
        )
    }
}
/// Fallbacks: ProgramUniform2ivEXT
#[inline]
pub unsafe fn ProgramUniform2iv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> ()>(
            *GL_API.get_unchecked(ProgramUniform2ivIdx as usize),
        )(program, location, count, value)
    }
}
/// Fallbacks: UniformMatrix2fvARB
#[inline]
pub unsafe fn UniformMatrix2fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(UniformMatrix2fvIdx as usize),
        )(location, count, transpose, value)
    }
}
#[inline]
pub unsafe fn GetnMinmax(
    target: GLenum,
    reset: GLboolean,
    format: GLenum,
    type_: GLenum,
    bufSize: GLsizei,
    values: *mut c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLboolean, GLenum, GLenum, GLsizei, *mut c_void) -> (),
        >(*GL_API.get_unchecked(GetnMinmaxIdx as usize))(
            target, reset, format, type_, bufSize, values,
        )
    }
}
/// Fallbacks: UniformMatrix2x4fvNV
#[inline]
pub unsafe fn UniformMatrix2x4fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(UniformMatrix2x4fvIdx as usize),
        )(location, count, transpose, value)
    }
}
#[inline]
pub unsafe fn Finish() {
    unsafe {
        mem::transmute::<_, extern "system" fn() -> ()>(*GL_API.get_unchecked(FinishIdx as usize))()
    }
}
/// Fallbacks: MultiDrawElementsIndirectAMD, MultiDrawElementsIndirectEXT
#[inline]
pub unsafe fn MultiDrawElementsIndirect(
    mode: GLenum,
    type_: GLenum,
    indirect: *const c_void,
    drawcount: GLsizei,
    stride: GLsizei,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const c_void, GLsizei, GLsizei) -> ()>(
            *GL_API.get_unchecked(MultiDrawElementsIndirectIdx as usize),
        )(mode, type_, indirect, drawcount, stride)
    }
}
/// Fallbacks: DebugMessageCallbackARB, DebugMessageCallbackKHR
#[inline]
pub unsafe fn DebugMessageCallback(callback: GLDEBUGPROC, userParam: *const c_void) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLDEBUGPROC, *const c_void) -> ()>(
            *GL_API.get_unchecked(DebugMessageCallbackIdx as usize),
        )(callback, userParam)
    }
}
/// Fallbacks: GetnUniformfvKHR
#[inline]
pub unsafe fn GetnUniformfv(
    program: GLuint,
    location: GLint,
    bufSize: GLsizei,
    params: *mut GLfloat,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *mut GLfloat) -> ()>(
            *GL_API.get_unchecked(GetnUniformfvIdx as usize),
        )(program, location, bufSize, params)
    }
}
/// Fallbacks: SamplerParameterIuivEXT, SamplerParameterIuivOES
#[inline]
pub unsafe fn SamplerParameterIuiv(sampler: GLuint, pname: GLenum, param: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLuint) -> ()>(
            *GL_API.get_unchecked(SamplerParameterIuivIdx as usize),
        )(sampler, pname, param)
    }
}
/// Fallbacks: CopyTexImage2DEXT
#[inline]
pub unsafe fn CopyTexImage2D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLsizei, GLint) -> (),
        >(*GL_API.get_unchecked(CopyTexImage2DIdx as usize))(
            target,
            level,
            internalformat,
            x,
            y,
            width,
            height,
            border,
        )
    }
}
#[inline]
pub unsafe fn UniformMatrix2x4dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(UniformMatrix2x4dvIdx as usize),
        )(location, count, transpose, value)
    }
}
/// Fallbacks: FramebufferTexture2DEXT
#[inline]
pub unsafe fn FramebufferTexture2D(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint) -> ()>(
            *GL_API.get_unchecked(FramebufferTexture2DIdx as usize),
        )(target, attachment, textarget, texture, level)
    }
}
#[inline]
pub unsafe fn VertexAttribFormat(
    attribindex: GLuint,
    size: GLint,
    type_: GLenum,
    normalized: GLboolean,
    relativeoffset: GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttribFormatIdx as usize),
        )(attribindex, size, type_, normalized, relativeoffset)
    }
}
#[inline]
pub unsafe fn ClearNamedBufferData(
    buffer: GLuint,
    internalformat: GLenum,
    format: GLenum,
    type_: GLenum,
    data: *const c_void,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, GLenum, *const c_void) -> ()>(
            *GL_API.get_unchecked(ClearNamedBufferDataIdx as usize),
        )(buffer, internalformat, format, type_, data)
    }
}
/// Fallbacks: CheckFramebufferStatusEXT
#[inline]
pub unsafe fn CheckFramebufferStatus(target: GLenum) -> GLenum {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum) -> GLenum>(
            *GL_API.get_unchecked(CheckFramebufferStatusIdx as usize),
        )(target)
    }
}
/// Fallbacks: VertexAttribI2uivEXT
#[inline]
pub unsafe fn VertexAttribI2uiv(index: GLuint, v: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttribI2uivIdx as usize),
        )(index, v)
    }
}
/// Fallbacks: BufferStorageEXT
#[inline]
pub unsafe fn BufferStorage(
    target: GLenum,
    size: GLsizeiptr,
    data: *const c_void,
    flags: GLbitfield,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLsizeiptr, *const c_void, GLbitfield) -> ()>(
            *GL_API.get_unchecked(BufferStorageIdx as usize),
        )(target, size, data, flags)
    }
}
/// Fallbacks: PointParameterfARB, PointParameterfEXT, PointParameterfSGIS
#[inline]
pub unsafe fn PointParameterf(pname: GLenum, param: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(
            *GL_API.get_unchecked(PointParameterfIdx as usize),
        )(pname, param)
    }
}
#[inline]
pub unsafe fn GetnColorTable(
    target: GLenum,
    format: GLenum,
    type_: GLenum,
    bufSize: GLsizei,
    table: *mut c_void,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut c_void) -> ()>(
            *GL_API.get_unchecked(GetnColorTableIdx as usize),
        )(target, format, type_, bufSize, table)
    }
}
#[inline]
pub unsafe fn GetnTexImage(
    target: GLenum,
    level: GLint,
    format: GLenum,
    type_: GLenum,
    bufSize: GLsizei,
    pixels: *mut c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLint, GLenum, GLenum, GLsizei, *mut c_void) -> (),
        >(*GL_API.get_unchecked(GetnTexImageIdx as usize))(
            target, level, format, type_, bufSize, pixels,
        )
    }
}
/// Fallbacks: DeleteQueriesARB
#[inline]
pub unsafe fn DeleteQueries(n: GLsizei, ids: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(DeleteQueriesIdx as usize),
        )(n, ids)
    }
}
#[inline]
pub unsafe fn CreateTransformFeedbacks(n: GLsizei, ids: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(CreateTransformFeedbacksIdx as usize),
        )(n, ids)
    }
}
/// Fallbacks: ProgramUniform3fvEXT
#[inline]
pub unsafe fn ProgramUniform3fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(ProgramUniform3fvIdx as usize),
        )(program, location, count, value)
    }
}
#[inline]
pub unsafe fn TransformFeedbackBufferBase(xfb: GLuint, index: GLuint, buffer: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(TransformFeedbackBufferBaseIdx as usize),
        )(xfb, index, buffer)
    }
}
#[inline]
pub unsafe fn UnmapNamedBuffer(buffer: GLuint) -> GLboolean {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
            *GL_API.get_unchecked(UnmapNamedBufferIdx as usize),
        )(buffer)
    }
}
#[inline]
pub unsafe fn GetUniformdv(program: GLuint, location: GLint, params: *mut GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLdouble) -> ()>(
            *GL_API.get_unchecked(GetUniformdvIdx as usize),
        )(program, location, params)
    }
}
/// Fallbacks: CompressedTexImage3DARB, CompressedTexImage3DOES
#[inline]
pub unsafe fn CompressedTexImage3D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    border: GLint,
    imageSize: GLsizei,
    data: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLint,
                GLenum,
                GLsizei,
                GLsizei,
                GLsizei,
                GLint,
                GLsizei,
                *const c_void,
            ) -> (),
        >(*GL_API.get_unchecked(CompressedTexImage3DIdx as usize))(
            target,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            imageSize,
            data,
        )
    }
}
/// Fallbacks: DrawElementsInstancedANGLE, DrawElementsInstancedARB, DrawElementsInstancedEXT, DrawElementsInstancedNV
#[inline]
pub unsafe fn DrawElementsInstanced(
    mode: GLenum,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
    instancecount: GLsizei,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei) -> ()>(
            *GL_API.get_unchecked(DrawElementsInstancedIdx as usize),
        )(mode, count, type_, indices, instancecount)
    }
}
/// Fallbacks: GenQueriesARB
#[inline]
pub unsafe fn GenQueries(n: GLsizei, ids: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(GenQueriesIdx as usize),
        )(n, ids)
    }
}
/// Fallbacks: CopyTexSubImage2DEXT
#[inline]
pub unsafe fn CopyTexSubImage2D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei) -> (),
        >(*GL_API.get_unchecked(CopyTexSubImage2DIdx as usize))(
            target, level, xoffset, yoffset, x, y, width, height,
        )
    }
}
/// Fallbacks: DrawArraysInstancedBaseInstanceEXT
#[inline]
pub unsafe fn DrawArraysInstancedBaseInstance(
    mode: GLenum,
    first: GLint,
    count: GLsizei,
    instancecount: GLsizei,
    baseinstance: GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei, GLsizei, GLuint) -> ()>(
            *GL_API.get_unchecked(DrawArraysInstancedBaseInstanceIdx as usize),
        )(mode, first, count, instancecount, baseinstance)
    }
}
#[inline]
pub unsafe fn TexCoordP4ui(type_: GLenum, coords: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(TexCoordP4uiIdx as usize),
        )(type_, coords)
    }
}
#[inline]
pub unsafe fn VertexAttribP2ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttribP2uiIdx as usize),
        )(index, type_, normalized, value)
    }
}
/// Fallbacks: VertexAttrib4dvARB, VertexAttrib4dvNV
#[inline]
pub unsafe fn VertexAttrib4dv(index: GLuint, v: *const GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(VertexAttrib4dvIdx as usize),
        )(index, v)
    }
}
#[inline]
pub unsafe fn ColorP4uiv(type_: GLenum, color: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(
            *GL_API.get_unchecked(ColorP4uivIdx as usize),
        )(type_, color)
    }
}
#[inline]
pub unsafe fn GetActiveSubroutineName(
    program: GLuint,
    shadertype: GLenum,
    index: GLuint,
    bufsize: GLsizei,
    length: *mut GLsizei,
    name: *mut GLchar,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> (),
        >(*GL_API.get_unchecked(GetActiveSubroutineNameIdx as usize))(
            program, shadertype, index, bufsize, length, name,
        )
    }
}
#[inline]
pub unsafe fn TexCoordP4uiv(type_: GLenum, coords: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(
            *GL_API.get_unchecked(TexCoordP4uivIdx as usize),
        )(type_, coords)
    }
}
/// Fallbacks: ProgramUniform3fEXT
#[inline]
pub unsafe fn ProgramUniform3f(
    program: GLuint,
    location: GLint,
    v0: GLfloat,
    v1: GLfloat,
    v2: GLfloat,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat) -> ()>(
            *GL_API.get_unchecked(ProgramUniform3fIdx as usize),
        )(program, location, v0, v1, v2)
    }
}
/// Fallbacks: ProgramUniform1ivEXT
#[inline]
pub unsafe fn ProgramUniform1iv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> ()>(
            *GL_API.get_unchecked(ProgramUniform1ivIdx as usize),
        )(program, location, count, value)
    }
}
/// Fallbacks: VertexAttrib1fARB, VertexAttrib1fNV
#[inline]
pub unsafe fn VertexAttrib1f(index: GLuint, x: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLfloat) -> ()>(
            *GL_API.get_unchecked(VertexAttrib1fIdx as usize),
        )(index, x)
    }
}
#[inline]
pub unsafe fn Uniform1d(location: GLint, x: GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLdouble) -> ()>(
            *GL_API.get_unchecked(Uniform1dIdx as usize),
        )(location, x)
    }
}
/// Fallbacks: Uniform2ivARB
#[inline]
pub unsafe fn Uniform2iv(location: GLint, count: GLsizei, value: *const GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(
            *GL_API.get_unchecked(Uniform2ivIdx as usize),
        )(location, count, value)
    }
}
/// Fallbacks: CompressedTexImage2DARB
#[inline]
pub unsafe fn CompressedTexImage2D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    imageSize: GLsizei,
    data: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLint,
                GLenum,
                GLsizei,
                GLsizei,
                GLint,
                GLsizei,
                *const c_void,
            ) -> (),
        >(*GL_API.get_unchecked(CompressedTexImage2DIdx as usize))(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            imageSize,
            data,
        )
    }
}
#[inline]
pub unsafe fn DrawBuffer(buf: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(
            *GL_API.get_unchecked(DrawBufferIdx as usize),
        )(buf)
    }
}
#[inline]
pub unsafe fn ClearNamedFramebufferiv(
    framebuffer: GLuint,
    buffer: GLenum,
    drawbuffer: GLint,
    value: *const GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint, *const GLint) -> ()>(
            *GL_API.get_unchecked(ClearNamedFramebufferivIdx as usize),
        )(framebuffer, buffer, drawbuffer, value)
    }
}
#[inline]
pub unsafe fn Hint(target: GLenum, mode: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(
            *GL_API.get_unchecked(HintIdx as usize),
        )(target, mode)
    }
}
/// Fallbacks: DeleteBuffersARB
#[inline]
pub unsafe fn DeleteBuffers(n: GLsizei, buffers: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(DeleteBuffersIdx as usize),
        )(n, buffers)
    }
}
#[inline]
pub unsafe fn VertexArrayAttribFormat(
    vaobj: GLuint,
    attribindex: GLuint,
    size: GLint,
    type_: GLenum,
    normalized: GLboolean,
    relativeoffset: GLuint,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLuint, GLint, GLenum, GLboolean, GLuint) -> (),
        >(*GL_API.get_unchecked(VertexArrayAttribFormatIdx as usize))(
            vaobj,
            attribindex,
            size,
            type_,
            normalized,
            relativeoffset,
        )
    }
}
/// Fallbacks: GenTransformFeedbacksNV
#[inline]
pub unsafe fn GenTransformFeedbacks(n: GLsizei, ids: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(GenTransformFeedbacksIdx as usize),
        )(n, ids)
    }
}
/// Fallbacks: IsBufferARB
#[inline]
pub unsafe fn IsBuffer(buffer: GLuint) -> GLboolean {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
            *GL_API.get_unchecked(IsBufferIdx as usize),
        )(buffer)
    }
}
/// Fallbacks: DrawElementsInstancedBaseVertexEXT, DrawElementsInstancedBaseVertexOES
#[inline]
pub unsafe fn DrawElementsInstancedBaseVertex(
    mode: GLenum,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
    instancecount: GLsizei,
    basevertex: GLint,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLint) -> (),
        >(*GL_API.get_unchecked(DrawElementsInstancedBaseVertexIdx as usize))(
            mode,
            count,
            type_,
            indices,
            instancecount,
            basevertex,
        )
    }
}
/// Fallbacks: Uniform3iARB
#[inline]
pub unsafe fn Uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint) -> ()>(
            *GL_API.get_unchecked(Uniform3iIdx as usize),
        )(location, v0, v1, v2)
    }
}
/// Fallbacks: GetProgramBinaryOES
#[inline]
pub unsafe fn GetProgramBinary(
    program: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    binaryFormat: *mut GLenum,
    binary: *mut c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLenum, *mut c_void) -> (),
        >(*GL_API.get_unchecked(GetProgramBinaryIdx as usize))(
            program,
            bufSize,
            length,
            binaryFormat,
            binary,
        )
    }
}
/// Fallbacks: GetVertexAttribPointervARB, GetVertexAttribPointervNV
#[inline]
pub unsafe fn GetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: *const *mut c_void) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const *mut c_void) -> ()>(
            *GL_API.get_unchecked(GetVertexAttribPointervIdx as usize),
        )(index, pname, pointer)
    }
}
#[inline]
pub unsafe fn GetActiveUniformBlockiv(
    program: GLuint,
    uniformBlockIndex: GLuint,
    pname: GLenum,
    params: *mut GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetActiveUniformBlockivIdx as usize),
        )(program, uniformBlockIndex, pname, params)
    }
}
#[inline]
pub unsafe fn ProgramUniform3dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(ProgramUniform3dvIdx as usize),
        )(program, location, count, value)
    }
}
/// Fallbacks: TexStorage3DEXT
#[inline]
pub unsafe fn TexStorage3D(
    target: GLenum,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei) -> (),
        >(*GL_API.get_unchecked(TexStorage3DIdx as usize))(
            target,
            levels,
            internalformat,
            width,
            height,
            depth,
        )
    }
}
#[inline]
pub unsafe fn GetQueryBufferObjectiv(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, GLintptr) -> ()>(
            *GL_API.get_unchecked(GetQueryBufferObjectivIdx as usize),
        )(id, buffer, pname, offset)
    }
}
/// Fallbacks: DepthRangefOES
#[inline]
pub unsafe fn DepthRangef(n: GLfloat, f: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(
            *GL_API.get_unchecked(DepthRangefIdx as usize),
        )(n, f)
    }
}
#[inline]
pub unsafe fn DeleteProgramPipelines(n: GLsizei, pipelines: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(DeleteProgramPipelinesIdx as usize),
        )(n, pipelines)
    }
}
/// Fallbacks: VertexAttrib4NusvARB
#[inline]
pub unsafe fn VertexAttrib4Nusv(index: GLuint, v: *const GLushort) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLushort) -> ()>(
            *GL_API.get_unchecked(VertexAttrib4NusvIdx as usize),
        )(index, v)
    }
}
#[inline]
pub unsafe fn ClearTexSubImage(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    type_: GLenum,
    data: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLsizei,
                GLenum,
                GLenum,
                *const c_void,
            ) -> (),
        >(*GL_API.get_unchecked(ClearTexSubImageIdx as usize))(
            texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, data,
        )
    }
}
#[inline]
pub unsafe fn MultiTexCoordP3ui(texture: GLenum, type_: GLenum, coords: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(MultiTexCoordP3uiIdx as usize),
        )(texture, type_, coords)
    }
}
/// Fallbacks: ProgramUniform2fEXT
#[inline]
pub unsafe fn ProgramUniform2f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLfloat, GLfloat) -> ()>(
            *GL_API.get_unchecked(ProgramUniform2fIdx as usize),
        )(program, location, v0, v1)
    }
}
/// Fallbacks: IsQueryARB
#[inline]
pub unsafe fn IsQuery(id: GLuint) -> GLboolean {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
            *GL_API.get_unchecked(IsQueryIdx as usize),
        )(id)
    }
}
#[inline]
pub unsafe fn GetnSeparableFilter(
    target: GLenum,
    format: GLenum,
    type_: GLenum,
    rowBufSize: GLsizei,
    row: *mut c_void,
    columnBufSize: GLsizei,
    column: *mut c_void,
    span: *mut c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLenum,
                GLenum,
                GLsizei,
                *mut c_void,
                GLsizei,
                *mut c_void,
                *mut c_void,
            ) -> (),
        >(*GL_API.get_unchecked(GetnSeparableFilterIdx as usize))(
            target,
            format,
            type_,
            rowBufSize,
            row,
            columnBufSize,
            column,
            span,
        )
    }
}
#[inline]
pub unsafe fn GetProgramInfoLog(
    program: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    infoLog: *mut GLchar,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> ()>(
            *GL_API.get_unchecked(GetProgramInfoLogIdx as usize),
        )(program, bufSize, length, infoLog)
    }
}
#[inline]
pub unsafe fn BindRenderbuffer(target: GLenum, renderbuffer: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(BindRenderbufferIdx as usize),
        )(target, renderbuffer)
    }
}
/// Fallbacks: RenderbufferStorageEXT
#[inline]
pub unsafe fn RenderbufferStorage(
    target: GLenum,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLsizei, GLsizei) -> ()>(
            *GL_API.get_unchecked(RenderbufferStorageIdx as usize),
        )(target, internalformat, width, height)
    }
}
/// Fallbacks: DebugMessageControlARB, DebugMessageControlKHR
#[inline]
pub unsafe fn DebugMessageControl(
    source: GLenum,
    type_: GLenum,
    severity: GLenum,
    count: GLsizei,
    ids: *const GLuint,
    enabled: GLboolean,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *const GLuint, GLboolean) -> (),
        >(*GL_API.get_unchecked(DebugMessageControlIdx as usize))(
            source, type_, severity, count, ids, enabled,
        )
    }
}
/// Fallbacks: GetnUniformuivKHR
#[inline]
pub unsafe fn GetnUniformuiv(
    program: GLuint,
    location: GLint,
    bufSize: GLsizei,
    params: *mut GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(GetnUniformuivIdx as usize),
        )(program, location, bufSize, params)
    }
}
#[inline]
pub unsafe fn PolygonOffset(factor: GLfloat, units: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(
            *GL_API.get_unchecked(PolygonOffsetIdx as usize),
        )(factor, units)
    }
}
/// Fallbacks: MultiDrawElementsBaseVertexEXT, MultiDrawElementsBaseVertexOES
#[inline]
pub unsafe fn MultiDrawElementsBaseVertex(
    mode: GLenum,
    count: *const GLsizei,
    type_: GLenum,
    indices: *const *const c_void,
    drawcount: GLsizei,
    basevertex: *const GLint,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                *const GLsizei,
                GLenum,
                *const *const c_void,
                GLsizei,
                *const GLint,
            ) -> (),
        >(*GL_API.get_unchecked(MultiDrawElementsBaseVertexIdx as usize))(
            mode, count, type_, indices, drawcount, basevertex,
        )
    }
}
#[inline]
pub unsafe fn NamedFramebufferDrawBuffer(framebuffer: GLuint, buf: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>(
            *GL_API.get_unchecked(NamedFramebufferDrawBufferIdx as usize),
        )(framebuffer, buf)
    }
}
/// Fallbacks: VertexAttrib2dARB, VertexAttrib2dNV
#[inline]
pub unsafe fn VertexAttrib2d(index: GLuint, x: GLdouble, y: GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble) -> ()>(
            *GL_API.get_unchecked(VertexAttrib2dIdx as usize),
        )(index, x, y)
    }
}
#[inline]
pub unsafe fn CreateTextures(target: GLenum, n: GLsizei, textures: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(CreateTexturesIdx as usize),
        )(target, n, textures)
    }
}
#[inline]
pub unsafe fn GetUniformSubroutineuiv(shadertype: GLenum, location: GLint, params: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(GetUniformSubroutineuivIdx as usize),
        )(shadertype, location, params)
    }
}
#[inline]
pub unsafe fn ClearNamedFramebufferfv(
    framebuffer: GLuint,
    buffer: GLenum,
    drawbuffer: GLint,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(ClearNamedFramebufferfvIdx as usize),
        )(framebuffer, buffer, drawbuffer, value)
    }
}
#[inline]
pub unsafe fn CreateRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(CreateRenderbuffersIdx as usize),
        )(n, renderbuffers)
    }
}
#[inline]
pub unsafe fn IsSampler(sampler: GLuint) -> GLboolean {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
            *GL_API.get_unchecked(IsSamplerIdx as usize),
        )(sampler)
    }
}
#[inline]
pub unsafe fn MultiTexCoordP4uiv(texture: GLenum, type_: GLenum, coords: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLuint) -> ()>(
            *GL_API.get_unchecked(MultiTexCoordP4uivIdx as usize),
        )(texture, type_, coords)
    }
}
/// Fallbacks: GetSyncivAPPLE
#[inline]
pub unsafe fn GetSynciv(
    sync: GLsync,
    pname: GLenum,
    bufSize: GLsizei,
    length: *mut GLsizei,
    values: *mut GLint,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLsync, GLenum, GLsizei, *mut GLsizei, *mut GLint) -> (),
        >(*GL_API.get_unchecked(GetSyncivIdx as usize))(sync, pname, bufSize, length, values)
    }
}
/// Fallbacks: UnmapBufferARB, UnmapBufferOES
#[inline]
pub unsafe fn UnmapBuffer(target: GLenum) -> GLboolean {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum) -> GLboolean>(
            *GL_API.get_unchecked(UnmapBufferIdx as usize),
        )(target)
    }
}
/// Fallbacks: GetBufferPointervARB, GetBufferPointervOES
#[inline]
pub unsafe fn GetBufferPointerv(target: GLenum, pname: GLenum, params: *const *mut c_void) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const *mut c_void) -> ()>(
            *GL_API.get_unchecked(GetBufferPointervIdx as usize),
        )(target, pname, params)
    }
}
/// Fallbacks: GenVertexArraysAPPLE, GenVertexArraysOES
#[inline]
pub unsafe fn GenVertexArrays(n: GLsizei, arrays: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(GenVertexArraysIdx as usize),
        )(n, arrays)
    }
}
#[inline]
pub unsafe fn SampleMaski(maskNumber: GLuint, mask: GLbitfield) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLbitfield) -> ()>(
            *GL_API.get_unchecked(SampleMaskiIdx as usize),
        )(maskNumber, mask)
    }
}
#[inline]
pub unsafe fn ClearStencil(s: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint) -> ()>(
            *GL_API.get_unchecked(ClearStencilIdx as usize),
        )(s)
    }
}
/// Fallbacks: BlendFuncSeparateIndexedAMD, BlendFuncSeparateiARB, BlendFuncSeparateiEXT, BlendFuncSeparateiOES
#[inline]
pub unsafe fn BlendFuncSeparatei(
    buf: GLuint,
    srcRGB: GLenum,
    dstRGB: GLenum,
    srcAlpha: GLenum,
    dstAlpha: GLenum,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, GLenum, GLenum) -> ()>(
            *GL_API.get_unchecked(BlendFuncSeparateiIdx as usize),
        )(buf, srcRGB, dstRGB, srcAlpha, dstAlpha)
    }
}
/// Fallbacks: VertexAttrib4NubARB, VertexAttrib4ubNV
#[inline]
pub unsafe fn VertexAttrib4Nub(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLubyte, GLubyte, GLubyte, GLubyte) -> ()>(
            *GL_API.get_unchecked(VertexAttrib4NubIdx as usize),
        )(index, x, y, z, w)
    }
}
#[inline]
pub unsafe fn ShaderBinary(
    count: GLsizei,
    shaders: *const GLuint,
    binaryformat: GLenum,
    binary: *const c_void,
    length: GLsizei,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLsizei, *const GLuint, GLenum, *const c_void, GLsizei) -> (),
        >(*GL_API.get_unchecked(ShaderBinaryIdx as usize))(
            count,
            shaders,
            binaryformat,
            binary,
            length,
        )
    }
}
#[inline]
pub unsafe fn TextureSubImage3D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLsizei,
                GLenum,
                GLenum,
                *const c_void,
            ) -> (),
        >(*GL_API.get_unchecked(TextureSubImage3DIdx as usize))(
            texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels,
        )
    }
}
/// Fallbacks: GetUniformivARB
#[inline]
pub unsafe fn GetUniformiv(program: GLuint, location: GLint, params: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetUniformivIdx as usize),
        )(program, location, params)
    }
}
/// Fallbacks: Uniform1uivEXT
#[inline]
pub unsafe fn Uniform1uiv(location: GLint, count: GLsizei, value: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(Uniform1uivIdx as usize),
        )(location, count, value)
    }
}
/// Fallbacks: VertexAttribI4svEXT
#[inline]
pub unsafe fn VertexAttribI4sv(index: GLuint, v: *const GLshort) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLshort) -> ()>(
            *GL_API.get_unchecked(VertexAttribI4svIdx as usize),
        )(index, v)
    }
}
#[inline]
pub unsafe fn BlitNamedFramebuffer(
    readFramebuffer: GLuint,
    drawFramebuffer: GLuint,
    srcX0: GLint,
    srcY0: GLint,
    srcX1: GLint,
    srcY1: GLint,
    dstX0: GLint,
    dstY0: GLint,
    dstX1: GLint,
    dstY1: GLint,
    mask: GLbitfield,
    filter: GLenum,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLuint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLbitfield,
                GLenum,
            ) -> (),
        >(*GL_API.get_unchecked(BlitNamedFramebufferIdx as usize))(
            readFramebuffer,
            drawFramebuffer,
            srcX0,
            srcY0,
            srcX1,
            srcY1,
            dstX0,
            dstY0,
            dstX1,
            dstY1,
            mask,
            filter,
        )
    }
}
#[inline]
pub unsafe fn GetAttachedShaders(
    program: GLuint,
    maxCount: GLsizei,
    count: *mut GLsizei,
    shaders: *mut GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(GetAttachedShadersIdx as usize),
        )(program, maxCount, count, shaders)
    }
}
#[inline]
pub unsafe fn InvalidateBufferSubData(buffer: GLuint, offset: GLintptr, length: GLsizeiptr) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLintptr, GLsizeiptr) -> ()>(
            *GL_API.get_unchecked(InvalidateBufferSubDataIdx as usize),
        )(buffer, offset, length)
    }
}
#[inline]
pub unsafe fn InvalidateFramebuffer(
    target: GLenum,
    numAttachments: GLsizei,
    attachments: *const GLenum,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const GLenum) -> ()>(
            *GL_API.get_unchecked(InvalidateFramebufferIdx as usize),
        )(target, numAttachments, attachments)
    }
}
#[inline]
pub unsafe fn TextureStorage1D(
    texture: GLuint,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, GLenum, GLsizei) -> ()>(
            *GL_API.get_unchecked(TextureStorage1DIdx as usize),
        )(texture, levels, internalformat, width)
    }
}
/// Fallbacks: FramebufferTexture1DEXT
#[inline]
pub unsafe fn FramebufferTexture1D(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint) -> ()>(
            *GL_API.get_unchecked(FramebufferTexture1DIdx as usize),
        )(target, attachment, textarget, texture, level)
    }
}
#[inline]
pub unsafe fn GetnMapiv(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLsizei, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetnMapivIdx as usize),
        )(target, query, bufSize, v)
    }
}
/// Fallbacks: GetQueryObjectuivARB
#[inline]
pub unsafe fn GetQueryObjectuiv(id: GLuint, pname: GLenum, params: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(GetQueryObjectuivIdx as usize),
        )(id, pname, params)
    }
}
/// Fallbacks: DetachObjectARB
#[inline]
pub unsafe fn DetachShader(program: GLuint, shader: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(DetachShaderIdx as usize),
        )(program, shader)
    }
}
#[inline]
pub unsafe fn GetActiveUniformBlockName(
    program: GLuint,
    uniformBlockIndex: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    uniformBlockName: *mut GLchar,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> (),
        >(*GL_API.get_unchecked(GetActiveUniformBlockNameIdx as usize))(
            program,
            uniformBlockIndex,
            bufSize,
            length,
            uniformBlockName,
        )
    }
}
/// Fallbacks: IsSyncAPPLE
#[inline]
pub unsafe fn IsSync(sync: GLsync) -> GLboolean {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsync) -> GLboolean>(
            *GL_API.get_unchecked(IsSyncIdx as usize),
        )(sync)
    }
}
#[inline]
pub unsafe fn GetBooleanv(pname: GLenum, data: *mut GLboolean) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *mut GLboolean) -> ()>(
            *GL_API.get_unchecked(GetBooleanvIdx as usize),
        )(pname, data)
    }
}
/// Fallbacks: QueryCounterEXT
#[inline]
pub unsafe fn QueryCounter(id: GLuint, target: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>(
            *GL_API.get_unchecked(QueryCounterIdx as usize),
        )(id, target)
    }
}
#[inline]
pub unsafe fn InvalidateNamedFramebufferData(
    framebuffer: GLuint,
    numAttachments: GLsizei,
    attachments: *const GLenum,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLenum) -> ()>(
            *GL_API.get_unchecked(InvalidateNamedFramebufferDataIdx as usize),
        )(framebuffer, numAttachments, attachments)
    }
}
/// Fallbacks: TexSubImage1DEXT
#[inline]
pub unsafe fn TexSubImage1D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    width: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLenum, *const c_void) -> (),
        >(*GL_API.get_unchecked(TexSubImage1DIdx as usize))(
            target, level, xoffset, width, format, type_, pixels,
        )
    }
}
#[inline]
pub unsafe fn CopyTextureSubImage1D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei) -> ()>(
            *GL_API.get_unchecked(CopyTextureSubImage1DIdx as usize),
        )(texture, level, xoffset, x, y, width)
    }
}
/// Fallbacks: GetIntegerIndexedvEXT
#[inline]
pub unsafe fn GetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetIntegeri_vIdx as usize),
        )(target, index, data)
    }
}
/// Fallbacks: Uniform3fvARB
#[inline]
pub unsafe fn Uniform3fv(location: GLint, count: GLsizei, value: *const GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(Uniform3fvIdx as usize),
        )(location, count, value)
    }
}
/// Fallbacks: VertexAttrib1dvARB, VertexAttrib1dvNV
#[inline]
pub unsafe fn VertexAttrib1dv(index: GLuint, v: *const GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(VertexAttrib1dvIdx as usize),
        )(index, v)
    }
}
/// Fallbacks: DisableIndexedEXT, DisableiEXT, DisableiNV, DisableiOES
#[inline]
pub unsafe fn Disablei(target: GLenum, index: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(DisableiIdx as usize),
        )(target, index)
    }
}
/// Fallbacks: ViewportIndexedfvNV
#[inline]
pub unsafe fn ViewportIndexedfv(index: GLuint, v: *const GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(ViewportIndexedfvIdx as usize),
        )(index, v)
    }
}
/// Fallbacks: PatchParameteriEXT, PatchParameteriOES
#[inline]
pub unsafe fn PatchParameteri(pname: GLenum, value: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(
            *GL_API.get_unchecked(PatchParameteriIdx as usize),
        )(pname, value)
    }
}
/// Fallbacks: VertexAttribI2iEXT
#[inline]
pub unsafe fn VertexAttribI2i(index: GLuint, x: GLint, y: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint) -> ()>(
            *GL_API.get_unchecked(VertexAttribI2iIdx as usize),
        )(index, x, y)
    }
}
/// Fallbacks: Uniform1iARB
#[inline]
pub unsafe fn Uniform1i(location: GLint, v0: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLint) -> ()>(
            *GL_API.get_unchecked(Uniform1iIdx as usize),
        )(location, v0)
    }
}
#[inline]
pub unsafe fn UniformMatrix3x4dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(UniformMatrix3x4dvIdx as usize),
        )(location, count, transpose, value)
    }
}
/// Fallbacks: VertexAttribL4dvEXT
#[inline]
pub unsafe fn VertexAttribL4dv(index: GLuint, v: *const GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(VertexAttribL4dvIdx as usize),
        )(index, v)
    }
}
#[inline]
pub unsafe fn SamplerParameterfv(sampler: GLuint, pname: GLenum, param: *const GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(SamplerParameterfvIdx as usize),
        )(sampler, pname, param)
    }
}
/// Fallbacks: VertexAttrib3dvARB, VertexAttrib3dvNV
#[inline]
pub unsafe fn VertexAttrib3dv(index: GLuint, v: *const GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(VertexAttrib3dvIdx as usize),
        )(index, v)
    }
}
#[inline]
pub unsafe fn ColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLboolean, GLboolean, GLboolean, GLboolean) -> ()>(
            *GL_API.get_unchecked(ColorMaskIdx as usize),
        )(red, green, blue, alpha)
    }
}
#[inline]
pub unsafe fn GetUniformBlockIndex(program: GLuint, uniformBlockName: *const GLchar) -> GLuint {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLuint>(
            *GL_API.get_unchecked(GetUniformBlockIndexIdx as usize),
        )(program, uniformBlockName)
    }
}
#[inline]
pub unsafe fn TextureParameterf(texture: GLuint, pname: GLenum, param: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLfloat) -> ()>(
            *GL_API.get_unchecked(TextureParameterfIdx as usize),
        )(texture, pname, param)
    }
}
/// Fallbacks: GetMultisamplefvNV
#[inline]
pub unsafe fn GetMultisamplefv(pname: GLenum, index: GLuint, val: *mut GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLfloat) -> ()>(
            *GL_API.get_unchecked(GetMultisamplefvIdx as usize),
        )(pname, index, val)
    }
}
/// Fallbacks: ProgramParameteriARB, ProgramParameteriEXT
#[inline]
pub unsafe fn ProgramParameteri(program: GLuint, pname: GLenum, value: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>(
            *GL_API.get_unchecked(ProgramParameteriIdx as usize),
        )(program, pname, value)
    }
}
#[inline]
pub unsafe fn MapNamedBuffer(buffer: GLuint, access: GLenum) -> *mut c_void {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> *mut c_void>(
            *GL_API.get_unchecked(MapNamedBufferIdx as usize),
        )(buffer, access)
    }
}
#[inline]
pub unsafe fn TextureBuffer(texture: GLuint, internalformat: GLenum, buffer: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(TextureBufferIdx as usize),
        )(texture, internalformat, buffer)
    }
}
#[inline]
pub unsafe fn NormalP3uiv(type_: GLenum, coords: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(
            *GL_API.get_unchecked(NormalP3uivIdx as usize),
        )(type_, coords)
    }
}
/// Fallbacks: BlendFuncIndexedAMD, BlendFunciARB, BlendFunciEXT, BlendFunciOES
#[inline]
pub unsafe fn BlendFunci(buf: GLuint, src: GLenum, dst: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum) -> ()>(
            *GL_API.get_unchecked(BlendFunciIdx as usize),
        )(buf, src, dst)
    }
}
/// Fallbacks: VertexAttrib2sARB, VertexAttrib2sNV
#[inline]
pub unsafe fn VertexAttrib2s(index: GLuint, x: GLshort, y: GLshort) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLshort, GLshort) -> ()>(
            *GL_API.get_unchecked(VertexAttrib2sIdx as usize),
        )(index, x, y)
    }
}
#[inline]
pub unsafe fn VertexAttribP3ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttribP3uiIdx as usize),
        )(index, type_, normalized, value)
    }
}
#[inline]
pub unsafe fn GetNamedFramebufferAttachmentParameteriv(
    framebuffer: GLuint,
    attachment: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetNamedFramebufferAttachmentParameterivIdx as usize),
        )(framebuffer, attachment, pname, params)
    }
}
#[inline]
pub unsafe fn NamedRenderbufferStorage(
    renderbuffer: GLuint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLsizei, GLsizei) -> ()>(
            *GL_API.get_unchecked(NamedRenderbufferStorageIdx as usize),
        )(renderbuffer, internalformat, width, height)
    }
}
/// Fallbacks: ProgramUniform1fvEXT
#[inline]
pub unsafe fn ProgramUniform1fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(ProgramUniform1fvIdx as usize),
        )(program, location, count, value)
    }
}
/// Fallbacks: BlendEquationSeparateEXT
#[inline]
pub unsafe fn BlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(
            *GL_API.get_unchecked(BlendEquationSeparateIdx as usize),
        )(modeRGB, modeAlpha)
    }
}
/// Fallbacks: TexBufferARB, TexBufferEXT, TexBufferOES
#[inline]
pub unsafe fn TexBuffer(target: GLenum, internalformat: GLenum, buffer: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(TexBufferIdx as usize),
        )(target, internalformat, buffer)
    }
}
#[inline]
pub unsafe fn TexImage1D(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLint,
                GLint,
                GLsizei,
                GLint,
                GLenum,
                GLenum,
                *const c_void,
            ) -> (),
        >(*GL_API.get_unchecked(TexImage1DIdx as usize))(
            target,
            level,
            internalformat,
            width,
            border,
            format,
            type_,
            pixels,
        )
    }
}
/// Fallbacks: TexParameterIuivEXT, TexParameterIuivOES
#[inline]
pub unsafe fn TexParameterIuiv(target: GLenum, pname: GLenum, params: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLuint) -> ()>(
            *GL_API.get_unchecked(TexParameterIuivIdx as usize),
        )(target, pname, params)
    }
}
#[inline]
pub unsafe fn VertexP2ui(type_: GLenum, value: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexP2uiIdx as usize),
        )(type_, value)
    }
}
/// Fallbacks: GenRenderbuffersEXT
#[inline]
pub unsafe fn GenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(GenRenderbuffersIdx as usize),
        )(n, renderbuffers)
    }
}
#[inline]
pub unsafe fn VertexBindingDivisor(bindingindex: GLuint, divisor: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexBindingDivisorIdx as usize),
        )(bindingindex, divisor)
    }
}
/// Fallbacks: ProgramUniform2iEXT
#[inline]
pub unsafe fn ProgramUniform2i(program: GLuint, location: GLint, v0: GLint, v1: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLint) -> ()>(
            *GL_API.get_unchecked(ProgramUniform2iIdx as usize),
        )(program, location, v0, v1)
    }
}
/// Fallbacks: EnableIndexedEXT, EnableiEXT, EnableiNV, EnableiOES
#[inline]
pub unsafe fn Enablei(target: GLenum, index: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(EnableiIdx as usize),
        )(target, index)
    }
}
#[inline]
pub unsafe fn GetnMapfv(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLsizei, *mut GLfloat) -> ()>(
            *GL_API.get_unchecked(GetnMapfvIdx as usize),
        )(target, query, bufSize, v)
    }
}
/// Fallbacks: IsEnabledIndexedEXT, IsEnablediEXT, IsEnablediNV, IsEnablediOES
#[inline]
pub unsafe fn IsEnabledi(target: GLenum, index: GLuint) -> GLboolean {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> GLboolean>(
            *GL_API.get_unchecked(IsEnablediIdx as usize),
        )(target, index)
    }
}
#[inline]
pub unsafe fn CompressedTextureSubImage3D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLsizei,
                GLenum,
                GLsizei,
                *const c_void,
            ) -> (),
        >(*GL_API.get_unchecked(CompressedTextureSubImage3DIdx as usize))(
            texture, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize,
            data,
        )
    }
}
#[inline]
pub unsafe fn GetShaderPrecisionFormat(
    shadertype: GLenum,
    precisiontype: GLenum,
    range: *mut GLint,
    precision: *mut GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetShaderPrecisionFormatIdx as usize),
        )(shadertype, precisiontype, range, precision)
    }
}
#[inline]
pub unsafe fn GetTextureImage(
    texture: GLuint,
    level: GLint,
    format: GLenum,
    type_: GLenum,
    bufSize: GLsizei,
    pixels: *mut c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLenum, GLenum, GLsizei, *mut c_void) -> (),
        >(*GL_API.get_unchecked(GetTextureImageIdx as usize))(
            texture, level, format, type_, bufSize, pixels,
        )
    }
}
/// Fallbacks: UniformMatrix3x4fvNV
#[inline]
pub unsafe fn UniformMatrix3x4fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(UniformMatrix3x4fvIdx as usize),
        )(location, count, transpose, value)
    }
}
/// Fallbacks: Uniform2uivEXT
#[inline]
pub unsafe fn Uniform2uiv(location: GLint, count: GLsizei, value: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(Uniform2uivIdx as usize),
        )(location, count, value)
    }
}
#[inline]
pub unsafe fn GetInternalformati64v(
    target: GLenum,
    internalformat: GLenum,
    pname: GLenum,
    bufSize: GLsizei,
    params: *mut GLint64,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut GLint64) -> ()>(
            *GL_API.get_unchecked(GetInternalformati64vIdx as usize),
        )(target, internalformat, pname, bufSize, params)
    }
}
#[inline]
pub unsafe fn ProgramUniform2dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(ProgramUniform2dvIdx as usize),
        )(program, location, count, value)
    }
}
/// Fallbacks: VertexAttrib3sARB, VertexAttrib3sNV
#[inline]
pub unsafe fn VertexAttrib3s(index: GLuint, x: GLshort, y: GLshort, z: GLshort) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLshort, GLshort, GLshort) -> ()>(
            *GL_API.get_unchecked(VertexAttrib3sIdx as usize),
        )(index, x, y, z)
    }
}
/// Fallbacks: FlushMappedBufferRangeAPPLE, FlushMappedBufferRangeEXT
#[inline]
pub unsafe fn FlushMappedBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr) -> ()>(
            *GL_API.get_unchecked(FlushMappedBufferRangeIdx as usize),
        )(target, offset, length)
    }
}
#[inline]
pub unsafe fn InvalidateTexImage(texture: GLuint, level: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint) -> ()>(
            *GL_API.get_unchecked(InvalidateTexImageIdx as usize),
        )(texture, level)
    }
}
#[inline]
pub unsafe fn GetProgramInterfaceiv(
    program: GLuint,
    programInterface: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetProgramInterfaceivIdx as usize),
        )(program, programInterface, pname, params)
    }
}
#[inline]
pub unsafe fn CullFace(mode: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(
            *GL_API.get_unchecked(CullFaceIdx as usize),
        )(mode)
    }
}
#[inline]
pub unsafe fn GetFramebufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetFramebufferParameterivIdx as usize),
        )(target, pname, params)
    }
}
/// Fallbacks: CreateShaderObjectARB
#[inline]
pub unsafe fn CreateShader(type_: GLenum) -> GLuint {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum) -> GLuint>(
            *GL_API.get_unchecked(CreateShaderIdx as usize),
        )(type_)
    }
}
#[inline]
pub unsafe fn ProgramUniformMatrix3dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> (),
        >(*GL_API.get_unchecked(ProgramUniformMatrix3dvIdx as usize))(
            program, location, count, transpose, value,
        )
    }
}
/// Fallbacks: PointParameterfvARB, PointParameterfvEXT, PointParameterfvSGIS
#[inline]
pub unsafe fn PointParameterfv(pname: GLenum, params: *const GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(PointParameterfvIdx as usize),
        )(pname, params)
    }
}
#[inline]
pub unsafe fn DrawArraysIndirect(mode: GLenum, indirect: *const c_void) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *const c_void) -> ()>(
            *GL_API.get_unchecked(DrawArraysIndirectIdx as usize),
        )(mode, indirect)
    }
}
/// Fallbacks: UseProgramObjectARB
#[inline]
pub unsafe fn UseProgram(program: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
            *GL_API.get_unchecked(UseProgramIdx as usize),
        )(program)
    }
}
#[inline]
pub unsafe fn ProgramUniformMatrix3x2dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> (),
        >(*GL_API.get_unchecked(ProgramUniformMatrix3x2dvIdx as usize))(
            program, location, count, transpose, value,
        )
    }
}
/// Fallbacks: SampleCoverageARB
#[inline]
pub unsafe fn SampleCoverage(value: GLfloat, invert: GLboolean) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLfloat, GLboolean) -> ()>(
            *GL_API.get_unchecked(SampleCoverageIdx as usize),
        )(value, invert)
    }
}
/// Fallbacks: Uniform3ivARB
#[inline]
pub unsafe fn Uniform3iv(location: GLint, count: GLsizei, value: *const GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(
            *GL_API.get_unchecked(Uniform3ivIdx as usize),
        )(location, count, value)
    }
}
/// Fallbacks: VertexAttribI3ivEXT
#[inline]
pub unsafe fn VertexAttribI3iv(index: GLuint, v: *const GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(
            *GL_API.get_unchecked(VertexAttribI3ivIdx as usize),
        )(index, v)
    }
}
#[inline]
pub unsafe fn ProgramUniform1dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(ProgramUniform1dvIdx as usize),
        )(program, location, count, value)
    }
}
/// Fallbacks: BlendEquationSeparateIndexedAMD, BlendEquationSeparateiARB, BlendEquationSeparateiEXT, BlendEquationSeparateiOES
#[inline]
pub unsafe fn BlendEquationSeparatei(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLenum) -> ()>(
            *GL_API.get_unchecked(BlendEquationSeparateiIdx as usize),
        )(buf, modeRGB, modeAlpha)
    }
}
/// Fallbacks: GetFloatIndexedvEXT, GetFloati_vEXT, GetFloati_vNV
#[inline]
pub unsafe fn GetFloati_v(target: GLenum, index: GLuint, data: *mut GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLfloat) -> ()>(
            *GL_API.get_unchecked(GetFloati_vIdx as usize),
        )(target, index, data)
    }
}
/// Fallbacks: ProgramUniform4ivEXT
#[inline]
pub unsafe fn ProgramUniform4iv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLint) -> ()>(
            *GL_API.get_unchecked(ProgramUniform4ivIdx as usize),
        )(program, location, count, value)
    }
}
#[inline]
pub unsafe fn SecondaryColorP3ui(type_: GLenum, color: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(SecondaryColorP3uiIdx as usize),
        )(type_, color)
    }
}
/// Fallbacks: VertexAttribI1uiEXT
#[inline]
pub unsafe fn VertexAttribI1ui(index: GLuint, x: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttribI1uiIdx as usize),
        )(index, x)
    }
}
/// Fallbacks: Uniform1ivARB
#[inline]
pub unsafe fn Uniform1iv(location: GLint, count: GLsizei, value: *const GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(
            *GL_API.get_unchecked(Uniform1ivIdx as usize),
        )(location, count, value)
    }
}
#[inline]
pub unsafe fn GetVertexArrayiv(vaobj: GLuint, pname: GLenum, param: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetVertexArrayivIdx as usize),
        )(vaobj, pname, param)
    }
}
#[inline]
pub unsafe fn IsProgram(program: GLuint) -> GLboolean {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
            *GL_API.get_unchecked(IsProgramIdx as usize),
        )(program)
    }
}
#[inline]
pub unsafe fn BindTextureUnit(unit: GLuint, texture: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(BindTextureUnitIdx as usize),
        )(unit, texture)
    }
}
#[inline]
pub unsafe fn GetnPolygonStipple(bufSize: GLsizei, pattern: *mut GLubyte) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLubyte) -> ()>(
            *GL_API.get_unchecked(GetnPolygonStippleIdx as usize),
        )(bufSize, pattern)
    }
}
#[inline]
pub unsafe fn GetIntegerv(pname: GLenum, data: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetIntegervIdx as usize),
        )(pname, data)
    }
}
#[inline]
pub unsafe fn NamedFramebufferParameteri(framebuffer: GLuint, pname: GLenum, param: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLint) -> ()>(
            *GL_API.get_unchecked(NamedFramebufferParameteriIdx as usize),
        )(framebuffer, pname, param)
    }
}
#[inline]
pub unsafe fn VertexP3uiv(type_: GLenum, value: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(
            *GL_API.get_unchecked(VertexP3uivIdx as usize),
        )(type_, value)
    }
}
/// Fallbacks: VertexAttrib4usvARB
#[inline]
pub unsafe fn VertexAttrib4usv(index: GLuint, v: *const GLushort) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLushort) -> ()>(
            *GL_API.get_unchecked(VertexAttrib4usvIdx as usize),
        )(index, v)
    }
}
/// Fallbacks: UniformMatrix2x3fvNV
#[inline]
pub unsafe fn UniformMatrix2x3fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(UniformMatrix2x3fvIdx as usize),
        )(location, count, transpose, value)
    }
}
#[inline]
pub unsafe fn GetnMapdv(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLsizei, *mut GLdouble) -> ()>(
            *GL_API.get_unchecked(GetnMapdvIdx as usize),
        )(target, query, bufSize, v)
    }
}
#[inline]
pub unsafe fn TexCoordP1uiv(type_: GLenum, coords: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(
            *GL_API.get_unchecked(TexCoordP1uivIdx as usize),
        )(type_, coords)
    }
}
/// Fallbacks: Uniform1fvARB
#[inline]
pub unsafe fn Uniform1fv(location: GLint, count: GLsizei, value: *const GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(Uniform1fvIdx as usize),
        )(location, count, value)
    }
}
#[inline]
pub unsafe fn GetNamedBufferSubData(
    buffer: GLuint,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *mut c_void,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLintptr, GLsizeiptr, *mut c_void) -> ()>(
            *GL_API.get_unchecked(GetNamedBufferSubDataIdx as usize),
        )(buffer, offset, size, data)
    }
}
/// Fallbacks: TransformFeedbackVaryingsEXT
#[inline]
pub unsafe fn TransformFeedbackVaryings(
    program: GLuint,
    count: GLsizei,
    varyings: *const *const GLchar,
    bufferMode: GLenum,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const *const GLchar, GLenum) -> ()>(
            *GL_API.get_unchecked(TransformFeedbackVaryingsIdx as usize),
        )(program, count, varyings, bufferMode)
    }
}
#[inline]
pub unsafe fn InvalidateNamedFramebufferSubData(
    framebuffer: GLuint,
    numAttachments: GLsizei,
    attachments: *const GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLsizei,
                *const GLenum,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
            ) -> (),
        >(*GL_API.get_unchecked(InvalidateNamedFramebufferSubDataIdx as usize))(
            framebuffer,
            numAttachments,
            attachments,
            x,
            y,
            width,
            height,
        )
    }
}
/// Fallbacks: PointParameteriNV
#[inline]
pub unsafe fn PointParameteri(pname: GLenum, param: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(
            *GL_API.get_unchecked(PointParameteriIdx as usize),
        )(pname, param)
    }
}
#[inline]
pub unsafe fn GetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLfloat) -> ()>(
            *GL_API.get_unchecked(GetTexParameterfvIdx as usize),
        )(target, pname, params)
    }
}
/// Fallbacks: IsTransformFeedbackNV
#[inline]
pub unsafe fn IsTransformFeedback(id: GLuint) -> GLboolean {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(
            *GL_API.get_unchecked(IsTransformFeedbackIdx as usize),
        )(id)
    }
}
#[inline]
pub unsafe fn TextureStorage3D(
    texture: GLuint,
    levels: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLsizei) -> (),
        >(*GL_API.get_unchecked(TextureStorage3DIdx as usize))(
            texture,
            levels,
            internalformat,
            width,
            height,
            depth,
        )
    }
}
#[inline]
pub unsafe fn ClearNamedBufferSubData(
    buffer: GLuint,
    internalformat: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    format: GLenum,
    type_: GLenum,
    data: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLenum,
                GLintptr,
                GLsizeiptr,
                GLenum,
                GLenum,
                *const c_void,
            ) -> (),
        >(*GL_API.get_unchecked(ClearNamedBufferSubDataIdx as usize))(
            buffer,
            internalformat,
            offset,
            size,
            format,
            type_,
            data,
        )
    }
}
/// Fallbacks: GetBufferSubDataARB
#[inline]
pub unsafe fn GetBufferSubData(
    target: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *mut c_void,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr, *mut c_void) -> ()>(
            *GL_API.get_unchecked(GetBufferSubDataIdx as usize),
        )(target, offset, size, data)
    }
}
/// Fallbacks: VertexAttrib4fvARB, VertexAttrib4fvNV
#[inline]
pub unsafe fn VertexAttrib4fv(index: GLuint, v: *const GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(VertexAttrib4fvIdx as usize),
        )(index, v)
    }
}
/// Fallbacks: GetVertexAttribIivEXT
#[inline]
pub unsafe fn GetVertexAttribIiv(index: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetVertexAttribIivIdx as usize),
        )(index, pname, params)
    }
}
/// Fallbacks: GetDebugMessageLogARB, GetDebugMessageLogKHR
#[inline]
pub unsafe fn GetDebugMessageLog(
    count: GLuint,
    bufSize: GLsizei,
    sources: *mut GLenum,
    types: *mut GLenum,
    ids: *mut GLuint,
    severities: *mut GLenum,
    lengths: *mut GLsizei,
    messageLog: *mut GLchar,
) -> GLuint {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLsizei,
                *mut GLenum,
                *mut GLenum,
                *mut GLuint,
                *mut GLenum,
                *mut GLsizei,
                *mut GLchar,
            ) -> GLuint,
        >(*GL_API.get_unchecked(GetDebugMessageLogIdx as usize))(
            count, bufSize, sources, types, ids, severities, lengths, messageLog,
        )
    }
}
#[inline]
pub unsafe fn UniformBlockBinding(
    program: GLuint,
    uniformBlockIndex: GLuint,
    uniformBlockBinding: GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(UniformBlockBindingIdx as usize),
        )(program, uniformBlockIndex, uniformBlockBinding)
    }
}
/// Fallbacks: MapBufferARB, MapBufferOES
#[inline]
pub unsafe fn MapBuffer(target: GLenum, access: GLenum) -> *mut c_void {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> *mut c_void>(
            *GL_API.get_unchecked(MapBufferIdx as usize),
        )(target, access)
    }
}
#[inline]
pub unsafe fn NamedFramebufferDrawBuffers(framebuffer: GLuint, n: GLsizei, bufs: *const GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLenum) -> ()>(
            *GL_API.get_unchecked(NamedFramebufferDrawBuffersIdx as usize),
        )(framebuffer, n, bufs)
    }
}
#[inline]
pub unsafe fn VertexAttribP1uiv(
    index: GLuint,
    type_: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttribP1uivIdx as usize),
        )(index, type_, normalized, value)
    }
}
/// Fallbacks: ClientWaitSyncAPPLE
#[inline]
pub unsafe fn ClientWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsync, GLbitfield, GLuint64) -> GLenum>(
            *GL_API.get_unchecked(ClientWaitSyncIdx as usize),
        )(sync, flags, timeout)
    }
}
/// Fallbacks: GetSamplerParameterIuivEXT, GetSamplerParameterIuivOES
#[inline]
pub unsafe fn GetSamplerParameterIuiv(sampler: GLuint, pname: GLenum, params: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(GetSamplerParameterIuivIdx as usize),
        )(sampler, pname, params)
    }
}
/// Fallbacks: ProgramUniformMatrix4x2fvEXT
#[inline]
pub unsafe fn ProgramUniformMatrix4x2fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
        >(*GL_API.get_unchecked(ProgramUniformMatrix4x2fvIdx as usize))(
            program, location, count, transpose, value,
        )
    }
}
/// Fallbacks: VertexAttribI4bvEXT
#[inline]
pub unsafe fn VertexAttribI4bv(index: GLuint, v: *const GLbyte) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLbyte) -> ()>(
            *GL_API.get_unchecked(VertexAttribI4bvIdx as usize),
        )(index, v)
    }
}
/// Fallbacks: GenFramebuffersEXT
#[inline]
pub unsafe fn GenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(GenFramebuffersIdx as usize),
        )(n, framebuffers)
    }
}
/// Fallbacks: GetVertexAttribIuivEXT
#[inline]
pub unsafe fn GetVertexAttribIuiv(index: GLuint, pname: GLenum, params: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(GetVertexAttribIuivIdx as usize),
        )(index, pname, params)
    }
}
#[inline]
pub unsafe fn ProgramUniformMatrix2x3dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> (),
        >(*GL_API.get_unchecked(ProgramUniformMatrix2x3dvIdx as usize))(
            program, location, count, transpose, value,
        )
    }
}
/// Fallbacks: BufferSubDataARB
#[inline]
pub unsafe fn BufferSubData(
    target: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    data: *const c_void,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLintptr, GLsizeiptr, *const c_void) -> ()>(
            *GL_API.get_unchecked(BufferSubDataIdx as usize),
        )(target, offset, size, data)
    }
}
/// Fallbacks: VertexAttrib3fARB, VertexAttrib3fNV
#[inline]
pub unsafe fn VertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat) -> ()>(
            *GL_API.get_unchecked(VertexAttrib3fIdx as usize),
        )(index, x, y, z)
    }
}
#[inline]
pub unsafe fn TexImage3DMultisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    fixedsamplelocations: GLboolean,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean) -> (),
        >(*GL_API.get_unchecked(TexImage3DMultisampleIdx as usize))(
            target,
            samples,
            internalformat,
            width,
            height,
            depth,
            fixedsamplelocations,
        )
    }
}
#[inline]
pub unsafe fn GetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetTexParameterivIdx as usize),
        )(target, pname, params)
    }
}
#[inline]
pub unsafe fn GetnConvolutionFilter(
    target: GLenum,
    format: GLenum,
    type_: GLenum,
    bufSize: GLsizei,
    image: *mut c_void,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut c_void) -> ()>(
            *GL_API.get_unchecked(GetnConvolutionFilterIdx as usize),
        )(target, format, type_, bufSize, image)
    }
}
/// Fallbacks: VertexAttrib4bvARB
#[inline]
pub unsafe fn VertexAttrib4bv(index: GLuint, v: *const GLbyte) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLbyte) -> ()>(
            *GL_API.get_unchecked(VertexAttrib4bvIdx as usize),
        )(index, v)
    }
}
/// Fallbacks: GetDoubleIndexedvEXT, GetDoublei_vEXT
#[inline]
pub unsafe fn GetDoublei_v(target: GLenum, index: GLuint, data: *mut GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLdouble) -> ()>(
            *GL_API.get_unchecked(GetDoublei_vIdx as usize),
        )(target, index, data)
    }
}
/// Fallbacks: DeleteSyncAPPLE
#[inline]
pub unsafe fn DeleteSync(sync: GLsync) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsync) -> ()>(
            *GL_API.get_unchecked(DeleteSyncIdx as usize),
        )(sync)
    }
}
#[inline]
pub unsafe fn FlushMappedNamedBufferRange(buffer: GLuint, offset: GLintptr, length: GLsizeiptr) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLintptr, GLsizeiptr) -> ()>(
            *GL_API.get_unchecked(FlushMappedNamedBufferRangeIdx as usize),
        )(buffer, offset, length)
    }
}
#[inline]
pub unsafe fn GetActiveUniformName(
    program: GLuint,
    uniformIndex: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    uniformName: *mut GLchar,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> (),
        >(*GL_API.get_unchecked(GetActiveUniformNameIdx as usize))(
            program,
            uniformIndex,
            bufSize,
            length,
            uniformName,
        )
    }
}
/// Fallbacks: ProgramUniform1uivEXT
#[inline]
pub unsafe fn ProgramUniform1uiv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(ProgramUniform1uivIdx as usize),
        )(program, location, count, value)
    }
}
/// Fallbacks: ProgramBinaryOES
#[inline]
pub unsafe fn ProgramBinary(
    program: GLuint,
    binaryFormat: GLenum,
    binary: *const c_void,
    length: GLsizei,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const c_void, GLsizei) -> ()>(
            *GL_API.get_unchecked(ProgramBinaryIdx as usize),
        )(program, binaryFormat, binary, length)
    }
}
#[inline]
pub unsafe fn GenerateTextureMipmap(texture: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
            *GL_API.get_unchecked(GenerateTextureMipmapIdx as usize),
        )(texture)
    }
}
#[inline]
pub unsafe fn DepthRangeArrayv(first: GLuint, count: GLsizei, v: *const GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(DepthRangeArrayvIdx as usize),
        )(first, count, v)
    }
}
#[inline]
pub unsafe fn ProgramUniform2d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLdouble, GLdouble) -> ()>(
            *GL_API.get_unchecked(ProgramUniform2dIdx as usize),
        )(program, location, v0, v1)
    }
}
#[inline]
pub unsafe fn CheckNamedFramebufferStatus(framebuffer: GLuint, target: GLenum) -> GLenum {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> GLenum>(
            *GL_API.get_unchecked(CheckNamedFramebufferStatusIdx as usize),
        )(framebuffer, target)
    }
}
/// Fallbacks: ResumeTransformFeedbackNV
#[inline]
pub unsafe fn ResumeTransformFeedback() {
    unsafe {
        mem::transmute::<_, extern "system" fn() -> ()>(
            *GL_API.get_unchecked(ResumeTransformFeedbackIdx as usize),
        )()
    }
}
#[inline]
pub unsafe fn VertexAttribBinding(attribindex: GLuint, bindingindex: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttribBindingIdx as usize),
        )(attribindex, bindingindex)
    }
}
#[inline]
pub unsafe fn PixelStoref(pname: GLenum, param: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(
            *GL_API.get_unchecked(PixelStorefIdx as usize),
        )(pname, param)
    }
}
#[inline]
pub unsafe fn MultiTexCoordP1ui(texture: GLenum, type_: GLenum, coords: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(MultiTexCoordP1uiIdx as usize),
        )(texture, type_, coords)
    }
}
#[inline]
pub unsafe fn GetSamplerParameterfv(sampler: GLuint, pname: GLenum, params: *mut GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLfloat) -> ()>(
            *GL_API.get_unchecked(GetSamplerParameterfvIdx as usize),
        )(sampler, pname, params)
    }
}
/// Fallbacks: GetTexParameterIuivEXT, GetTexParameterIuivOES
#[inline]
pub unsafe fn GetTexParameterIuiv(target: GLenum, pname: GLenum, params: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(GetTexParameterIuivIdx as usize),
        )(target, pname, params)
    }
}
#[inline]
pub unsafe fn ClipControl(origin: GLenum, depth: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(
            *GL_API.get_unchecked(ClipControlIdx as usize),
        )(origin, depth)
    }
}
#[inline]
pub unsafe fn GetSubroutineIndex(
    program: GLuint,
    shadertype: GLenum,
    name: *const GLchar,
) -> GLuint {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLchar) -> GLuint>(
            *GL_API.get_unchecked(GetSubroutineIndexIdx as usize),
        )(program, shadertype, name)
    }
}
/// Fallbacks: GenBuffersARB
#[inline]
pub unsafe fn GenBuffers(n: GLsizei, buffers: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(GenBuffersIdx as usize),
        )(n, buffers)
    }
}
/// Fallbacks: GetSamplerParameterIivEXT, GetSamplerParameterIivOES
#[inline]
pub unsafe fn GetSamplerParameterIiv(sampler: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetSamplerParameterIivIdx as usize),
        )(sampler, pname, params)
    }
}
#[inline]
pub unsafe fn Uniform3dv(location: GLint, count: GLsizei, value: *const GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(Uniform3dvIdx as usize),
        )(location, count, value)
    }
}
/// Fallbacks: ProgramUniformMatrix3x4fvEXT
#[inline]
pub unsafe fn ProgramUniformMatrix3x4fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
        >(*GL_API.get_unchecked(ProgramUniformMatrix3x4fvIdx as usize))(
            program, location, count, transpose, value,
        )
    }
}
#[inline]
pub unsafe fn LineWidth(width: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(
            *GL_API.get_unchecked(LineWidthIdx as usize),
        )(width)
    }
}
#[inline]
pub unsafe fn VertexArrayAttribLFormat(
    vaobj: GLuint,
    attribindex: GLuint,
    size: GLint,
    type_: GLenum,
    relativeoffset: GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLint, GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexArrayAttribLFormatIdx as usize),
        )(vaobj, attribindex, size, type_, relativeoffset)
    }
}
#[inline]
pub unsafe fn DepthRangeIndexed(index: GLuint, n: GLdouble, f: GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLdouble, GLdouble) -> ()>(
            *GL_API.get_unchecked(DepthRangeIndexedIdx as usize),
        )(index, n, f)
    }
}
#[inline]
pub unsafe fn ProgramUniformMatrix3x4dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> (),
        >(*GL_API.get_unchecked(ProgramUniformMatrix3x4dvIdx as usize))(
            program, location, count, transpose, value,
        )
    }
}
#[inline]
pub unsafe fn GetTextureParameteriv(texture: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetTextureParameterivIdx as usize),
        )(texture, pname, params)
    }
}
/// Fallbacks: DrawElementsInstancedBaseInstanceEXT
#[inline]
pub unsafe fn DrawElementsInstancedBaseInstance(
    mode: GLenum,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
    instancecount: GLsizei,
    baseinstance: GLuint,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLuint) -> (),
        >(*GL_API.get_unchecked(DrawElementsInstancedBaseInstanceIdx as usize))(
            mode,
            count,
            type_,
            indices,
            instancecount,
            baseinstance,
        )
    }
}
/// Fallbacks: GetVertexAttribLdvEXT
#[inline]
pub unsafe fn GetVertexAttribLdv(index: GLuint, pname: GLenum, params: *mut GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLdouble) -> ()>(
            *GL_API.get_unchecked(GetVertexAttribLdvIdx as usize),
        )(index, pname, params)
    }
}
#[inline]
pub unsafe fn VertexP3ui(type_: GLenum, value: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(VertexP3uiIdx as usize),
        )(type_, value)
    }
}
#[inline]
pub unsafe fn ClearNamedFramebufferfi(
    framebuffer: GLuint,
    buffer: GLenum,
    depth: GLfloat,
    stencil: GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLfloat, GLint) -> ()>(
            *GL_API.get_unchecked(ClearNamedFramebufferfiIdx as usize),
        )(framebuffer, buffer, depth, stencil)
    }
}
#[inline]
pub unsafe fn DrawTransformFeedbackStreamInstanced(
    mode: GLenum,
    id: GLuint,
    stream: GLuint,
    instancecount: GLsizei,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLuint, GLsizei) -> ()>(
            *GL_API.get_unchecked(DrawTransformFeedbackStreamInstancedIdx as usize),
        )(mode, id, stream, instancecount)
    }
}
/// Fallbacks: ProgramUniform3uiEXT
#[inline]
pub unsafe fn ProgramUniform3ui(
    program: GLuint,
    location: GLint,
    v0: GLuint,
    v1: GLuint,
    v2: GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(ProgramUniform3uiIdx as usize),
        )(program, location, v0, v1, v2)
    }
}
#[inline]
pub unsafe fn GetTextureLevelParameteriv(
    texture: GLuint,
    level: GLint,
    pname: GLenum,
    params: *mut GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetTextureLevelParameterivIdx as usize),
        )(texture, level, pname, params)
    }
}
#[inline]
pub unsafe fn Uniform2dv(location: GLint, count: GLsizei, value: *const GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(Uniform2dvIdx as usize),
        )(location, count, value)
    }
}
/// Fallbacks: GetQueryObjectui64vEXT
#[inline]
pub unsafe fn GetQueryObjectui64v(id: GLuint, pname: GLenum, params: *mut GLuint64) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint64) -> ()>(
            *GL_API.get_unchecked(GetQueryObjectui64vIdx as usize),
        )(id, pname, params)
    }
}
/// Fallbacks: ProgramUniform2fvEXT
#[inline]
pub unsafe fn ProgramUniform2fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(ProgramUniform2fvIdx as usize),
        )(program, location, count, value)
    }
}
#[inline]
pub unsafe fn MultiTexCoordP1uiv(texture: GLenum, type_: GLenum, coords: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLuint) -> ()>(
            *GL_API.get_unchecked(MultiTexCoordP1uivIdx as usize),
        )(texture, type_, coords)
    }
}
/// Fallbacks: RenderbufferStorageMultisampleEXT, RenderbufferStorageMultisampleNV
#[inline]
pub unsafe fn RenderbufferStorageMultisample(
    target: GLenum,
    samples: GLsizei,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei) -> ()>(
            *GL_API.get_unchecked(RenderbufferStorageMultisampleIdx as usize),
        )(target, samples, internalformat, width, height)
    }
}
#[inline]
pub unsafe fn ColorP3uiv(type_: GLenum, color: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(
            *GL_API.get_unchecked(ColorP3uivIdx as usize),
        )(type_, color)
    }
}
#[inline]
pub unsafe fn MultiTexCoordP2ui(texture: GLenum, type_: GLenum, coords: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(MultiTexCoordP2uiIdx as usize),
        )(texture, type_, coords)
    }
}
/// Fallbacks: BindFragDataLocationEXT
#[inline]
pub unsafe fn BindFragDataLocation(program: GLuint, color: GLuint, name: *const GLchar) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, *const GLchar) -> ()>(
            *GL_API.get_unchecked(BindFragDataLocationIdx as usize),
        )(program, color, name)
    }
}
/// Fallbacks: Uniform4uivEXT
#[inline]
pub unsafe fn Uniform4uiv(location: GLint, count: GLsizei, value: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(Uniform4uivIdx as usize),
        )(location, count, value)
    }
}
/// Fallbacks: GetFramebufferAttachmentParameterivEXT
#[inline]
pub unsafe fn GetFramebufferAttachmentParameteriv(
    target: GLenum,
    attachment: GLenum,
    pname: GLenum,
    params: *mut GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetFramebufferAttachmentParameterivIdx as usize),
        )(target, attachment, pname, params)
    }
}
#[inline]
pub unsafe fn GetVertexArrayIndexediv(
    vaobj: GLuint,
    index: GLuint,
    pname: GLenum,
    param: *mut GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetVertexArrayIndexedivIdx as usize),
        )(vaobj, index, pname, param)
    }
}
/// Fallbacks: TexParameterIivEXT, TexParameterIivOES
#[inline]
pub unsafe fn TexParameterIiv(target: GLenum, pname: GLenum, params: *const GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(
            *GL_API.get_unchecked(TexParameterIivIdx as usize),
        )(target, pname, params)
    }
}
#[inline]
pub unsafe fn GetNamedBufferParameteri64v(buffer: GLuint, pname: GLenum, params: *mut GLint64) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint64) -> ()>(
            *GL_API.get_unchecked(GetNamedBufferParameteri64vIdx as usize),
        )(buffer, pname, params)
    }
}
/// Fallbacks: UniformMatrix3fvARB
#[inline]
pub unsafe fn UniformMatrix3fv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(UniformMatrix3fvIdx as usize),
        )(location, count, transpose, value)
    }
}
#[inline]
pub unsafe fn ClearBufferData(
    target: GLenum,
    internalformat: GLenum,
    format: GLenum,
    type_: GLenum,
    data: *const c_void,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLenum, *const c_void) -> ()>(
            *GL_API.get_unchecked(ClearBufferDataIdx as usize),
        )(target, internalformat, format, type_, data)
    }
}
#[inline]
pub unsafe fn VertexP4uiv(type_: GLenum, value: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(
            *GL_API.get_unchecked(VertexP4uivIdx as usize),
        )(type_, value)
    }
}
/// Fallbacks: CopyImageSubDataEXT, CopyImageSubDataOES
#[inline]
pub unsafe fn CopyImageSubData(
    srcName: GLuint,
    srcTarget: GLenum,
    srcLevel: GLint,
    srcX: GLint,
    srcY: GLint,
    srcZ: GLint,
    dstName: GLuint,
    dstTarget: GLenum,
    dstLevel: GLint,
    dstX: GLint,
    dstY: GLint,
    dstZ: GLint,
    srcWidth: GLsizei,
    srcHeight: GLsizei,
    srcDepth: GLsizei,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLenum,
                GLint,
                GLint,
                GLint,
                GLint,
                GLuint,
                GLenum,
                GLint,
                GLint,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLsizei,
            ) -> (),
        >(*GL_API.get_unchecked(CopyImageSubDataIdx as usize))(
            srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstName, dstTarget, dstLevel, dstX,
            dstY, dstZ, srcWidth, srcHeight, srcDepth,
        )
    }
}
#[inline]
pub unsafe fn Uniform4dv(location: GLint, count: GLsizei, value: *const GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(Uniform4dvIdx as usize),
        )(location, count, value)
    }
}
#[inline]
pub unsafe fn GenTextures(n: GLsizei, textures: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(GenTexturesIdx as usize),
        )(n, textures)
    }
}
#[inline]
pub unsafe fn TexCoordP2uiv(type_: GLenum, coords: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, *const GLuint) -> ()>(
            *GL_API.get_unchecked(TexCoordP2uivIdx as usize),
        )(type_, coords)
    }
}
/// Fallbacks: VertexAttribL3dvEXT
#[inline]
pub unsafe fn VertexAttribL3dv(index: GLuint, v: *const GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(VertexAttribL3dvIdx as usize),
        )(index, v)
    }
}
/// Fallbacks: CompressedTexImage1DARB
#[inline]
pub unsafe fn CompressedTexImage1D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    border: GLint,
    imageSize: GLsizei,
    data: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLint, GLsizei, *const c_void) -> (),
        >(*GL_API.get_unchecked(CompressedTexImage1DIdx as usize))(
            target,
            level,
            internalformat,
            width,
            border,
            imageSize,
            data,
        )
    }
}
#[inline]
pub unsafe fn GetTextureParameterIuiv(texture: GLuint, pname: GLenum, params: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(GetTextureParameterIuivIdx as usize),
        )(texture, pname, params)
    }
}
#[inline]
pub unsafe fn InvalidateTexSubImage(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei) -> (),
        >(*GL_API.get_unchecked(InvalidateTexSubImageIdx as usize))(
            texture, level, xoffset, yoffset, zoffset, width, height, depth,
        )
    }
}
/// Fallbacks: FenceSyncAPPLE
#[inline]
pub unsafe fn FenceSync(condition: GLenum, flags: GLbitfield) -> GLsync {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLbitfield) -> GLsync>(
            *GL_API.get_unchecked(FenceSyncIdx as usize),
        )(condition, flags)
    }
}
/// Fallbacks: VertexAttribL1dEXT
#[inline]
pub unsafe fn VertexAttribL1d(index: GLuint, x: GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLdouble) -> ()>(
            *GL_API.get_unchecked(VertexAttribL1dIdx as usize),
        )(index, x)
    }
}
#[inline]
pub unsafe fn UniformMatrix4x2dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(UniformMatrix4x2dvIdx as usize),
        )(location, count, transpose, value)
    }
}
/// Fallbacks: PauseTransformFeedbackNV
#[inline]
pub unsafe fn PauseTransformFeedback() {
    unsafe {
        mem::transmute::<_, extern "system" fn() -> ()>(
            *GL_API.get_unchecked(PauseTransformFeedbackIdx as usize),
        )()
    }
}
/// Fallbacks: VertexAttrib4ivARB
#[inline]
pub unsafe fn VertexAttrib4iv(index: GLuint, v: *const GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLint) -> ()>(
            *GL_API.get_unchecked(VertexAttrib4ivIdx as usize),
        )(index, v)
    }
}
/// Fallbacks: FramebufferTextureLayerARB, FramebufferTextureLayerEXT
#[inline]
pub unsafe fn FramebufferTextureLayer(
    target: GLenum,
    attachment: GLenum,
    texture: GLuint,
    level: GLint,
    layer: GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint, GLint, GLint) -> ()>(
            *GL_API.get_unchecked(FramebufferTextureLayerIdx as usize),
        )(target, attachment, texture, level, layer)
    }
}
#[inline]
pub unsafe fn TextureSubImage2D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLint,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLenum,
                GLenum,
                *const c_void,
            ) -> (),
        >(*GL_API.get_unchecked(TextureSubImage2DIdx as usize))(
            texture, level, xoffset, yoffset, width, height, format, type_, pixels,
        )
    }
}
#[inline]
pub unsafe fn ColorP4ui(type_: GLenum, color: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(ColorP4uiIdx as usize),
        )(type_, color)
    }
}
#[inline]
pub unsafe fn TexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(TexParameterfvIdx as usize),
        )(target, pname, params)
    }
}
/// Fallbacks: PushDebugGroupKHR
#[inline]
pub unsafe fn PushDebugGroup(source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar) -> ()>(
            *GL_API.get_unchecked(PushDebugGroupIdx as usize),
        )(source, id, length, message)
    }
}
/// Fallbacks: MinSampleShadingARB, MinSampleShadingOES
#[inline]
pub unsafe fn MinSampleShading(value: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(
            *GL_API.get_unchecked(MinSampleShadingIdx as usize),
        )(value)
    }
}
#[inline]
pub unsafe fn BindFragDataLocationIndexed(
    program: GLuint,
    colorNumber: GLuint,
    index: GLuint,
    name: *const GLchar,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, *const GLchar) -> ()>(
            *GL_API.get_unchecked(BindFragDataLocationIndexedIdx as usize),
        )(program, colorNumber, index, name)
    }
}
/// Fallbacks: ScissorIndexedNV
#[inline]
pub unsafe fn ScissorIndexed(
    index: GLuint,
    left: GLint,
    bottom: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLint, GLsizei, GLsizei) -> ()>(
            *GL_API.get_unchecked(ScissorIndexedIdx as usize),
        )(index, left, bottom, width, height)
    }
}
/// Fallbacks: VertexAttrib1dARB, VertexAttrib1dNV
#[inline]
pub unsafe fn VertexAttrib1d(index: GLuint, x: GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLdouble) -> ()>(
            *GL_API.get_unchecked(VertexAttrib1dIdx as usize),
        )(index, x)
    }
}
#[inline]
pub unsafe fn LogicOp(opcode: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(
            *GL_API.get_unchecked(LogicOpIdx as usize),
        )(opcode)
    }
}
/// Fallbacks: GetBooleanIndexedvEXT
#[inline]
pub unsafe fn GetBooleani_v(target: GLenum, index: GLuint, data: *mut GLboolean) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, *mut GLboolean) -> ()>(
            *GL_API.get_unchecked(GetBooleani_vIdx as usize),
        )(target, index, data)
    }
}
/// Fallbacks: GetActiveUniformARB
#[inline]
pub unsafe fn GetActiveUniform(
    program: GLuint,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    size: *mut GLint,
    type_: *mut GLenum,
    name: *mut GLchar,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLuint,
                GLsizei,
                *mut GLsizei,
                *mut GLint,
                *mut GLenum,
                *mut GLchar,
            ) -> (),
        >(*GL_API.get_unchecked(GetActiveUniformIdx as usize))(
            program, index, bufSize, length, size, type_, name,
        )
    }
}
/// Fallbacks: VertexAttrib2fvARB, VertexAttrib2fvNV
#[inline]
pub unsafe fn VertexAttrib2fv(index: GLuint, v: *const GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLfloat) -> ()>(
            *GL_API.get_unchecked(VertexAttrib2fvIdx as usize),
        )(index, v)
    }
}
/// Fallbacks: Uniform4uiEXT
#[inline]
pub unsafe fn Uniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLuint, GLuint, GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(Uniform4uiIdx as usize),
        )(location, v0, v1, v2, v3)
    }
}
#[inline]
pub unsafe fn ProgramUniform3d(
    program: GLuint,
    location: GLint,
    v0: GLdouble,
    v1: GLdouble,
    v2: GLdouble,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLdouble, GLdouble, GLdouble) -> ()>(
            *GL_API.get_unchecked(ProgramUniform3dIdx as usize),
        )(program, location, v0, v1, v2)
    }
}
/// Fallbacks: VertexAttribI1iEXT
#[inline]
pub unsafe fn VertexAttribI1i(index: GLuint, x: GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint) -> ()>(
            *GL_API.get_unchecked(VertexAttribI1iIdx as usize),
        )(index, x)
    }
}
/// Fallbacks: VertexAttribPointerARB
#[inline]
pub unsafe fn VertexAttribPointer(
    index: GLuint,
    size: GLint,
    type_: GLenum,
    normalized: GLboolean,
    stride: GLsizei,
    pointer: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLsizei, *const c_void) -> (),
        >(*GL_API.get_unchecked(VertexAttribPointerIdx as usize))(
            index, size, type_, normalized, stride, pointer,
        )
    }
}
/// Fallbacks: GetUniformLocationARB
#[inline]
pub unsafe fn GetUniformLocation(program: GLuint, name: *const GLchar) -> GLint {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(
            *GL_API.get_unchecked(GetUniformLocationIdx as usize),
        )(program, name)
    }
}
#[inline]
pub unsafe fn CreateFramebuffers(n: GLsizei, framebuffers: *mut GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(
            *GL_API.get_unchecked(CreateFramebuffersIdx as usize),
        )(n, framebuffers)
    }
}
#[inline]
pub unsafe fn BindSamplers(first: GLuint, count: GLsizei, samplers: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(BindSamplersIdx as usize),
        )(first, count, samplers)
    }
}
#[inline]
pub unsafe fn GetProgramResourceIndex(
    program: GLuint,
    programInterface: GLenum,
    name: *const GLchar,
) -> GLuint {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLchar) -> GLuint>(
            *GL_API.get_unchecked(GetProgramResourceIndexIdx as usize),
        )(program, programInterface, name)
    }
}
/// Fallbacks: GetTexParameterIivEXT, GetTexParameterIivOES
#[inline]
pub unsafe fn GetTexParameterIiv(target: GLenum, pname: GLenum, params: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetTexParameterIivIdx as usize),
        )(target, pname, params)
    }
}
/// Fallbacks: GetQueryObjectivARB, GetQueryObjectivEXT
#[inline]
pub unsafe fn GetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetQueryObjectivIdx as usize),
        )(id, pname, params)
    }
}
/// Fallbacks: VertexAttrib4NbvARB
#[inline]
pub unsafe fn VertexAttrib4Nbv(index: GLuint, v: *const GLbyte) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLbyte) -> ()>(
            *GL_API.get_unchecked(VertexAttrib4NbvIdx as usize),
        )(index, v)
    }
}
#[inline]
pub unsafe fn GetString(name: GLenum) -> *const GLubyte {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum) -> *const GLubyte>(
            *GL_API.get_unchecked(GetStringIdx as usize),
        )(name)
    }
}
#[inline]
pub unsafe fn MultiTexCoordP4ui(texture: GLenum, type_: GLenum, coords: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(MultiTexCoordP4uiIdx as usize),
        )(texture, type_, coords)
    }
}
#[inline]
pub unsafe fn ProgramUniformMatrix4dv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble) -> (),
        >(*GL_API.get_unchecked(ProgramUniformMatrix4dvIdx as usize))(
            program, location, count, transpose, value,
        )
    }
}
/// Fallbacks: ColorMaskIndexedEXT, ColorMaskiEXT, ColorMaskiOES
#[inline]
pub unsafe fn ColorMaski(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLboolean, GLboolean, GLboolean, GLboolean) -> (),
        >(*GL_API.get_unchecked(ColorMaskiIdx as usize))(index, r, g, b, a)
    }
}
#[inline]
pub unsafe fn BindFramebuffer(target: GLenum, framebuffer: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(BindFramebufferIdx as usize),
        )(target, framebuffer)
    }
}
#[inline]
pub unsafe fn GetSubroutineUniformLocation(
    program: GLuint,
    shadertype: GLenum,
    name: *const GLchar,
) -> GLint {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint>(
            *GL_API.get_unchecked(GetSubroutineUniformLocationIdx as usize),
        )(program, shadertype, name)
    }
}
#[inline]
pub unsafe fn NamedFramebufferTexture(
    framebuffer: GLuint,
    attachment: GLenum,
    texture: GLuint,
    level: GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLuint, GLint) -> ()>(
            *GL_API.get_unchecked(NamedFramebufferTextureIdx as usize),
        )(framebuffer, attachment, texture, level)
    }
}
/// Fallbacks: SamplerParameterIivEXT, SamplerParameterIivOES
#[inline]
pub unsafe fn SamplerParameterIiv(sampler: GLuint, pname: GLenum, param: *const GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLint) -> ()>(
            *GL_API.get_unchecked(SamplerParameterIivIdx as usize),
        )(sampler, pname, param)
    }
}
#[inline]
pub unsafe fn TexCoordP3ui(type_: GLenum, coords: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(TexCoordP3uiIdx as usize),
        )(type_, coords)
    }
}
/// Fallbacks: FramebufferRenderbufferEXT
#[inline]
pub unsafe fn FramebufferRenderbuffer(
    target: GLenum,
    attachment: GLenum,
    renderbuffertarget: GLenum,
    renderbuffer: GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(FramebufferRenderbufferIdx as usize),
        )(target, attachment, renderbuffertarget, renderbuffer)
    }
}
#[inline]
pub unsafe fn GetProgramResourceName(
    program: GLuint,
    programInterface: GLenum,
    index: GLuint,
    bufSize: GLsizei,
    length: *mut GLsizei,
    name: *mut GLchar,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar) -> (),
        >(*GL_API.get_unchecked(GetProgramResourceNameIdx as usize))(
            program,
            programInterface,
            index,
            bufSize,
            length,
            name,
        )
    }
}
/// Fallbacks: ProgramUniform3uivEXT
#[inline]
pub unsafe fn ProgramUniform3uiv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    value: *const GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(ProgramUniform3uivIdx as usize),
        )(program, location, count, value)
    }
}
/// Fallbacks: CompressedTexSubImage1DARB
#[inline]
pub unsafe fn CompressedTexSubImage1D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    width: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLsizei, *const c_void) -> (),
        >(*GL_API.get_unchecked(CompressedTexSubImage1DIdx as usize))(
            target, level, xoffset, width, format, imageSize, data,
        )
    }
}
#[inline]
pub unsafe fn TextureParameterIuiv(texture: GLuint, pname: GLenum, params: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *const GLuint) -> ()>(
            *GL_API.get_unchecked(TextureParameterIuivIdx as usize),
        )(texture, pname, params)
    }
}
#[inline]
pub unsafe fn UniformMatrix3x2dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(UniformMatrix3x2dvIdx as usize),
        )(location, count, transpose, value)
    }
}
#[inline]
pub unsafe fn GetTextureParameterIiv(texture: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetTextureParameterIivIdx as usize),
        )(texture, pname, params)
    }
}
#[inline]
pub unsafe fn PrimitiveRestartIndex(index: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint) -> ()>(
            *GL_API.get_unchecked(PrimitiveRestartIndexIdx as usize),
        )(index)
    }
}
#[inline]
pub unsafe fn StencilMaskSeparate(face: GLenum, mask: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(StencilMaskSeparateIdx as usize),
        )(face, mask)
    }
}
#[inline]
pub unsafe fn ProgramUniform4d(
    program: GLuint,
    location: GLint,
    v0: GLdouble,
    v1: GLdouble,
    v2: GLdouble,
    v3: GLdouble,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLdouble, GLdouble, GLdouble, GLdouble) -> (),
        >(*GL_API.get_unchecked(ProgramUniform4dIdx as usize))(
            program, location, v0, v1, v2, v3
        )
    }
}
#[inline]
pub unsafe fn DepthRange(near: GLdouble, far: GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLdouble, GLdouble) -> ()>(
            *GL_API.get_unchecked(DepthRangeIdx as usize),
        )(near, far)
    }
}
#[inline]
pub unsafe fn StencilFunc(func: GLenum, ref_: GLint, mask: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, GLuint) -> ()>(
            *GL_API.get_unchecked(StencilFuncIdx as usize),
        )(func, ref_, mask)
    }
}
/// Fallbacks: DrawElementsBaseVertexEXT, DrawElementsBaseVertexOES
#[inline]
pub unsafe fn DrawElementsBaseVertex(
    mode: GLenum,
    count: GLsizei,
    type_: GLenum,
    indices: *const c_void,
    basevertex: GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLint) -> ()>(
            *GL_API.get_unchecked(DrawElementsBaseVertexIdx as usize),
        )(mode, count, type_, indices, basevertex)
    }
}
/// Fallbacks: Uniform4ivARB
#[inline]
pub unsafe fn Uniform4iv(location: GLint, count: GLsizei, value: *const GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, *const GLint) -> ()>(
            *GL_API.get_unchecked(Uniform4ivIdx as usize),
        )(location, count, value)
    }
}
/// Fallbacks: ProgramUniform1fEXT
#[inline]
pub unsafe fn ProgramUniform1f(program: GLuint, location: GLint, v0: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLfloat) -> ()>(
            *GL_API.get_unchecked(ProgramUniform1fIdx as usize),
        )(program, location, v0)
    }
}
/// Fallbacks: VertexAttribI3uivEXT
#[inline]
pub unsafe fn VertexAttribI3uiv(index: GLuint, v: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttribI3uivIdx as usize),
        )(index, v)
    }
}
#[inline]
pub unsafe fn CompressedTextureSubImage2D(
    texture: GLuint,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLuint,
                GLint,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLenum,
                GLsizei,
                *const c_void,
            ) -> (),
        >(*GL_API.get_unchecked(CompressedTextureSubImage2DIdx as usize))(
            texture, level, xoffset, yoffset, width, height, format, imageSize, data,
        )
    }
}
/// Fallbacks: BlitFramebufferEXT, BlitFramebufferNV
#[inline]
pub unsafe fn BlitFramebuffer(
    srcX0: GLint,
    srcY0: GLint,
    srcX1: GLint,
    srcY1: GLint,
    dstX0: GLint,
    dstY0: GLint,
    dstX1: GLint,
    dstY1: GLint,
    mask: GLbitfield,
    filter: GLenum,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLint,
                GLbitfield,
                GLenum,
            ) -> (),
        >(*GL_API.get_unchecked(BlitFramebufferIdx as usize))(
            srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter,
        )
    }
}
/// Fallbacks: BeginQueryARB
#[inline]
pub unsafe fn BeginQuery(target: GLenum, id: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint) -> ()>(
            *GL_API.get_unchecked(BeginQueryIdx as usize),
        )(target, id)
    }
}
#[inline]
pub unsafe fn UniformMatrix3dv(
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLdouble,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble) -> ()>(
            *GL_API.get_unchecked(UniformMatrix3dvIdx as usize),
        )(location, count, transpose, value)
    }
}
#[inline]
pub unsafe fn DisableVertexArrayAttrib(vaobj: GLuint, index: GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLuint) -> ()>(
            *GL_API.get_unchecked(DisableVertexArrayAttribIdx as usize),
        )(vaobj, index)
    }
}
/// Fallbacks: VertexAttrib4fARB, VertexAttrib4fNV
#[inline]
pub unsafe fn VertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(
            *GL_API.get_unchecked(VertexAttrib4fIdx as usize),
        )(index, x, y, z, w)
    }
}
/// Fallbacks: ObjectLabelKHR
#[inline]
pub unsafe fn ObjectLabel(identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar) -> ()>(
            *GL_API.get_unchecked(ObjectLabelIdx as usize),
        )(identifier, name, length, label)
    }
}
#[inline]
pub unsafe fn MultiTexCoordP3uiv(texture: GLenum, type_: GLenum, coords: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLuint) -> ()>(
            *GL_API.get_unchecked(MultiTexCoordP3uivIdx as usize),
        )(texture, type_, coords)
    }
}
#[inline]
pub unsafe fn GetNamedFramebufferParameteriv(
    framebuffer: GLuint,
    pname: GLenum,
    param: *mut GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetNamedFramebufferParameterivIdx as usize),
        )(framebuffer, pname, param)
    }
}
/// Fallbacks: EndQueryARB
#[inline]
pub unsafe fn EndQuery(target: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum) -> ()>(
            *GL_API.get_unchecked(EndQueryIdx as usize),
        )(target)
    }
}
#[inline]
pub unsafe fn ProgramUniform1d(program: GLuint, location: GLint, v0: GLdouble) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLint, GLdouble) -> ()>(
            *GL_API.get_unchecked(ProgramUniform1dIdx as usize),
        )(program, location, v0)
    }
}
#[inline]
pub unsafe fn VertexAttribP3uiv(
    index: GLuint,
    type_: GLenum,
    normalized: GLboolean,
    value: *const GLuint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint) -> ()>(
            *GL_API.get_unchecked(VertexAttribP3uivIdx as usize),
        )(index, type_, normalized, value)
    }
}
#[inline]
pub unsafe fn GetInternalformativ(
    target: GLenum,
    internalformat: GLenum,
    pname: GLenum,
    bufSize: GLsizei,
    params: *mut GLint,
) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetInternalformativIdx as usize),
        )(target, internalformat, pname, bufSize, params)
    }
}
#[inline]
pub unsafe fn ClearBufferSubData(
    target: GLenum,
    internalformat: GLenum,
    offset: GLintptr,
    size: GLsizeiptr,
    format: GLenum,
    type_: GLenum,
    data: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLenum,
                GLintptr,
                GLsizeiptr,
                GLenum,
                GLenum,
                *const c_void,
            ) -> (),
        >(*GL_API.get_unchecked(ClearBufferSubDataIdx as usize))(
            target,
            internalformat,
            offset,
            size,
            format,
            type_,
            data,
        )
    }
}
/// Fallbacks: BeginConditionalRenderNV
#[inline]
pub unsafe fn BeginConditionalRender(id: GLuint, mode: GLenum) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>(
            *GL_API.get_unchecked(BeginConditionalRenderIdx as usize),
        )(id, mode)
    }
}
/// Fallbacks: DrawArraysEXT
#[inline]
pub unsafe fn DrawArrays(mode: GLenum, first: GLint, count: GLsizei) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLenum, GLint, GLsizei) -> ()>(
            *GL_API.get_unchecked(DrawArraysIdx as usize),
        )(mode, first, count)
    }
}
#[inline]
pub unsafe fn TexImage2D(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(
                GLenum,
                GLint,
                GLint,
                GLsizei,
                GLsizei,
                GLint,
                GLenum,
                GLenum,
                *const c_void,
            ) -> (),
        >(*GL_API.get_unchecked(TexImage2DIdx as usize))(
            target,
            level,
            internalformat,
            width,
            height,
            border,
            format,
            type_,
            pixels,
        )
    }
}
#[inline]
pub unsafe fn DeleteSamplers(count: GLsizei, samplers: *const GLuint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLsizei, *const GLuint) -> ()>(
            *GL_API.get_unchecked(DeleteSamplersIdx as usize),
        )(count, samplers)
    }
}
/// Fallbacks: ProgramUniformMatrix2x4fvEXT
#[inline]
pub unsafe fn ProgramUniformMatrix2x4fv(
    program: GLuint,
    location: GLint,
    count: GLsizei,
    transpose: GLboolean,
    value: *const GLfloat,
) {
    unsafe {
        mem::transmute::<
            _,
            extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat) -> (),
        >(*GL_API.get_unchecked(ProgramUniformMatrix2x4fvIdx as usize))(
            program, location, count, transpose, value,
        )
    }
}
#[inline]
pub unsafe fn GetTransformFeedbackiv(xfb: GLuint, pname: GLenum, param: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetTransformFeedbackivIdx as usize),
        )(xfb, pname, param)
    }
}
#[inline]
pub unsafe fn GetFragDataIndex(program: GLuint, name: *const GLchar) -> GLint {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, *const GLchar) -> GLint>(
            *GL_API.get_unchecked(GetFragDataIndexIdx as usize),
        )(program, name)
    }
}
#[inline]
pub unsafe fn GetProgramPipelineiv(pipeline: GLuint, pname: GLenum, params: *mut GLint) {
    unsafe {
        mem::transmute::<_, extern "system" fn(GLuint, GLenum, *mut GLint) -> ()>(
            *GL_API.get_unchecked(GetProgramPipelineivIdx as usize),
        )(pipeline, pname, params)
    }
}

static mut GL_API: [usize; 695] = [0; 695];

static LOAD_DESC: &[(u16, &[u8])] = &[
    // Program functions
    (CreateProgramIdx, b"glCreateProgram\0"),
    (DeleteProgramIdx, b"glDeleteProgram\0"),
    (AttachShaderIdx, b"glAttachShader\0"),
    (LinkProgramIdx, b"glLinkProgram\0"),
    (GetProgramInfoLogIdx, b"glGetProgramInfoLog\0"),
    (ValidateProgramIdx, b"glValidateProgram\0"),
    (GetProgramivIdx, b"glGetProgramiv\0"),
    (UseProgramIdx, b"glUseProgram\0"),
    (Uniform1iIdx, b"glUniform1i\0"),
    // Shader functions
    (CreateShaderIdx, b"glCreateShader\0"),
    (DeleteShaderIdx, b"glDeleteShader\0"),
    (ShaderSourceIdx, b"glShaderSource\0"),
    (CompileShaderIdx, b"glCompileShader\0"),
    (GetShaderInfoLogIdx, b"glGetShaderInfoLog\0"),
    (GetShaderivIdx, b"glGetShaderiv\0"),
    // Vertex Buffer Object functions
    (GenBuffersIdx, b"glGenBuffers\0"),
    (DeleteBuffersIdx, b"glDeleteBuffers\0"),
    (BindBufferIdx, b"glBindBuffer\0"),
    (BindBufferBaseIdx, b"glBindBufferBase\0"),
    (BufferDataIdx, b"glBufferData\0"),
    // Vertex Array Object functions
    (GenVertexArraysIdx, b"glGenVertexArrays\0"),
    (DeleteVertexArraysIdx, b"glDeleteVertexArrays\0"),
    (BindVertexArrayIdx, b"glBindVertexArray\0"),
    (EnableVertexAttribArrayIdx, b"glEnableVertexAttribArray\0"),
    (VertexAttribPointerIdx, b"glVertexAttribPointer\0"),
    (DrawArraysIdx, b"glDrawArrays\0"),
    (DrawArraysInstancedIdx, b"glDrawArraysInstanced\0"),
    // Framebuffer
    (GenFramebuffersIdx, b"glGenFramebuffers\0"),
    (DeleteFramebuffersIdx, b"glDeleteFramebuffers\0"),
    (FramebufferTexture2DIdx, b"glFramebufferTexture2D\0"),
    (BindFramebufferIdx, b"glBindFramebuffer\0"),
    (DrawBuffersIdx, b"glDrawBuffers\0"),
    (CheckFramebufferStatusIdx, b"glCheckFramebufferStatus\0"),
    // Texture
    (GenTexturesIdx, b"glGenTextures\0"),
    (DeleteTexturesIdx, b"glDeleteTextures\0"),
    (BindTextureIdx, b"glBindTexture\0"),
    (ActiveTextureIdx, b"glActiveTexture\0"),
    (TexImage2DIdx, b"glTexImage2D\0"),
    (TexParameteriIdx, b"glTexParameteri\0"),
    // Misc
    (ClearBufferfvIdx, b"glClearBufferfv\0"),
    (ViewportIndexedfIdx, b"glViewportIndexedf\0"),
    (EnableIdx, b"glEnable\0"),
    (GetErrorIdx, b"glGetError\0"),
    (DebugMessageCallbackIdx, b"glDebugMessageCallback\0"),
    (ScissorIdx, b"glScissor\0"),
    (BlendFuncIdx, b"glBlendFunc\0"),
    (DisableIdx, b"glDisable\0"),
    // Get
    (GetInteger64vIdx, b"glGetInteger64v\0"),
    // Querys
    //(GenQueriesIdx, b"glGenQueries\0"),
    //(DeleteQueriesIdx, b"glDeleteQueries\0"),
    //(QueryCounterIdx, b"glQueryCounter\0"),
    //(BeginQueryIdx, b"glBeginQuery\0"),
    //(EndQueryIdx, b"glEndQuery\0"),
    //(GetQueryObjectui64vIdx, b"glGetQueryObjectui64v\0"),
];

#[inline]
pub fn init() {
    for &(index, name) in LOAD_DESC {
        unsafe {
            (*GL_API.get_unchecked_mut(index as usize)) =
                win32::wgl_get_proc_address(name) as usize;
        }
    }
}
