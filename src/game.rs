use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(PartialEq, Clone, Copy)]
pub enum PlayerPos {
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

pub enum GameEvent {
    Performed(PlayerAction),
    Finished(u32),
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

    pub fn update(&mut self, action: PlayerAction) -> GameEvent {
        self.player.apply_action(action);
        let lowest_branch = self.tree.front().unwrap();
        if self.player.collides_with(lowest_branch) {
            self.player.alive = false;
        } else {
            self.tree.pop_front();
            self.player.score += 1;
            let lowest_branch = self.tree.front().unwrap();
            if self.player.collides_with(lowest_branch) {
                self.player.alive = false;
            }

            let new_branch = if *self.tree.back().unwrap() == Branch::None {
                rand::random::<Branch>()
            } else {
                Branch::None
            };
            self.tree.push_back(new_branch);
        }

        if self.player.alive {
            GameEvent::Performed(action)
        } else {
            GameEvent::Finished(self.player.score)
        }
    }

    pub fn get_score(&self) -> u32 {
        self.player.score
    }

    pub fn get_player_pos(&self) -> PlayerPos {
        self.player.pos
    }
}
