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
    frag_uv = vertex_pos.xy/2.0 + 0.5;

    gl_Position = mvp[gl_InstanceID] * vec4(vertex_pos.x + sin(time), vertex_pos.yz, 1.0);
}