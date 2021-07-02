in vec3 position;
in vec3 normal;
in vec2 uv;

out vec3 v_normal;
out vec2 v_uv;

uniform mat4 local_transform;
uniform mat4 model_transform;
uniform mat4 projection;
uniform mat4 view;

void main() {
  v_normal = normalize(model_transform * local_transform * vec4(normal, 0.)).xyz;
  gl_Position = projection * view * model_transform * local_transform * vec4(position, 1.);
  v_uv = uv;
}
