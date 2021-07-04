in vec2 uv;
in vec3 position;

out vec2 v_uv;

void main() {
     gl_Position = vec4(position, 1.);
     v_uv = uv;
}