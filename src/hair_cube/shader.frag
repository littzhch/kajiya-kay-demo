#version 460 core

out vec4 FragColor;

in vec2 TexCoord;
in vec3 Normal;
in vec3 FragPos;
in vec3 Binormal;

uniform sampler2D tnt_texture;
uniform sampler2D shifp_map;
uniform vec3 light_color;
uniform vec3 light_pos;
uniform vec3 camera_pos;


void main() {
    float ambient_strength = 0.2;
    vec3 ambient = ambient_strength * light_color;

    float diffuse_strength = 0.5;
    vec3 light_dir = normalize(light_pos - FragPos);
    float diff = max(dot(light_dir, Normal), 0.0);
    vec3 diffuse = diffuse_strength * diff * light_color;

    float specular_strength = 1.5;
    float ctrl = max(normalize(dot(light_dir, Normal)), 0.0);
    vec3 binormal = 0.3 * (length(texture(shifp_map, TexCoord)) - 0.5) * Normal + Binormal;

    vec3 view_dir = normalize(camera_pos - FragPos);
    vec3 half_vec = normalize(view_dir + light_dir);
    float result = sqrt(1.0 - pow(dot(half_vec, binormal), 2));
    float dir_atten = smoothstep(-1.0, 0.0, dot(half_vec, binormal));
    float spec = dir_atten * pow(max(result, 0.0), 1024);
    vec3 specular = specular_strength * ctrl * spec * light_color;

    vec3 light_result = ambient + diffuse + specular;

    FragColor = texture(tnt_texture, TexCoord) * vec4(light_result, 0.0);
}