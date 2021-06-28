use crate::object::{Object, Transform};
use nalgebra::{RealField, Translation3, UnitQuaternion, Vector3};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
enum Branch {
    None,
    Left,
    Right,
}

impl Distribution<Branch> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Branch {
        match rng.gen_range(0..=1) {
            0 => Branch::Left,
            _ => Branch::Right,
        }
    }
}

#[derive(PartialEq)]
enum PlayerPos {
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub enum PlayerAction {
    ChopLeft,
    ChopRight,
}

struct Player {
    pos: PlayerPos,
    alive: bool,
    score: u32,
}

impl Player {
    fn apply_action(&mut self, action: PlayerAction) {
        self.pos = match action {
            PlayerAction::ChopLeft => PlayerPos::Left,
            PlayerAction::ChopRight => PlayerPos::Right,
        }
    }

    fn collides_with(&self, branch: &Branch) -> bool {
        self.pos == PlayerPos::Left && *branch == Branch::Left
            || self.pos == PlayerPos::Right && *branch == Branch::Right
    }
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

    pub fn update(&mut self, action: PlayerAction) {
        self.player.apply_action(action);
        let lowest_branch = self.tree.front().unwrap();
        if self.player.collides_with(lowest_branch) {
            self.player.alive = false;
        }
        self.tree.pop_front();
        let lowest_branch = self.tree.front().unwrap();
        if self.player.collides_with(lowest_branch) {
            self.player.alive = false;
        } else {
            self.player.score += 1;
        }
        let new_branch = if *self.tree.back().unwrap() == Branch::None {
            rand::random::<Branch>()
        } else {
            Branch::None
        };
        self.tree.push_back(new_branch);
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
}
