use win32::types::Filetime;

include!(concat!(env!("OUT_DIR"), "/shader_ids.rs"));

pub struct ShaderSource {
    pub vertex_source: &'static [u8],
    pub vertex_path: &'static [u8],
    pub vertex_filetime: Filetime,
    pub fragment_source: &'static [u8],
    pub fragment_path: &'static [u8],
    pub fragment_filetime: Filetime,
}

include!(concat!(env!("OUT_DIR"), "/shader_source.rs"));