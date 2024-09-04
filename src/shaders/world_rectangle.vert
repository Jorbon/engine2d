#version 150

in vec2 position;
out vec2 uv;

uniform vec2 texture_size;
uniform vec3 texture_position;
uniform vec3 render_position;
uniform vec3 tile_size;

const float PROJECTION_OFFSET = 0.5;

void main() {
	uv = position;
	vec3 pos = texture_position - render_position;
	pos.y -= pos.z * PROJECTION_OFFSET;
	pos.xy += position * texture_size;
	pos.y *= -1;
	gl_Position = vec4(pos * tile_size * 2, 1);
}