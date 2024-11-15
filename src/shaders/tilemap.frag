#version 150

flat in vec3 normalf;
in vec2 uvf;
out vec4 color;
out vec4 data;

uniform sampler2D tex;

void main() {
	vec4 c = texture(tex, uvf);
	if (c.a < 0.1) discard;
	
	float shade = max(dot(normalf, normalize(vec3(2, 1, 3))), 0.4);
	color = vec4(c.rgb * shade, c.a);
	data = vec4(0, 0, 0, 0);
}
