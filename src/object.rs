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

#[derive(Clone)]
pub struct Object {
    pub tess: TessResource,
    pub texture: TextureResource,
    pub transform: Transform,
}

impl Object {
    pub fn get_transform(&self) -> Matrix4<f32> {
        let local_transform = self.transform.to_matrix();
        local_transform
    }
}

pub struct Object2 {
    pub tess: TessResource,
    pub texture: TextureResource,
    pub transform: Transform2,
}

impl Object2 {
    pub fn get_transform(&self) -> Matrix3<f32> {
        let local_transform = self.transform.to_matrix();
        local_transform
    }
}

pub type Model = Vec<Object>;

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
    tess_counter: u32,
    texture_counter: u32,
    model_counter: u32,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            tesses: HashMap::new(),
            textures: HashMap::new(),
            models: HashMap::new(),
            tess_counter: 0,
            texture_counter: 0,
            model_counter: 0,
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

    pub fn log() -> ModelResource {
        ModelResource { idx: 0 }
    }

    pub fn branch_log() -> ModelResource {
        ModelResource { idx: 1 }
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

    fn load_tesses(&mut self, ctxt: &mut impl GraphicsContext<Backend = Backend>) {
        let cylinder = geometry::cylinder(1., 0.5, 20).make_tess(ctxt);
        self.add_tess(cylinder);
    }

    fn load_textures(&mut self, ctxt: &mut impl GraphicsContext<Backend = Backend>) {
        let img = image::io::Reader::open("../textures/pine-tree-bark-texture.jpg")
            .unwrap()
            .decode()
            .unwrap()
            .into_rgb8();
        let bark = make_texture(ctxt, &img);
        self.add_texture(bark);
    }

    fn load_models(&mut self) {
        let angle: f32 = RealField::frac_pi_2();
        let log = Object {
            tess: TessResource { idx: 0 },
            texture: TextureResource { idx: 0 },
            transform: Transform {
                translation: None,
                scale: None,
                rotation: Some(UnitQuaternion::from_axis_angle(
                    &Vector3::<f32>::x_axis(),
                    -angle,
                )),
            },
        };
        let branch = Object {
            tess: TessResource { idx: 0 },
            texture: TextureResource { idx: 0 },
            transform: Transform {
                translation: Some(Translation3::new(0.9, 0., 0.)),
                scale: Some([0.2, 0.2, 1.]),
                rotation: Some(UnitQuaternion::from_axis_angle(
                    &Vector3::<f32>::y_axis(),
                    RealField::frac_pi_2(),
                )),
            },
        };
        let log2 = log.clone();
        self.add_model(vec![log]);
        self.add_model(vec![log2, branch]);
    }

    pub fn load_defaults(&mut self, ctxt: &mut impl GraphicsContext<Backend = Backend>) {
        self.load_tesses(ctxt);
        self.load_textures(ctxt);
        self.load_models();
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
}
