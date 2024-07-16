#version 150

in vec2 screen_position;
out vec4 color;


void main() {
	color = 1 - vec4(screen_position.yyyy);
}
