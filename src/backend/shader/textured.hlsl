struct VsOutput {
    float4 pos: SV_Position;
    float2 tc: TEXCOORD;
};

cbuffer Locals {
	float4x4 u_Transform;
};

VsOutput Vertex(float2 pos : a_Pos, float2 tc: a_TexCoord) {
    VsOutput output = {
        mul(u_Transform, float4(pos, 0.0, 1.0)),
        tc
    };
    return output;
}

Texture2D<float4> t_Color;
SamplerState t_Color_;

float4 Pixel(VsOutput pin) : SV_Target {
    return t_Color.Sample(t_Color_, pin.tc);
}
