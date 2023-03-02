#version 460 core

out vec4 FragColor;
in vec2 texCoord;

uniform sampler2D tnt_texture;
uniform vec3 light_color;

void main() {
    FragColor = texture(tnt_texture, texCoord) * vec4(light_color, 0.0);
}