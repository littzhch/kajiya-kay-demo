#version 460 core

out vec4 FragColor;

in vec2 TexCoord;
in vec3 Normal;
in vec3 FragPos;

uniform sampler2D tnt_texture;
uniform vec3 light_color;
uniform vec3 light_pos;
uniform vec3 camera_pos;

void main() {
    float ambient_strength = 0.05;
    vec3 ambient = ambient_strength * light_color;

    float diffuse_strength = 1.0;
    vec3 light_dir = normalize(light_pos - FragPos);
    float diff = max(dot(light_dir, Normal), 0.0);
    vec3 diffuse = diffuse_strength * diff * light_color;


    float specular_strength = 2.0;
    vec3 view_dir = normalize(camera_pos - FragPos);
    vec3 half_vec = normalize(view_dir + light_dir);
    float spec = pow(max(dot(Normal, half_vec), 0.0), 64);
    vec3 specular = specular_strength * spec * light_color;

    vec3 light_result = ambient + diffuse + specular;

    FragColor = texture(tnt_texture, TexCoord) * vec4(light_result, 0.0);
}