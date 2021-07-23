in vec3 v_normal;
in vec2 v_uv;

out vec4 frag_color;

uniform sampler2D tex;

void main() {
  vec3 light_color = vec3(1., 1., 1.);
  vec3 light_dir = normalize(vec3(0., 0.5, 1.));

  float ambient_strength = 0.2;
  vec3 ambient = ambient_strength * light_color;

  vec3 obj_color = vec3(texture(tex, v_uv));

  float diff = max(dot(v_normal, light_dir), 0.0);
  vec3 diffuse = diff * light_color;

  vec3 result = (ambient + diffuse) * obj_color;
  frag_color = vec4(result, 1.);
}
