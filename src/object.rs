use crate::{
    geometry,
    semantics::Vertex,
    transform::{Transform, Transform2},
};
use image::RgbImage;
use luminance_front::{
    context::GraphicsContext,
    pixel::NormRGB8UI,
    tess::{Interleaved, Mode, Tess},
    texture::{Dim2, GenMipmaps, Sampler, Texture},
    Backend,
};
use nalgebra::{
    Matrix3, Matrix4, RealField, Rotation2, Translation2, Translation3, UnitQuaternion, Vector3,
};
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

pub type RgbTexture = Texture<Dim2, NormRGB8UI>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TessResource {
    idx: u32,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TextureResource {
    idx: u32,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ModelResource {
    idx: u32,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Model2Resource {
    idx: u32,
}

#[derive(Clone)]
pub struct Object {
    pub tess: TessResource,
    pub texture: TextureResource,
    pub transform: Transform,
}

impl Object {
    pub fn get_transform(&self) -> Matrix4<f32> {
        self.transform.to_matrix()
    }
}

#[derive(Clone)]
pub struct Object2 {
    pub tess: TessResource,
    pub texture: TextureResource,
    pub transform: Transform2,
}

impl Object2 {
    pub fn get_transform(&self) -> Matrix4<f32> {
        self.transform.to_matrix()
    }
}

pub type Model = Vec<Object>;
pub type Model2 = Vec<Object2>;

fn make_texture(
    context: &mut impl GraphicsContext<Backend = Backend>,
    img: &image::RgbImage,
) -> RgbTexture {
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
    textures: HashMap<u32, RgbTexture>,
    models: HashMap<u32, Model>,
    model2s: HashMap<u32, Model2>,
    tess_counter: u32,
    texture_counter: u32,
    model_counter: u32,
    model2_counter: u32,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            tesses: HashMap::new(),
            textures: HashMap::new(),
            models: HashMap::new(),
            model2s: HashMap::new(),
            tess_counter: 0,
            texture_counter: 0,
            model_counter: 0,
            model2_counter: 0,
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

    fn add_texture(&mut self, texture: RgbTexture) -> TextureResource {
        self.textures.insert(self.texture_counter, texture);
        let result = TextureResource {
            idx: self.texture_counter,
        };
        self.texture_counter += 1;
        result
    }

    fn add_model(&mut self, model: Model) -> ModelResource {
        self.models.insert(self.model_counter, model);
        let result = ModelResource {
            idx: self.model_counter,
        };
        self.model_counter += 1;
        result
    }

    fn add_model2(&mut self, model: Model2) -> Model2Resource {
        self.model2s.insert(self.model2_counter, model);
        let result = Model2Resource {
            idx: self.model2_counter,
        };
        self.model2_counter += 1;
        result
    }

    pub fn get_tess(&self, tess: &TessResource) -> &DefaultTess {
        self.tesses.get(&tess.idx).unwrap()
    }

    pub fn get_texture(&mut self, texture: &TextureResource) -> &mut RgbTexture {
        self.textures.get_mut(&texture.idx).unwrap()
    }

    pub fn get_model(&self, model: &ModelResource) -> &Model {
        self.models.get(&model.idx).unwrap()
    }

    pub fn get_model2(&self, model: &Model2Resource) -> &Model2 {
        self.model2s.get(&model.idx).unwrap()
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
        img: &RgbImage,
    ) -> TextureResource {
        self.add_texture(make_texture(ctxt, img))
    }

    pub fn make_model(&mut self, model: Model) -> ModelResource {
        self.add_model(model)
    }

    pub fn make_model2(&mut self, model: Model2) -> Model2Resource {
        self.add_model2(model)
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
        img: &RgbImage,
    ) {
        self.textures.insert(resource.idx, make_texture(ctxt, img));
    }

    pub fn update_model(&mut self, resource: ModelResource, model: Model) {
        self.models.insert(resource.idx, model);
    }

    pub fn update_model2(&mut self, resource: Model2Resource, model: Model2) {
        self.model2s.insert(resource.idx, model);
    }
}
