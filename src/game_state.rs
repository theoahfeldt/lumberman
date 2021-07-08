use crate::{
    game::{Game, PlayerAction},
    game_graphics::{self, GameObject, GameResources, UIObject, UIResources},
};

pub enum GameState {
    StartMenu,
    InGame(Game),
    GameOver(u32),
}

pub enum GameAction {
    Left,
    Right,
    Down,
    Up,
    Enter,
}

impl GameAction {
    fn to_player_action(self) -> Option<PlayerAction> {
        match self {
            Self::Left => Some(PlayerAction::ChopLeft),
            Self::Right => Some(PlayerAction::ChopRight),
            _ => None,
        }
    }
}

impl GameState {
    pub fn update(&mut self, action: GameAction) {
        match self {
            Self::StartMenu => *self = Self::InGame(Game::new()),
            Self::InGame(game) => {
                if let Some(pa) = action.to_player_action() {
                    if let Some(final_score) = game.update(pa) {
                        *self = Self::GameOver(final_score)
                    }
                }
            }
            Self::GameOver(_) => *self = Self::InGame(Game::new()),
        }
    }

    pub fn make_ui(&self, resources: &UIResources) -> Vec<UIObject> {
        match self {
            Self::StartMenu => vec![],
            Self::InGame(game) => game_graphics::make_ui(resources),
            Self::GameOver(_) => vec![],
        }
    }

    pub fn make_scene(&self, resources: &GameResources) -> Vec<GameObject> {
        match self {
            Self::StartMenu => vec![],
            Self::InGame(game) => game_graphics::make_scene(&game, resources),
            Self::GameOver(_) => vec![],
        }
    }
}
