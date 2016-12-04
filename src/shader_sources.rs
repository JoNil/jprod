pub enum ShaderId {

}

struct ShaderSource {
    source: &[u8],
    path: &[u8],
}

pub fn get_shader_source(id: ShaderId) -> &[u8] {
    b"\0"
}