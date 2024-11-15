#version 150

in vec2 position;
out vec2 uv;

uniform vec2 texture_size;
uniform vec3 texture_position;
uniform vec3 render_position;
uniform vec3 tile_size;
uniform mat3 view_transform;
uniform int first_person;

void main() {
	uv = position;
	vec3 pos = texture_position + render_position;
	pos.xy += position * texture_size;
	pos.y *= -1;
	pos = view_transform * pos;
	gl_Position = vec4(pos * tile_size * 2, 1);
	
	if (first_person == 1) gl_Position = vec4(gl_Position.xy * 10, 0.1 - gl_Position.z, -pos.z);
}