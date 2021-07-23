in vec2 v_uv;

out vec4 frag_color;

uniform sampler2D tex;

void main() {
    vec4 texColor = texture(tex, v_uv);
    if(texColor.a < 0.1)
        discard;
    frag_color = texColor;
}