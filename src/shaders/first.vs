#version 450 core

layout(location = 0) in vec3 vertex_pos;

out vec2 frag_uv;


//layout(std140, binding = 0) uniform Uniforms
//{
//	float time;
//};

layout(std430, binding = 0) buffer Uniforms
{
    mat4 mvp[];
};

void main()
{
    frag_uv = vertex_pos.xy/2.0 + 0.5;

    gl_Position = mvp[gl_InstanceID] * vec4(vertex_pos, 1.0);
}