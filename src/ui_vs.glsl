in vec2 uv;
in vec3 position;

out vec2 v_uv;

uniform mat4 local_transform;
uniform mat4 model_transform;

void main() {
     gl_Position = model_transform * local_transform * vec4(position, 1.);
     v_uv = uv;
}