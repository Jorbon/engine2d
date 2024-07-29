#version 150

in vec2 screen_position;
out vec4 color;

uniform float aspect_ratio;
uniform sampler2D screen_texture;

void main() {
	vec4 c = texture(screen_texture, screen_position);
	color = c;//vec4(1.0 - c.r, 1.0 - c.g, 1.0 - c.b, c.a);
}