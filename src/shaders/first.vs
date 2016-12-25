#version 450 core

layout(location = 0) in vec3 vertex_pos;

out vec2 frag_uv;


layout(std430, binding = 0) buffer instance_data
{
    mat4 mvp[];
};

layout(std430, binding = 1) buffer uniforms
{
    float time;
};

void main()
{
    mat4 rot = mat4(
        sin(time), cos(time), 0.0, 0.0,
        cos(time), -sin(time), 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0);
    
    vec4 rotated_vec = rot * vec4(vertex_pos, 1.0);

    frag_uv = rotated_vec.xy/2.0 + 0.5;

    gl_Position = mvp[gl_InstanceID] * rotated_vec;
}