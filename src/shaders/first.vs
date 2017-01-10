#version 450 core

layout(location = 0) in vec3 vertex_pos;

out vec2 frag_uv;
out vec3 frag_pos;

layout(std430, binding = 0) buffer uniforms
{
    mat4 vp;
    float time;
};

layout(std430, binding = 1) buffer instance_data
{
    mat4 m[];
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

    vec4 pos = vp * m[gl_InstanceID] * rotated_vec;

    frag_pos = pos.xyz;
    gl_Position = pos;
}