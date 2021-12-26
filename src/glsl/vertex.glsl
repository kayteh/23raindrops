#version 140

in vec2 position;
in vec2 uv;
out vec2 texCoord;

uniform mat4 matrix;

void main() {
    texCoord = uv;
    gl_Position = matrix * vec4(position, 0.0, 1.0);
}