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

float w0(float a) {
  return 0.166667 * (a * ((a * ((-a) + 3.0)) - 3.0) + 1.0);
}

float w1(float a) {
  return 0.166667 * (((a * a) * ((3.0 * a) - 6.0)) + 4.0);
}

float w2(float a) {
  return 0.166667 * (a * ((a * ((-3.0 * a) + 3.0)) + 3.0) + 1.0);
}

float w3(float a) {
  return 0.166667 * ((a * a) * a);
}

float g0(float a) {
  return w0(a) + w1(a);
}

float g1(float a) {
  return w2(a) + w3(a);
}

float h0(float a) {
  float w1a = w1(a);
  float w0a = w0(a);
  return -1.0 + (w1a / (w0a + w1a));
}

float h1(float a) {
  float w3a = w3(a);
  float w2a = w2(a);
  return 1.0 + (w3a / (w2a + w3a));
}

vec4 textureBicubic(sampler2D tex, vec2 uv, vec2 size) {

  vec2 uv_biased = (uv * size) + vec2(0.5);
  vec2 iuv = floor(uv_biased);
  vec2 fuv = fract(uv_biased);
  float g0x = g0(fuv.x);
  float g1x = g1(fuv.x);
  float h0x = h0(fuv.x);
  float h1x = h1(fuv.x);
  float h0y = h0(fuv.y);
  float h1y = h1(fuv.y);
  float g0y = g0(fuv.y);
  float g1y = g1(fuv.y);
  vec2 a = (vec2(iuv.x + h0x, iuv.y + h0y) - vec2(0.5)) / size;
  vec2 b = (vec2(iuv.x + h1x, iuv.y + h0y) - vec2(0.5)) / size;
  vec2 c = (vec2(iuv.x + h0x, iuv.y + h1y) - vec2(0.5)) / size;
  vec2 d = (vec2(iuv.x + h1x, iuv.y + h1y) - vec2(0.5)) / size;
  vec4 as = texture(tex, a);
  vec4 bs = texture(tex, b);
  vec4 cs = texture(tex, c);
  vec4 ds = texture(tex, d);
  return (((as * g0x) + (bs * g1x)) * g0y) + (((cs * g0x) + (ds * g1x)) * g1y);
}

void main()
{
    float dof_z = 1.0;

    vec4 orig_color = texture(color_tex, frag_uv);

    vec4 depth_sample = texture(depth_tex, frag_uv);
    float depth =
            depth_sample.x * 255.0 / 256.0 +
            depth_sample.y * 255.0 / 65536.0 +
            depth_sample.z * 255.0 / 16777216.0 +
            depth_sample.w * 255.0 / 4294967296.0;

    float dist = depth_to_distance(depth);

    if (dist >= plane_in_focus) {
        float farfield_bias = 1.0;
        float coc_far = clamp(((dist - plane_in_focus) - farfield_bias) / ((dof_z - plane_in_focus) - farfield_bias), 0.0, 1.0);
        float blend_ratio = 10.0;
        vec4 dof_sample = textureBicubic(far_blur_tex, frag_uv, textureSize(color_tex, 0));
        float dof_scale = clamp(sqrt(coc_far * blend_ratio), 0.0, 1.0);
        float orig_scale = clamp(1.0 - sqrt(coc_far * blend_ratio), 0.0, 1.0);
        color = orig_scale * orig_color + vec4(dof_sample.xyz * dof_scale, 1.0);
    } else {
        color = orig_color;
    }
}