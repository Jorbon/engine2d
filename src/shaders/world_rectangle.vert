#version 150

in vec2 position;
out vec2 uv;

uniform vec2 texture_position;
uniform vec2 texture_size;
uniform vec2 render_position;
uniform vec2 render_size_inverse;

void main() {
	uv = position;
	gl_Position = vec4(((position * texture_size + (texture_position - render_position)) * render_size_inverse * 2.0 - 1.0) * vec2(1, -1), 0.0, 1.0);
}