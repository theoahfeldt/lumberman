in vec3 position;
in vec3 normal;
in vec3 in_position;
in float weight;
in mat3 orientation;

out vec3 v_normal;

uniform mat4 local_transform;
uniform mat4 model_transform;
uniform mat4 projection;
uniform mat4 view;

void main() {
  v_normal = normal;
  gl_Position = projection * view * model_transform * local_transform * vec4(position, 1.);
}
