#version 450 core

in vec2 frag_uv;

layout(location = 0) out vec4 color;

layout(location = 2) uniform sampler2D color_tex;

const float weight[5] = float[5] (0.227027, 0.1945946, 0.1216216, 0.054054, 0.016216);

void main()
{
    vec2 tex_offset = 1.0 / textureSize(color_tex, 0);
    vec3 result = texture(color_tex, frag_uv).rgb * weight[0];

    for (int i = 1; i < 5; ++i) {
        result += texture(color_tex, frag_uv + vec2(tex_offset.x * i, 0.0)).rgb * weight[i];
        result += texture(color_tex, frag_uv - vec2(tex_offset.x * i, 0.0)).rgb * weight[i];
    }

    color = vec4(result, 1.0);
}