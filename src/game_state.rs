use crate::{
    game::{Game, GameResult, PlayerAction},
    game_graphics::{self, GameObject, GameResources, UIObject, UIResources},
    game_physics::GamePhysics,
    menu::{Menu, MenuAction, MenuResult},
};

pub enum GameState {
    StartMenu(Menu),
    InGame(Game, GamePhysics),
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
    pub fn update(&mut self, action: Option<GameAction>) -> bool {
        let mut to_quit = false;
        match self {
            Self::StartMenu(menu) => {
                if let Some(ma) = action.and_then(GameAction::to_menu_action) {
                    match menu.update(ma) {
                        Some(MenuResult::Start) => {
                            *self = Self::InGame(Game::new(), GamePhysics::new())
                        }
                        Some(MenuResult::Quit) => to_quit = true,
                        None => (),
                    }
                }
            }
            Self::InGame(game, physics) => {
                if let Some(pa) = action.and_then(GameAction::to_player_action) {
                    let result = game.update(pa);
                    physics.update(game, pa);
                    physics.step();
                    if let GameResult::Finished(final_score) = result {
                        *self = Self::GameOver(final_score)
                    }
                } else {
                    physics.step()
                }
            }
            Self::GameOver(_) => *self = Self::StartMenu(Menu::new()),
        }
        to_quit
    }

    pub fn make_ui(&self, resources: &UIResources) -> Vec<UIObject> {
        match self {
            Self::StartMenu(menu) => game_graphics::make_menu(&menu, resources),
            Self::InGame(game, _) => game_graphics::make_ui(&game, resources),
            Self::GameOver(_) => vec![],
        }
    }

    pub fn make_scene(&self, resources: &GameResources) -> Vec<GameObject> {
        match self {
            Self::StartMenu(_) => vec![],
            Self::InGame(game, physics) => physics.make_scene(game, resources),
            Self::GameOver(_) => vec![],
        }
    }
}
