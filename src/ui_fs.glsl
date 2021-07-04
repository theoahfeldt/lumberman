in vec2 v_uv;

out vec4 frag_color;

uniform sampler2D tex;

void main() {
     frag_color = texture(tex, v_uv);
}