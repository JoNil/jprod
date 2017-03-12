#version 450 core

#define NUM_OF_TAPS 8

in vec2 frag_uv;

layout(location = 0) out vec4 color;

layout(location = 2) uniform sampler2D color_tex;

layout(std430, binding = 0) buffer uniforms
{
    float plane_in_focus;
};

/*void main() {
  int* y;
  int* x;
  float2* tap;

  float2 _604_ = screenPosToTexcoord(gl_FragCoord.xy, freqHigh_fragmentUniforms.renderpositiontoviewtexture);
  float2* texcoord = _604_ * float2(1.0f / freqLow_fragmentUniforms.resolutionscale.x, 1.0f / freqLow_fragmentUniforms.resolutionscale.y);
  int* NUM_TAPS_SIDE = 8;
  float4 _626_ = tex2Dlod(samp_tex0, float4(texcoord, 0.0f, 0.0f));
  float* farCOC = saturate(_626_.w);
  float4* accFar = 0.0f.xxxx;
  if(farCOC > 0.0f) {
    y = 0;
    while(y < NUM_TAPS_SIDE) {
      x = 0;
      while(x < NUM_TAPS_SIDE) {
        tap = float2(ConvertSToF(x), ConvertSToF(y)) / float2(ConvertSToF(NUM_TAPS_SIDE - 1), ConvertSToF(NUM_TAPS_SIDE - 1));
        tap = ToUnitDisk(tap);
        float2 a = ((freqHigh_fragmentUniforms.texsize.zw * 7.0f) * farCOC) * freqHigh_fragmentUniforms.dofparms.w;
        float4 _694_ = tex2Dlod(samp_tex0, float4(texcoord + (tap * _684_), 0.0f, 0.0f));
        float _701_ = saturate(1.0f - (farCOC - far.w));
        accFar = accFar + (_694_ * _701_);
        x++;
      }
      y++;
    }
  }
  out_FragColor0 = float4((accFar.xyz / float3(accFar.w + 1e-006f)).xyz, out_FragColor0.w);
  out_FragColor0.w = accFar.w / ConvertSToF(NUM_TAPS_SIDE * NUM_TAPS_SIDE);
} // main*/

const float bokeh_scale = 1.0; 
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

                vec2 uv_scale = (step_size * tap_length) * bokeh_scale;
                vec4 color_sample = texture(color_tex, frag_uv + (uv_offset * uv_scale));
                color_sum += color_sample;
            }
        }
    }

    color = vec4(color_sum.xyz / vec3(color_sum.w + 1e-006), color_sum.w / float(NUM_OF_TAPS * NUM_OF_TAPS));
}