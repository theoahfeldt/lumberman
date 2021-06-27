use std::collections::VecDeque;
use rand::Rng;

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

struct Game {
	player: Player,
	tree: VecDeque<Branch>,
}

impl Game {
	fn new() -> Self {
		let mut tree: VecDeque<Branch> = vec![Branch::None; 5].into();
		
	} 
}
