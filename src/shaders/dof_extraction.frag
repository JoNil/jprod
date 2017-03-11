#version 450 core

in vec2 frag_uv;

layout(location = 0) out vec4 color;

layout(location = 2) uniform sampler2D color_tex;
layout(location = 3) uniform sampler2D pos_tex;

layout(std430, binding = 0) buffer uniforms
{
    vec4 eye_pos;
    float plane_in_focus;
};

void main()
{
    vec4 frag_color = texture(color_tex, frag_uv);

    vec3 pos = texture(pos_tex, frag_uv).xyz;

    float dist = length(eye_pos.xyz - pos);

    if (dist > plane_in_focus) {
        color = frag_color;
    }
}