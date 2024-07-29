#version 150

in vec2 world_position;
out vec4 color;

uniform usampler2D tile_data_texture;
uniform sampler2D tilemap_texture;

const uint TEXTURE_DIMENSIONS = uint(16);

void main() {
	ivec2 tile_position = ivec2(world_position);
	vec2 sub_tile_position = mod(world_position, 1.0);
	
	uvec4 tile_data = texelFetch(tile_data_texture, tile_position, 0);
	if ((sub_tile_position.x + tile_data.x) / TEXTURE_DIMENSIONS < 0.0) {
		color = vec4(1.0, 0.0, 0.0, 1.0);
	}
	else color = texelFetch(tilemap_texture, ivec2((sub_tile_position + tile_data.xy) * TEXTURE_DIMENSIONS), 0);
}
