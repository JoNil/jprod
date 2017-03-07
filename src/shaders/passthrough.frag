#version 450 core

in vec2 frag_uv;

layout(location = 0) out vec4 color;

layout(location = 2) uniform sampler2D color_tex;
layout(location = 3) uniform sampler2D pos_tex;
layout(location = 4) uniform sampler2D normal_tex;

void main()
{
    color = texture(color_tex, frag_uv);
}