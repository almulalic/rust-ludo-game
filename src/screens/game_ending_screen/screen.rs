use crossterm::event::KeyEvent;

use crate::{app::App, entities::player::Player, tui::Tui};

pub struct GameEndingScreen {
    pub player: Player,
}

impl GameEndingScreen {
    pub fn new(player: Player) -> GameEndingScreen {
        GameEndingScreen { player }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, app: &mut App) {
        match key_event.code {
            _ => {
                app.should_quit = true;
            }
        }
    }

    pub fn draw_ui(&mut self, tui: &mut Tui) {
        let _ = tui.draw_game_ending_screen(self);
    }
}
