#version 450 core

in vec3 model_pos;
in vec3 frag_pos;
in vec3 frag_normal;

layout(location = 0) out vec3 color;
layout(location = 1) out vec3 pos;
layout(location = 2) out vec3 normal;

void main()
{
    color = model_pos;
    pos = frag_pos;
    normal = frag_normal;
}