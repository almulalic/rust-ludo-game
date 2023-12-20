use crate::app::App;
use crate::app::CurrentScreen;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;

pub struct PauseMenu {
    pub should_quit: bool,
}

impl PauseMenu {
    pub fn new() -> PauseMenu {
        PauseMenu { should_quit: false }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, app: &mut App) {
        match key_event.code {
            KeyCode::Char('q') => {
                self.should_quit = true;
                app.should_quit = true
            }
            KeyCode::Esc => {
                if app.current_screen == CurrentScreen::GameScene {
                    self.should_quit = true;
                    app.current_screen = CurrentScreen::MainMenu;
                } else {
                    app.current_screen = CurrentScreen::GameScene;
                }
            }
            //KeyCode::Up => self.change_selected_button((self.selected_button_id + self.buttons.len() - 1) % self.buttons.len()),
            //KeyCode::Down => self.change_selected_button((self.selected_button_id + 1) % self.buttons.len()),
            //KeyCode::Enter => self.handle_enter_press(app),
            _ => {}
        }
    }
}
