#version 150

in vec2 position;
out vec2 screen_position;

void main() {
	screen_position = position;
	gl_Position = vec4((position * 2.0 - 1.0) * vec2(1, -1), 0.0, 1.0);
}
