pub enum MenuAction {
    Up,
    Down,
    Select,
}

#[derive(Clone, Copy)]
pub enum MenuResult {
    Start,
    Quit,
}

pub struct Menu {
    pub options: Vec<MenuResult>,
    pub selected_idx: usize,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            options: vec![MenuResult::Start, MenuResult::Quit],
            selected_idx: 0,
        }
    }

    pub fn update(&mut self, action: MenuAction) -> Option<MenuResult> {
        let mut result = None;
        match action {
            MenuAction::Up => self.selected_idx += self.options.len() + 1,
            MenuAction::Down => self.selected_idx += 1,
            MenuAction::Select => result = Some(self.selected()),
        }
        self.selected_idx = self.selected_idx % 2;
        result
    }

    pub fn selected(&self) -> MenuResult {
        self.options[self.selected_idx]
    }
}
