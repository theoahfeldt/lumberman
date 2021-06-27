use crate::object::Object;
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

pub struct Game {
    player: Player,
    tree: VecDeque<Branch>,
}

impl Game {
    pub fn new() -> Self {
        let mut tree: VecDeque<Branch> = vec![Branch::None; 5].into();
        let player = Player {
            pos: PlayerPos::Left,
            alive: true,
            score: 0,
        };
        Self { player, tree }
    }

    pub fn to_scene(&self) -> Vec<Object> {
        vec![]
    }
}
