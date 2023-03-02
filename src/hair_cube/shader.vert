#version 460 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 tex_coord;

out vec2 texCoord;
uniform mat4 camera;

void main() {
    gl_Position = camera * vec4(position, 1.0);
    texCoord = tex_coord;
}