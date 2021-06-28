use crate::object::{Transform, Object};
use crate::object;
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

struct Asset<'a> {
    objects: Vec<Object<'a>>,
    transform: Transform,
}

pub struct Game<'a> {
    player: Player,
    tree: VecDeque<Branch>,
    assets: Vec<Asset<'a>>,
    meshes: Vec<Tess<Vertex, VertexIndex, (), Interleaved>>
}

impl Game<'_> {
    pub fn new() -> Self {
        let mut tree: VecDeque<Branch> = vec![Branch::None; 5].into();
        let player = Player {
            pos: PlayerPos::Left,
            alive: true,
            score: 0,
        };
        let cylinder = object::cylinder(1., 0.5, 20);
        let assets = vec![Asset {}]
        Self { player, tree }
    }

    pub fn to_scene(&self) -> Vec<Object> {
        vec![]
    }
}
