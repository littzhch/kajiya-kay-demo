#version 460 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 tex_coord;

out vec2 TexCoord;
out vec3 Normal;

uniform mat4 camera;

void main() {
    gl_Position = camera * vec4(position, 1.0);
    TexCoord = tex_coord;
    Normal = normal;
}