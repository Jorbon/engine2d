#version 150

in vec3 position;
in vec2 uv;
out vec2 uv_;

uniform vec3 cell_position;
uniform vec3 tile_size;

const float PROJECTION_OFFSET = 0.5;

void main() {
	uv_ = uv;
	vec3 pos = position + cell_position;
	pos.y -= pos.z * PROJECTION_OFFSET;
	pos.y *= -1;
	gl_Position = vec4(pos * tile_size * 2, 1);
}
