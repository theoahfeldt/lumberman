use glfw::{FlushedMessages, WindowEvent};

use crate::{
    animation::GameAnimations,
    audio::{AudioPlayer, AudioResources},
    controls::{Controls, GameAction},
    game::{Game, GameEvent},
    game_graphics::{self, GameObject, GameResources, UIResources},
    game_physics::GamePhysics,
    menu::{Menu, MenuResult},
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
    animations: GameAnimations,
    player: AudioPlayer,
    event: Option<GameEvent>,
    controls: Controls,
}

impl GameRunner {
    pub fn new(animations: GameAnimations) -> Self {
        Self {
            menu: Menu::new(),
            state: GameState::StartMenu,
            game: Game::new(),
            physics: GamePhysics::new(),
            animations,
            player: AudioPlayer::new(),
            event: None,
            controls: Controls::default(),
        }
    }

    pub fn update(&mut self, events: FlushedMessages<(f64, WindowEvent)>) -> bool {
        let mut to_quit = false;
        let action = events
            .map(|(_, e)| {
                if let WindowEvent::Close = e {
                    to_quit = true
                }
                e
            })
            .find_map(|e| self.controls.convert(e));
        match self.state {
            GameState::StartMenu => {
                if let Some(ma) = action.and_then(GameAction::into_menu_action) {
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
                if let Some(pa) = action.and_then(GameAction::into_player_action) {
                    let event = self.game.update(pa);
                    self.animations.update();
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
                self.physics.step();
                if let Some(GameAction::Enter) = action {
                    self.state = GameState::StartMenu;
                    self.game = Game::new();
                    self.physics.reset();
                }
            }
        }
        to_quit
    }

    pub fn play_bgm(&mut self, resources: &AudioResources) {
        self.player.play(resources.bgm.clone());
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

    pub fn make_ui(&self, resources: &UIResources) -> Vec<GameObject> {
        match self.state {
            GameState::StartMenu => game_graphics::make_menu(&self.menu, resources),
            GameState::InGame => game_graphics::make_ui(&self.game, resources),
            GameState::GameOver => game_graphics::make_game_over_ui(&self.game, resources),
        }
    }

    fn make_game_scene(&self, resources: &GameResources) -> Vec<GameObject> {
        let mut scene = self.physics.make_scene(&self.game, resources);
        scene.push(game_graphics::make_player(
            &self.game,
            resources,
            &self.animations.chop,
        ));
        scene
    }

    pub fn make_scene(&self, resources: &GameResources) -> Vec<GameObject> {
        match self.state {
            GameState::StartMenu => vec![],
            GameState::InGame => self.make_game_scene(resources),
            GameState::GameOver => self.make_game_scene(resources),
        }
    }
}
