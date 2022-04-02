#version 100
attribute vec2 pos;
attribute vec2 uv;

varying lowp vec2 texcoord;
uniform mat4 mvp;

void main() {
    gl_Position = vec4(pos, 0, 1) * mvp;
    texcoord = uv;
}
