#version 150

in vec2 position;
out vec2 world_position;

uniform float aspect_ratio;
uniform vec2 offset;
uniform float screen_width_in_tiles;

void main() {
	world_position = position * screen_width_in_tiles * vec2(1, aspect_ratio) + offset;
	gl_Position = vec4((position * 2.0 - 1.0) * vec2(1, -1), 0.0, 1.0);
}
