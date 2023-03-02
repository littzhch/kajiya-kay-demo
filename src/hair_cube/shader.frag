#version 460 core

out vec4 FragColor;
in vec2 texCoord;

uniform sampler2D tnt_texture;

void main() {
    FragColor = texture(tnt_texture, texCoord);
}