#version 150

in vec2 uv;
out vec4 color;
out vec4 data;

uniform sampler2D tex;

void main() {
	color = texture(tex, uv);
	data = vec4(0, 0, 0, 0);
}
