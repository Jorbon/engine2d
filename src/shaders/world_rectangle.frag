#version 150

in vec2 uv;
out vec4 color;
out vec4 data;

uniform sampler2D tex;

void main() {
	vec4 c = texture(tex, uv);
	if (c.a < 0.1) discard;
	color = c;
	data = vec4(0, 0, 0, 0);
}
