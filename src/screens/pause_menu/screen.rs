use crate::custom_widgets::button::{Button, ButtonState, BLUE, GREEN, RED};
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;

use super::load_handler::LoadHandler;
use super::load_handler::LoadState;
use super::save_handler::{SaveHandler, SaveState};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PauseMenuButton {
    #[default]
    Resume,
    Save,
    Load,
    Exit,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PauseMenuState {
    #[default]
    Resume,
    Saving,
    Saved,
    Loading,
    Loaded,
    Wait,
    Exit,
}

#[derive(Debug, Default)]
pub struct PauseMenu<'a> {
    pub buttons: Vec<Button<'a, PauseMenuButton>>,
    pub selected_button_id: usize,
    pub should_quit: bool,
    pub state: PauseMenuState,
    pub save_state: SaveState,
    pub load_state: LoadState,
}

impl<'a> PauseMenu<'a> {
    pub fn new() -> PauseMenu<'a> {
        PauseMenu {
            buttons: vec![
                Button::new("Resume")
                    .value(PauseMenuButton::Resume)
                    .theme(GREEN)
                    .state(ButtonState::Selected),
                Button::new("Save")
                    .value(PauseMenuButton::Save)
                    .theme(BLUE)
                    .state(ButtonState::Normal),
                Button::new("Load")
                    .value(PauseMenuButton::Load)
                    .theme(BLUE)
                    .state(ButtonState::Normal),
                Button::new("Exit")
                    .value(PauseMenuButton::Exit)
                    .theme(RED)
                    .state(ButtonState::Normal),
            ],
            selected_button_id: 0,
            should_quit: false,
            state: PauseMenuState::Wait,
            save_state: SaveState::new(),
            load_state: LoadState::new(),
        }
    }

    pub fn get_button(&self, index: usize) -> Option<Button<PauseMenuButton>> {
        self.buttons.get(index).cloned()
    }

    pub fn get_selected_button(&self) -> Button<PauseMenuButton> {
        self.buttons.get(self.selected_button_id).unwrap().clone()
    }

    pub fn handle_menu_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Up => {
                self.change_selected_button(
                    (self.selected_button_id + self.buttons.len() - 1) % self.buttons.len(),
                );
                self.state = PauseMenuState::Wait;
            }
            KeyCode::Down => {
                self.change_selected_button((self.selected_button_id + 1) % self.buttons.len());
                self.state = PauseMenuState::Wait;
            }
            KeyCode::Esc => {
                self.state = PauseMenuState::Resume;
            }
            KeyCode::Enter => {
                self.handle_enter_press();
                self.save_state = SaveState::new();
            }
            _ => {}
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.state {
            PauseMenuState::Saving | PauseMenuState::Saved => {
                SaveHandler::handle_key_event(self, key_event)
            }
            PauseMenuState::Loading | PauseMenuState::Loaded => {
                LoadHandler::handle_key_event(self, key_event)
            }
            _ => self.handle_menu_key_event(key_event),
        }
    }

    fn handle_enter_press(&mut self) {
        if let Some(selected_button) = self.buttons.get_mut(self.selected_button_id) {
            self.should_quit = true;

            match selected_button.value {
                PauseMenuButton::Resume => self.state = PauseMenuState::Resume,
                PauseMenuButton::Save => self.state = PauseMenuState::Saving,
                PauseMenuButton::Load => self.state = PauseMenuState::Loading,
                PauseMenuButton::Exit => self.state = PauseMenuState::Exit,
            }
        } else {
            self.state = PauseMenuState::Wait
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
