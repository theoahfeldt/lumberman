in vec3 v_normal;
in vec2 v_uv;

out vec4 frag_color;

uniform sampler2D tex;

void main() {
  vec3 light_color = vec3(1., 1., 1.);
  vec3 light_dir = normalize(vec3(0., 0.5, 1.));

  float ambient_strength = 0.2;
  vec3 ambient_color = vec3(1., 0.8, 0.6);
  vec3 ambient = ambient_strength * ambient_color;

  vec4 tex_color = texture(tex, v_uv);
  vec3 obj_color = vec3(tex_color);
  float transparency = tex_color.w;

  float diff = max(dot(v_normal, light_dir), 0.0);
  vec3 diffuse = diff * light_color;

  vec3 result = (ambient + diffuse) * obj_color;
  frag_color = vec4(result, transparency);
}
