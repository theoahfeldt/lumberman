use nalgebra::{RealField, Translation3, UnitQuaternion, Vector3};

use crate::{
    game::{Branch, Game},
    object::{Object, Transform},
};

pub type Model<'a> = Vec<Object<'a>>;

pub struct GameObject<'a> {
    pub model: &'a Model<'a>,
    pub transform: Transform,
}

pub struct GameModels<'a> {
    pub log: Model<'a>,
    pub branch_log: Model<'a>,
}

pub fn to_scene<'a>(game: &Game, models: &'a GameModels) -> Vec<GameObject<'a>> {
    game.tree
        .clone()
        .iter()
        .enumerate()
        .map(|(i, val)| {
            let model = match val {
                Branch::None => &models.log,
                Branch::Left | Branch::Right => &models.branch_log,
            };
            let orientation = match val {
                Branch::Right => Some(UnitQuaternion::from_axis_angle(
                    &Vector3::<f32>::y_axis(),
                    RealField::pi(),
                )),
                Branch::Left | Branch::None => None,
            };
            let transform = Transform {
                translation: Some(Translation3::new(0., i as f32, 0.)),
                scale: None,
                orientation,
            };
            GameObject { model, transform }
        })
        .collect()
}
