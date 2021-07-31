use std::collections::HashMap;

use glfw::{Action, Key, WindowEvent};

use crate::{game::PlayerAction, menu::MenuAction};

#[derive(Clone, Copy)]
pub enum GameAction {
    Left,
    Right,
    Down,
    Up,
    Enter,
}

impl GameAction {
    pub fn into_player_action(self) -> Option<PlayerAction> {
        match self {
            Self::Left => Some(PlayerAction::ChopLeft),
            Self::Right => Some(PlayerAction::ChopRight),
            _ => None,
        }
    }

    pub fn into_menu_action(self) -> Option<MenuAction> {
        match self {
            Self::Up => Some(MenuAction::Up),
            Self::Down => Some(MenuAction::Down),
            Self::Enter => Some(MenuAction::Select),
            _ => None,
        }
    }
}

pub struct Controls {
    bindings: HashMap<Key, GameAction>,
}

impl Default for Controls {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        bindings.insert(Key::Up, GameAction::Up);
        bindings.insert(Key::Left, GameAction::Left);
        bindings.insert(Key::Down, GameAction::Down);
        bindings.insert(Key::Right, GameAction::Right);

        bindings.insert(Key::W, GameAction::Up);
        bindings.insert(Key::A, GameAction::Left);
        bindings.insert(Key::S, GameAction::Down);
        bindings.insert(Key::D, GameAction::Right);

        bindings.insert(Key::I, GameAction::Up);
        bindings.insert(Key::J, GameAction::Left);
        bindings.insert(Key::K, GameAction::Down);
        bindings.insert(Key::L, GameAction::Right);

        bindings.insert(Key::Enter, GameAction::Enter);

        Self::new(bindings)
    }
}

impl Controls {
    pub fn new(bindings: HashMap<Key, GameAction>) -> Self {
        Self { bindings }
    }

    pub fn convert(&self, event: WindowEvent) -> Option<GameAction> {
        if let WindowEvent::Key(key, _, Action::Press, _) = event {
            self.bindings.get(&key).cloned()
        } else {
            None
        }
    }
}
