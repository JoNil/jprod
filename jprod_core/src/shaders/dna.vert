#version 450 core

layout(location = 0) in vec3 vertex_pos;
layout(location = 1) in vec3 vertex_normal;

out vec3 model_pos;
out vec3 frag_pos;
out vec3 frag_normal;

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

    mat4 transform = m[gl_InstanceID] * rot;
    mat3 normal_transform = mat3(transpose(inverse(transform)));
    
    vec4 pos = transform * vec4(vertex_pos, 1.0);
    vec3 normal = normal_transform * vertex_normal;

    model_pos = vertex_pos.xyz;
    frag_pos = pos.xyz;
    frag_normal = normal;
    gl_Position = vp * pos;
}