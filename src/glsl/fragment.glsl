#version 140
#define PI 3.1415926538

out vec4 color;
in vec2 texCoord;

uniform float Time;
uniform sampler2D Texture;

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

vec3 getBaseColor() {
    vec3 initialColor = vec3(1, 0, 1);
    return hueShift(initialColor, mix(0, PI, sinTimeNormalized()));
}

mat4 getTexelData(ivec2 position) {
    // this looks out of order, but it's correct
    return mat4(
        texelFetch(Texture, position + ivec2(0, 1), 0),
        texelFetch(Texture, position + ivec2(1, 1), 0),
        texelFetch(Texture, position, 0),
        texelFetch(Texture, position + ivec2(1, 0), 0)
    );
}

ivec2 getFramePosition(int width, int frameNumber) {
    // if frame number * 2 is longer than the width, we need to wrap around
    int vertical = frameNumber * 2 % width;
    int horizontal = (frameNumber * 2 / width) % width;
    return ivec2(vertical, horizontal);
}


float bassLayer(vec2 uv, float value) {
    return value;
}
 
void main() {
    ivec2 position = getFramePosition(512, int(Time/100));
    mat4 texels = getTexelData(position);


    color.r = bassLayer(texCoord, texels[3].g);
}
