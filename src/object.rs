use crate::semantics::Vertex;
use image::RgbaImage;

use luminance_front::{
    context::GraphicsContext,
    pixel::NormRGBA8UI,
    tess::{Interleaved, Mode, Tess},
    texture::{Dim2, GenMipmaps, Sampler, Texture},
    Backend,
};
use rapier3d::na::Matrix4;
use std::collections::HashMap;

pub type VertexIndex = u32;
pub type DefaultTess = Tess<Vertex, VertexIndex, (), Interleaved>;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<VertexIndex>,
}

impl Mesh {
    pub fn make_tess<C>(self, ctxt: &mut C) -> DefaultTess
    where
        C: GraphicsContext<Backend = Backend>,
    {
        ctxt.new_tess()
            .set_mode(Mode::Triangle)
            .set_vertices(self.vertices)
            .set_indices(self.indices)
            .build()
            .expect("Building tess")
    }
}

pub type RgbaTexture = Texture<Dim2, NormRGBA8UI>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TessResource {
    idx: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureResource {
    idx: u32,
}

#[derive(Clone)]
pub struct Object {
    pub tess: TessResource,
    pub texture: TextureResource,
    pub transform: Matrix4<f32>,
}

pub type Model = Vec<Object>;

fn make_texture(
    context: &mut impl GraphicsContext<Backend = Backend>,
    img: &image::RgbaImage,
) -> RgbaTexture {
    let (width, height) = img.dimensions();
    let texels = img.as_raw();

    context
        .new_texture_raw(
            [width, height],
            0,
            Sampler::default(),
            GenMipmaps::No,
            texels,
        )
        .unwrap()
}

pub struct ResourceManager {
    tesses: HashMap<u32, DefaultTess>,
    textures: HashMap<u32, RgbaTexture>,
    tess_counter: u32,
    texture_counter: u32,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            tesses: HashMap::new(),
            textures: HashMap::new(),
            tess_counter: 0,
            texture_counter: 0,
        }
    }

    fn add_tess(&mut self, tess: DefaultTess) -> TessResource {
        self.tesses.insert(self.tess_counter, tess);
        let result = TessResource {
            idx: self.tess_counter,
        };
        self.tess_counter += 1;
        result
    }

    fn add_texture(&mut self, texture: RgbaTexture) -> TextureResource {
        self.textures.insert(self.texture_counter, texture);
        let result = TextureResource {
            idx: self.texture_counter,
        };
        self.texture_counter += 1;
        result
    }

    pub fn get_tess(&self, tess: &TessResource) -> &DefaultTess {
        self.tesses.get(&tess.idx).unwrap()
    }

    pub fn get_texture(&mut self, texture: &TextureResource) -> &mut RgbaTexture {
        self.textures.get_mut(&texture.idx).unwrap()
    }

    pub fn make_tess(
        &mut self,
        ctxt: &mut impl GraphicsContext<Backend = Backend>,
        mesh: Mesh,
    ) -> TessResource {
        self.add_tess(mesh.make_tess(ctxt))
    }

    pub fn make_texture(
        &mut self,
        ctxt: &mut impl GraphicsContext<Backend = Backend>,
        img: &RgbaImage,
    ) -> TextureResource {
        self.add_texture(make_texture(ctxt, img))
    }

    pub fn update_tess(
        &mut self,
        resource: TessResource,
        ctxt: &mut impl GraphicsContext<Backend = Backend>,
        mesh: Mesh,
    ) {
        self.tesses.insert(resource.idx, mesh.make_tess(ctxt));
    }

    pub fn update_texture(
        &mut self,
        resource: TextureResource,
        ctxt: &mut impl GraphicsContext<Backend = Backend>,
        img: &RgbaImage,
    ) {
        self.textures.insert(resource.idx, make_texture(ctxt, img));
    }
}
