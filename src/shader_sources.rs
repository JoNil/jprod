
include!(concat!(env!("OUT_DIR"), "/shader_ids.rs"));

pub struct ShaderSource {
    pub vertex_source: &'static [u8],
    pub vertex_path: &'static [u8],
    pub fragment_source: &'static [u8],
    pub fragment_path: &'static [u8],
}

include!(concat!(env!("OUT_DIR"), "/shader_source.rs"));