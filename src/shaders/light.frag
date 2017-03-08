#version 450 core

in vec2 frag_uv;

layout(location = 0) out vec4 color;

layout(location = 2) uniform sampler2D color_tex;
layout(location = 3) uniform sampler2D pos_tex;
layout(location = 4) uniform sampler2D normal_tex;

layout(std430, binding = 0) buffer uniforms
{
    vec4 eye_pos;
};

void main()
{
    vec3 light_pos = vec3(1.0, 1.0, 1.0);

    vec3 material_color = texture(color_tex, frag_uv).rgb;
    vec3 pos = texture(pos_tex, frag_uv).xyz;
    vec3 normal = normalize(texture(normal_tex, frag_uv).xyz);

    vec3 light_dir = normalize(light_pos - pos);

    float material_roughness = 0.3; // 0 : smooth, 1: rough
    float F0 = 0.3;                 // fresnel reflectance at normal incidence
    float k = 0.4;                  // fraction of diffuse reflection (specular reflection = 1 - k)
    vec3 light_color = vec3(0.9, 0.1, 0.1);

    float dot_n_l = max(dot(normal, light_dir), 0.0);

    float specular = 0.0;
    if (dot_n_l > 0.0) {
        vec3 eye_dir = normalize(eye_pos.xyz - pos);

        vec3 half_vector = normalize(light_dir + eye_dir);
        float dot_n_h = max(dot(normal, half_vector), 0.0); 
        float dot_n_v = max(dot(normal, eye_dir), 0.0);
        float dot_v_h = max(dot(eye_dir, half_vector), 0.0);
        float material_roughness_2 = material_roughness * material_roughness;
        
        // geometric attenuation
        float nh2 = 2.0 * dot_n_h;
        float g1 = (nh2 * dot_n_v) / dot_v_h;
        float g2 = (nh2 * dot_n_l) / dot_v_h;
        float geo_att = min(1.0, min(g1, g2));
     
        // roughness (or: microfacet distribution function)
        // beckmann distribution function
        float r1 = 1.0 / (4.0 * material_roughness_2 * pow(dot_n_h, 4.0));
        float r2 = (dot_n_h * dot_n_h - 1.0) / (material_roughness_2 * dot_n_h * dot_n_h);
        float roughness = r1 * exp(r2);
        
        // fresnel
        // Schlick approximation
        float fresnel = F0 + (1.0 - F0) * pow(1.0 - dot_v_h, 5.0);
        
        specular = (fresnel * geo_att * roughness) / (dot_n_v * dot_n_l * 3.1415976536);
    }
    
    color = vec4(light_color * dot_n_l * (k + specular * (1.0 - k)), 1.0);
    //color = normalize(texture(normal_tex, frag_uv));
}