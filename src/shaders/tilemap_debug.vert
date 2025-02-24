#version 150

in vec3 position;
in vec3 normal;
in vec3 color;
flat out vec3 normalf;
out vec3 c;

uniform vec3 tile_size;
uniform vec3 render_position;
uniform mat3 view_transform;
uniform int first_person;

void main() {
	normalf = normal;
	c = color;
	
	vec3 pos = position + render_position;
	pos.y *= -1;
	pos = view_transform * pos;
	gl_Position = vec4(pos * tile_size * 2, 1);
	
	if (first_person == 1) gl_Position = vec4(gl_Position.xy * 10, 0.1 - gl_Position.z, -pos.z);
}
