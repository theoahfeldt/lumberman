in vec3 v_normal;
in vec2 v_uv;

out vec3 frag_color;

uniform sampler2D tex;

void main() {
  vec3 obj_color = vec3(texture(tex, v_uv));
  vec3 light_dir = vec3(0., -0.5, -1.);
  float kd = dot(v_normal, -light_dir);

  frag_color = obj_color * kd;
}
