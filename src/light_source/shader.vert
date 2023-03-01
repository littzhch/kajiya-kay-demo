#version 460 core

layout (location = 0) in vec3 position;

uniform vec3 shift;
uniform mat4 camera;

void main()
{
    vec3 final_pos = position + shift;
    gl_Position = camera * vec4(final_pos, 1.0);
}