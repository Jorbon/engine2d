#version 150

in vec2 world_position;
out vec4 color;
out vec4 data;

uniform float z;
uniform usampler2D tile_data_texture;
uniform sampler2D tilemap_texture;

const uint TEXTURE_DIMENSIONS = uint(16);

void main() {
	ivec2 tile_position = ivec2(world_position);
	vec2 sub_tile_position = mod(world_position, 1.0);
	
	uvec2 tile_data_above = texelFetch(tile_data_texture, ivec2(tile_position.x, tile_position.y - 1), 0).xy;
	vec4 color_back = vec4(0.0, 0.0, 0.0, 0.0);
	if (sub_tile_position.y <= 0.5) {
		color_back = texelFetch(tilemap_texture, ivec2((sub_tile_position * vec2(1.0, 2.0) + tile_data_above.xy) * TEXTURE_DIMENSIONS), 0);
	}
	
	uvec2 tile_data = texelFetch(tile_data_texture, tile_position, 0).xy;
	vec4 color_front = texelFetch(tilemap_texture, ivec2((sub_tile_position + tile_data.xy) * TEXTURE_DIMENSIONS), 0);
	
	color = vec4((color_back.rgb * 0.5 - color_front.rgb) * color_back.a + color_front.rgb, color_back.a + color_front.a * (1.0 - color_back.a));
	
	float actual_z = mix(z - sub_tile_position.y * 2.0, z, color_front.a);
	data = vec4(clamp(actual_z / 16.0 + 0.5, 0.0, 1.0), 0.0, 0.0, color.a);
	
}
