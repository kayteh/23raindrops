#version 140

out vec4 color;
in vec2 texCoord;

vec3 float2vec3(highp float f) {
    return vec3(f, f, f);
}

void main() {
    float distanceFromCenter = distance(vec2(0.5), texCoord);

    vec3 outColor = float2vec3(distanceFromCenter);
    color = vec4( outColor.x, 0.0, 1-outColor.x, 1.0);
}