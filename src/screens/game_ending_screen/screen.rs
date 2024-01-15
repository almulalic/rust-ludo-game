use crossterm::event::{KeyCode, KeyEvent};

use crate::{entities::player::Player, tui::Tui};

pub struct GameEndingScreen {
    pub player: Player,
}

impl GameEndingScreen {
    pub fn new(player: Player) -> GameEndingScreen {
        GameEndingScreen { player }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Enter => {}
            _ => {}
        }
    }

    pub fn draw_ui(&mut self, tui: &mut Tui) {
        let _ = tui.draw_game_ending_screen(self);
    }
}
