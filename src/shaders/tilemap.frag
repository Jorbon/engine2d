#version 150

flat in vec3 normal_;
in vec2 uv_;
out vec4 color;
out vec4 data;

uniform sampler2D tilemap_texture;

const float TEXTURE_DIMENSIONS = 16;

void main() {
	vec3 c = texture(tilemap_texture, uv_ / TEXTURE_DIMENSIONS).rgb;
	color = vec4(c * dot(normal_, normalize(vec3(2, 1, 3))), 1.0);
	data = vec4(0, 0, 0, 0);
}
