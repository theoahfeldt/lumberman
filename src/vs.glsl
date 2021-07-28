in vec3 position;
in vec3 normal;
in vec2 uv;

out vec3 v_normal;
out vec2 v_uv;

uniform mat4 model;
uniform mat4 projection;
uniform mat4 view;

void main() {
  v_normal = normalize(model * vec4(normal, 0.)).xyz;
  gl_Position = projection * view * model * vec4(position, 1.);
  v_uv = uv;
}
