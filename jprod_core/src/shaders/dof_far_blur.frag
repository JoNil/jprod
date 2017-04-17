#version 450 core

#define NUM_OF_TAPS 8

in vec2 frag_uv;

layout(location = 0) out vec4 color;

layout(location = 2) uniform sampler2D color_tex;

const float tap_length = float(NUM_OF_TAPS - 1);

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
    vec2 step_size = vec2(1.0) / textureSize(color_tex, 0);
    vec4 frag_color = texture(color_tex, frag_uv);
    float far_coc = clamp(frag_color.w, 0.0, 1.0);
    vec4 color_sum = vec4(0.0);

    if (far_coc > 0.0) {
        
        for (int y = 0; y < NUM_OF_TAPS; ++y) {

            for (int x = 0; x < NUM_OF_TAPS; ++x) {

                vec2 uv_offset = to_unit_disk(vec2(float(x), float(y)) / vec2(tap_length));
                vec2 uv_scale = step_size * tap_length;

                vec4 color_sample = texture(color_tex, frag_uv + (uv_offset * uv_scale));
                color_sum += color_sample;
            }
        }
    }

    color = vec4(color_sum.xyz / vec3(color_sum.w + 1e-006), color_sum.w / float(NUM_OF_TAPS * NUM_OF_TAPS));
}