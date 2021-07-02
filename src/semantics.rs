use luminance_derive::{Semantics, UniformInterface, Vertex};
use luminance_front::{
    pipeline::TextureBinding, pixel::NormUnsigned, shader::Uniform, texture::Dim2,
};

#[derive(Debug, UniformInterface)]
pub struct ShaderInterface {
    #[uniform(unbound)]
    pub projection: Uniform<[[f32; 4]; 4]>,
    #[uniform(unbound)]
    pub view: Uniform<[[f32; 4]; 4]>,
    #[uniform(unbound)]
    pub local_transform: Uniform<[[f32; 4]; 4]>,
    #[uniform(unbound)]
    pub model_transform: Uniform<[[f32; 4]; 4]>,
    pub tex: Uniform<TextureBinding<Dim2, NormUnsigned>>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Semantics)]
pub enum Semantics {
    #[sem(name = "position", repr = "[f32; 3]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "normal", repr = "[f32; 3]", wrapper = "VertexNormal")]
    Normal,
    #[sem(name = "uv", repr = "[f32; 2]", wrapper = "VertexUV")]
    UV,
}

#[derive(Clone, Copy, Debug, Vertex)]
#[vertex(sem = "Semantics")]
pub struct Vertex {
    position: VertexPosition,
    normal: VertexNormal,
    uv: VertexUV,
}
