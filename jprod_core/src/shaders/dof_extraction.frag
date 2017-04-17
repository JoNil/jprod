#version 450 core

in vec2 frag_uv;

layout(location = 0) out vec4 color;

layout(location = 2) uniform sampler2D color_tex;
layout(location = 3) uniform sampler2D depth_tex;

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
    vec4 frag_color = texture(color_tex, frag_uv);

    vec4 depth_sample = texture(depth_tex, frag_uv);
    float depth =
            depth_sample.x * 255.0 / 256.0 +
            depth_sample.y * 255.0 / 65536.0 +
            depth_sample.z * 255.0 / 16777216.0 +
            depth_sample.w * 255.0 / 4294967296.0;

    float dist = depth_to_distance(depth);

    if (dist > plane_in_focus) {
        color = vec4(frag_color.xyz, 1.0);
    } else {
        color = vec4(0.0, 0.0, 0.0, 0.0);
    }
}