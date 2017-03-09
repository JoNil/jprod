#version 450 core

in vec2 frag_uv;

layout(location = 0) out vec4 color;

layout(location = 2) uniform sampler2D color_tex;
layout(location = 3) uniform sampler2D bloom_color_tex;

void main()
{
    vec4 color1 = texture(color_tex, frag_uv);
    vec4 color2 = texture(bloom_color_tex, frag_uv);

    color = vec4((color1.rgb + color2.rgb)/2.0, color1.a);
}