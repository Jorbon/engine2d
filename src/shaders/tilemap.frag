#version 150

in vec2 uv_;
out vec4 color;
out vec4 data;

uniform sampler2D tilemap_texture;

const float TEXTURE_DIMENSIONS = 16;

void main() {
	color = texture(tilemap_texture, uv_ / TEXTURE_DIMENSIONS);
	data = vec4(0, 0, 0, 0);
}
