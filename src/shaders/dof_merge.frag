#version 450 core

#define NUM_OF_TAPS 8

in vec2 frag_uv;

layout(location = 0) out vec4 color;

layout(location = 2) uniform sampler2D color_tex;
layout(location = 3) uniform sampler2D far_blur_tex;

vec2 to_unit_disk(vec2 uv)
{
    float r = 0.0;
    float phi = 0.0;
    float a = (2.0 * uv.x) - 1.0;
    float b = (2.0 * uv.y) - 1.0;

    if (abs(a) > abs(b)) {
        r = a;
        phi = 0.785398 * (b / (a + 1e-006));
    } else {
        r = b;
        phi = 1.5708 - (0.785398 * (a / (b + 1e-006)));
    }

    return vec2(r * cos(phi), r * sin(phi));
}

void main()
{
    vec4 color1 = texture(color_tex, frag_uv);
    vec4 color2 = texture(far_blur_tex, frag_uv);
    color = color2;
}