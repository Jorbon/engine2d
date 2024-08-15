#version 150

in vec2 position;
out vec2 screen_position;
out vec2 uv;

uniform vec2 texture_position;
uniform vec2 texture_size;
uniform vec2 render_position;
uniform vec2 render_size_inverse;

void main() {
	uv = position;
	screen_position = (position * texture_size + (texture_position - render_position)) * render_size_inverse;
	screen_position.y = 1.0 - screen_position.y;
	gl_Position = vec4((screen_position * 2.0 - 1.0), 0.0, 1.0);
}