use crate::{
    audio::{AudioPlayer, AudioResources},
    game::{Game, GameEvent, PlayerAction},
    game_graphics::{self, GameObject, GameResources, UIObject, UIResources},
    game_physics::GamePhysics,
    menu::{Menu, MenuAction, MenuResult},
};

enum GameState {
    StartMenu,
    InGame,
    GameOver,
}

pub struct GameRunner {
    state: GameState,
    menu: Menu,
    game: Game,
    physics: GamePhysics,
    player: AudioPlayer,
    event: Option<GameEvent>,
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

impl GameRunner {
    pub fn new() -> Self {
        Self {
            menu: Menu::new(),
            state: GameState::StartMenu,
            game: Game::new(),
            physics: GamePhysics::new(),
            player: AudioPlayer::new(),
            event: None,
        }
    }

    pub fn update(&mut self, action: Option<GameAction>) -> bool {
        let mut to_quit = false;
        match self.state {
            GameState::StartMenu => {
                if let Some(ma) = action.and_then(GameAction::to_menu_action) {
                    match self.menu.update(ma) {
                        Some(MenuResult::Start) => {
                            self.state = GameState::InGame;
                        }
                        Some(MenuResult::Quit) => to_quit = true,
                        None => (),
                    }
                }
            }
            GameState::InGame => {
                if let Some(pa) = action.and_then(GameAction::to_player_action) {
                    let event = self.game.update(pa);
                    self.physics.update(&self.game, pa);
                    self.physics.step();
                    if let GameEvent::Finished(_) = event {
                        self.state = GameState::GameOver;
                    }
                    self.event = Some(event);
                } else {
                    self.physics.step();
                    self.event = None;
                }
            }
            GameState::GameOver => {
                self.state = GameState::StartMenu;
                self.game = Game::new();
                self.physics.reset();
            }
        }
        to_quit
    }

    pub fn play_audio(&mut self, resources: &AudioResources) {
        match self.state {
            GameState::InGame => {
                if let Some(GameEvent::Performed(_action)) = self.event {
                    self.player.play(resources.chop.clone())
                }
            }
            _ => (),
        }
    }

    pub fn make_ui(&self, resources: &UIResources) -> Vec<UIObject> {
        match self.state {
            GameState::StartMenu => game_graphics::make_menu(&self.menu, resources),
            GameState::InGame => game_graphics::make_ui(&self.game, resources),
            GameState::GameOver => vec![],
        }
    }

    pub fn make_scene(&self, resources: &GameResources) -> Vec<GameObject> {
        match self.state {
            GameState::StartMenu => vec![],
            GameState::InGame => self.physics.make_scene(&self.game, resources),
            GameState::GameOver => vec![],
        }
    }
}
