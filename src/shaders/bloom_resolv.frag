#version 450 core

in vec2 frag_uv;

layout(location = 0) out vec4 color;

layout(location = 2) uniform sampler2D color_tex;
layout(location = 3) uniform sampler2D bright_color_tex;

void main()
{
    color = texture(bright_color_tex, frag_uv);
}