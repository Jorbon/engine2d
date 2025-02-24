#version 150

flat in vec3 normalf;
in vec3 c;
out vec4 color;
out vec4 data;

void main() {
	float shade = max(dot(normalf, normalize(vec3(2, 1, 3))), 0.4);
	color = vec4(c.rgb * shade, 1.0);
	data = vec4(0, 0, 0, 0);
}
