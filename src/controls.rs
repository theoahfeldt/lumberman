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
        Self::new(Key::Up, Key::Down, Key::Left, Key::Right, Key::Enter)
    }
}

impl Controls {
    pub fn new(up: Key, down: Key, left: Key, right: Key, enter: Key) -> Self {
        let mut bindings = HashMap::new();
        bindings.insert(up, GameAction::Up);
        bindings.insert(down, GameAction::Down);
        bindings.insert(left, GameAction::Left);
        bindings.insert(right, GameAction::Right);
        bindings.insert(enter, GameAction::Enter);
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
