use crate::{
    game::{Game, PlayerAction},
    game_graphics::{self, GameObject, GameResources, UIObject, UIResources},
    menu::{Menu, MenuAction, MenuResult},
};

pub enum GameState {
    StartMenu(Menu),
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

    fn to_menu_action(self) -> Option<MenuAction> {
        match self {
            Self::Up => Some(MenuAction::Up),
            Self::Down => Some(MenuAction::Down),
            Self::Enter => Some(MenuAction::Select),
            _ => None,
        }
    }
}

impl GameState {
    pub fn update(&mut self, action: GameAction) -> bool {
        let mut to_quit = false;
        match self {
            Self::StartMenu(menu) => {
                if let Some(ma) = action.to_menu_action() {
                    match menu.update(ma) {
                        Some(MenuResult::Start) => *self = Self::InGame(Game::new()),
                        Some(MenuResult::Quit) => to_quit = true,
                        None => (),
                    }
                }
            }
            Self::InGame(game) => {
                if let Some(pa) = action.to_player_action() {
                    if let Some(final_score) = game.update(pa) {
                        *self = Self::GameOver(final_score)
                    }
                }
            }
            Self::GameOver(_) => *self = Self::StartMenu(Menu::new()),
        }
        to_quit
    }

    pub fn make_ui(&self, resources: &UIResources) -> Vec<UIObject> {
        match self {
            Self::StartMenu(_) => vec![],
            Self::InGame(game) => game_graphics::make_ui(&game, resources),
            Self::GameOver(_) => vec![],
        }
    }

    pub fn make_scene(&self, resources: &GameResources) -> Vec<GameObject> {
        match self {
            Self::StartMenu(_) => vec![],
            Self::InGame(game) => game_graphics::make_scene(&game, resources),
            Self::GameOver(_) => vec![],
        }
    }
}
