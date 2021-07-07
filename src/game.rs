use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum Branch {
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

#[derive(Clone)]
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

pub struct Game {
    player: Player,
    pub tree: VecDeque<Branch>,
}

impl Game {
    pub fn new() -> Self {
        let tree: VecDeque<Branch> = vec![Branch::None; 5].into();
        let player = Player {
            pos: PlayerPos::Left,
            alive: true,
            score: 0,
        };
        Self { player, tree }
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

    pub fn get_score(&self) -> u32 {
        self.player.score
    }
}
