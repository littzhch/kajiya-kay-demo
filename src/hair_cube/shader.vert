#version 460 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 tex_coord;
layout (location = 3) in vec3 binormal;

out vec2 TexCoord;
out vec3 Normal;
out vec3 FragPos;
out vec3 Binormal;

uniform mat4 camera;

void main() {
    gl_Position = camera * vec4(position, 1.0);
    TexCoord = tex_coord;
    Normal = normal;
    FragPos = position;
    Binormal = binormal;
}