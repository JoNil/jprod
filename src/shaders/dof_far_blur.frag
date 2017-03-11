#version 450 core

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
        float2 _684_ = ((freqHigh_fragmentUniforms.texsize.zw * 7.0f) * farCOC) * freqHigh_fragmentUniforms.dofparms.w;
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

void main()
{
    vec4 frag_color = texture(color_tex, frag_uv);

    color = frag_color;
}