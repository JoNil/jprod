#version 450 core

in vec2 frag_uv;

layout(location = 0) out vec4 color;
layout(location = 1) out vec4 bright_color;

layout(location = 2) uniform sampler2D color_tex;

void main()
{
    vec4 frag_color = texture(color_tex, frag_uv);
    color = frag_color;

    float brightness = dot(frag_color.rgb, vec3(0.213, 0.715, 0.072));
    if (brightness > 1.0) {
        bright_color = frag_color;
    }
}