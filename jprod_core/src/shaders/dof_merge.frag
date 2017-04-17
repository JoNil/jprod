#version 450 core

#define NUM_OF_TAPS 8

in vec2 frag_uv;

layout(location = 0) out vec4 color;

layout(location = 2) uniform sampler2D color_tex;
layout(location = 3) uniform sampler2D far_blur_tex;
layout(location = 4) uniform sampler2D depth_tex;

layout(std430, binding = 0) buffer uniforms
{
    float z_near;
    float z_far;
    float plane_in_focus;
};

float depth_to_distance(float depth)
{
    return (z_near * z_far) / (z_far - depth * (z_far - z_near));
}

void main()
{
    float distance_to_fade = 0.75;

    vec4 orig_color = texture(color_tex, frag_uv);

    vec4 depth_sample = texture(depth_tex, frag_uv);
    float depth =
            depth_sample.x * 255.0 / 256.0 +
            depth_sample.y * 255.0 / 65536.0 +
            depth_sample.z * 255.0 / 16777216.0 +
            depth_sample.w * 255.0 / 4294967296.0;

    float dist = depth_to_distance(depth);

    if (dist >= plane_in_focus) {
        float coc_far = clamp(((dist - plane_in_focus)) / ((distance_to_fade - plane_in_focus)), 0.0, 1.0);
        vec4 dof_sample = texture(far_blur_tex, frag_uv);
        float dof_scale = clamp(sqrt(coc_far), 0.0, 1.0);
        float orig_scale = clamp(1.0 - sqrt(coc_far), 0.0, 1.0);
        color = orig_scale * orig_color + vec4(dof_sample.xyz * dof_scale, 1.0);
    } else {
        color = orig_color;
    }
}