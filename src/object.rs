use crate::semantics::Vertex;
use cgmath::{Matrix4, Vector3};
use luminance_front::context::GraphicsContext;
use luminance_front::tess::{Interleaved, Mode, Tess, TessError};
use luminance_front::Backend;

pub type VertexIndex = u32;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<VertexIndex>,
}

impl Mesh {
    pub fn to_tess<C>(
        self,
        ctxt: &mut C,
    ) -> Result<Tess<Vertex, VertexIndex, (), Interleaved>, TessError>
    where
        C: GraphicsContext<Backend = Backend>,
    {
        ctxt.new_tess()
            .set_mode(Mode::Triangle)
            .set_vertices(self.vertices)
            .set_indices(self.indices)
            .build()
    }
}

pub struct Object<'a> {
    pub mesh: &'a Tess<Vertex, VertexIndex, (), Interleaved>,
    pub position: Vector3<f32>,
    pub scale: f32,
    pub orientation: Vector3<f32>,
}

impl Object<'_> {
    pub fn get_transform(&self) -> Matrix4<f32> {
        let local_transform = Matrix4::<f32>::from_translation(self.position);
        local_transform
    }
}
