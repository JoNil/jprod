#version 450 core

layout(location = 0) in vec3 vertex_pos;

out vec2 frag_uv;

void main()
{   
    vec4 pos = vec4(vertex_pos, 1.0);

    frag_uv = pos.xy/2.0 + 0.5;

    gl_Position = pos;
}