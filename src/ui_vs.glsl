in vec2 uv;
in vec3 position;

out vec2 v_uv;

uniform mat4 model;

void main() {
     gl_Position = model * vec4(position, 1.);
     v_uv = uv;
}