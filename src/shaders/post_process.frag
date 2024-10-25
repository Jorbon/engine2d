#version 150

in vec2 screen_position;
out vec4 color;

uniform float aspect_ratio;
uniform float tile_depth_inverse;
uniform sampler2D screen_texture;
uniform sampler2D data_texture;
uniform sampler2D depth_texture;

const vec3 kernel[12] = vec3[](
	vec3(-1.0,  0.0, 1.0),
	vec3( 1.0,  0.0, 1.0),
	vec3( 0.0, -1.0, 1.0),
	vec3( 0.0,  1.0, 1.0),
	vec3(-1.0, -1.0, 0.8),
	vec3( 1.0, -1.0, 0.8),
	vec3(-1.0,  1.0, 0.8),
	vec3( 1.0,  1.0, 0.8),
	vec3(-2.0,  0.0, 0.5),
	vec3( 2.0,  0.0, 0.5),
	vec3( 0.0, -2.0, 0.5),
	vec3( 0.0,  2.0, 0.5)
);

void main() {
	vec4 c = texture(screen_texture, screen_position);
	
	float z = texture(depth_texture, screen_position).x;
	
	float shade = 0.0;
	for (int i = 0; i < 12; i++) {
		shade += clamp(texture(depth_texture, screen_position + kernel[i].xy * 0.0015 * vec2(1.0, aspect_ratio)).x - z, 0.0, 0.2) * kernel[i].z;
	}
	
	color = vec4(c.rgb * (1.0 - shade * tile_depth_inverse * 0.25), c.a);
}