use crate::app::App;
use crate::{
    app::CurrentScreen,
    custom_widgets::button::{Button, ButtonState, BLUE, GREEN, RED},
};
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;

pub struct MainMenu<'a> {
    pub buttons: Vec<Button<'a, String>>,
    pub selected_button_id: usize,
    pub should_quit: bool,
}

impl<'a> MainMenu<'a> {
    pub fn new() -> MainMenu<'a> {
        MainMenu {
            buttons: vec![
                Button::new("New Game")
                    .value(String::from("NEWGAME"))
                    .theme(GREEN)
                    .state(ButtonState::Selected),
                Button::new("Load")
                    .value(String::from("LOAD"))
                    .theme(BLUE)
                    .state(ButtonState::Normal),
                Button::new("Exit")
                    .value(String::from("EXIT"))
                    .theme(RED)
                    .state(ButtonState::Normal),
            ],
            selected_button_id: 0,
            should_quit: false,
        }
    }

    pub fn get_button(&self, index: usize) -> Option<Button<String>> {
        self.buttons.get(index).cloned()
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, app: &mut App) {
        match key_event.code {
            KeyCode::Up => self.change_selected_button(
                (self.selected_button_id + self.buttons.len() - 1) % self.buttons.len(),
            ),
            KeyCode::Down => {
                self.change_selected_button((self.selected_button_id + 1) % self.buttons.len())
            }
            KeyCode::Enter => self.handle_enter_press(app),
            _ => {}
        }
    }

    fn handle_enter_press(&mut self, app: &mut App) {
        if let Some(selected_button) = self.buttons.get_mut(self.selected_button_id) {
            self.should_quit = true;

            match selected_button.value.as_str() {
                "NEWGAME" => app.current_screen = CurrentScreen::GameScene,
                "LOAD" => app.current_screen = CurrentScreen::GameScene,
                "EXIT" => app.should_quit = true,
                _ => {}
            }
        }
    }

    fn change_selected_button(&mut self, new_button_id: usize) {
        if let Some(old_button) = self.buttons.get_mut(self.selected_button_id) {
            old_button.set_state(ButtonState::Normal);
        }

        if let Some(new_button) = self.buttons.get_mut(new_button_id) {
            new_button.set_state(ButtonState::Selected);
            self.selected_button_id = new_button_id;
        }
    }
}
