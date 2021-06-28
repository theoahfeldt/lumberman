use crate::object::{Object, Transform};
use cgmath::Vector3;
use rand::Rng;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum Branch {
    None,
    Left,
    Right,
}

enum PlayerPos {
    Left,
    Right,
}

enum PlayerAction {
    ChopLeft,
    ChopRight,
}

struct Player {
    pos: PlayerPos,
    alive: bool,
    score: u32,
}

pub type Model<'a> = Vec<Object<'a>>;

pub struct GameObject<'a> {
    pub model: &'a Model<'a>,
    pub transform: Transform,
}

pub struct GameModels<'a> {
    pub log: Model<'a>,
    pub branch_log: Model<'a>,
}

pub struct Game<'a> {
    player: Player,
    tree: VecDeque<Branch>,
    models: GameModels<'a>,
}

impl<'a> Game<'a> {
    pub fn new(models: GameModels<'a>) -> Self {
        let tree: VecDeque<Branch> = vec![Branch::None; 5].into();
        let player = Player {
            pos: PlayerPos::Left,
            alive: true,
            score: 0,
        };
        Self {
            player,
            tree,
            models,
        }
    }

    pub fn to_scene(&self) -> Vec<GameObject> {
        self.tree
            .clone()
            .iter()
            .enumerate()
            .map(|(i, val)| {
                let model = match val {
                    Branch::None => &self.models.log,
                    Branch::Left | Branch::Right => &self.models.branch_log,
                };
                let transform = Transform {
                    position: Vector3::new(0., i as f32, 0.),
                    scale: 0.,
                    orientation: Vector3::new(0., 0., 0.),
                };
                GameObject { model, transform }
            })
            .collect()
    }
}
