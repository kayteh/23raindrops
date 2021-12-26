#version 140
#define PI 3.1415926538

out vec4 color;
in vec2 texCoord;

uniform float Time;

vec3 float2vec3(float f) {
    return vec3(f, f, f);
}

float sinTimeNormalized() {
    return sin(Time/1000) - 0.5 * 2;
}

vec3 hueShift( vec3 color, float hueAdjust ){

    const vec3  kRGBToYPrime = vec3 (0.299, 0.587, 0.114);
    const vec3  kRGBToI      = vec3 (0.596, -0.275, -0.321);
    const vec3  kRGBToQ      = vec3 (0.212, -0.523, 0.311);

    const vec3  kYIQToR     = vec3 (1.0, 0.956, 0.621);
    const vec3  kYIQToG     = vec3 (1.0, -0.272, -0.647);
    const vec3  kYIQToB     = vec3 (1.0, -1.107, 1.704);

    float   YPrime  = dot (color, kRGBToYPrime);
    float   I       = dot (color, kRGBToI);
    float   Q       = dot (color, kRGBToQ);
    float   hue     = atan (Q, I);
    float   chroma  = sqrt (I * I + Q * Q);

    hue += hueAdjust;

    Q = chroma * sin (hue);
    I = chroma * cos (hue);

    vec3    yIQ   = vec3 (YPrime, I, Q);

    return vec3( dot (yIQ, kYIQToR), dot (yIQ, kYIQToG), dot (yIQ, kYIQToB) );

}

vec3 getBaseColor(vec2 uv) {
    vec3 initialColor = vec3(1, 0, 1);
    return hueShift(initialColor, mix(0, PI, sinTimeNormalized()));
}
 
void main() {
    float distanceFromCenter = distance(vec2(0.5), texCoord);
    float mirrorDimension = distanceFromCenter * 2.0;
    float ring = 0;
    float ringPower = abs(mirrorDimension - mix(0.2, 0.05, sinTimeNormalized()));
    if (ringPower > mix(0, 0.15, sinTimeNormalized())) {
        ring = ringPower * 10;
    }

    vec3 outStencil = float2vec3(ring);

    vec3 baseColor = clamp(getBaseColor(vec2(mirrorDimension)), 0.0, 1.0);
    color = vec4(baseColor * (1-outStencil), 1.0);
}