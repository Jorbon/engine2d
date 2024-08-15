#version 150

in vec2 screen_position;
in vec2 uv;
out vec4 color;
out vec4 data;

uniform float z;
uniform sampler2D tex;
uniform sampler2D data_texture;

void main() {
	float tile_z = texture(data_texture, screen_position).x;
	if (z / 16.0 + 0.5 < tile_z) discard;
	
	color = texture(tex, uv);
	data = vec4(clamp(z / 16.0 + 0.5, 0.0, 1.0), 0.0, 0.0, color.a);
}
