#version 150

in vec3 position;
in vec3 normal;
in vec2 uv;
flat out vec3 normal_;
out vec2 uv_;

uniform vec3 tile_size;
uniform vec3 cell_position;
uniform mat3 view_transform;

void main() {
	normal_ = normal;
	uv_ = uv;
	vec3 pos = position + cell_position;
	pos.y *= -1;
	pos = view_transform * pos;
	gl_Position = vec4(pos * tile_size * 2, 1);
}
