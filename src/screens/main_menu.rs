use crossterm::event::{ KeyCode };
use crate::{custom_widgets::button::{ Button, State, BLUE, RED, GREEN }, app::CurrentScreen, screens::main_menu};
use crossterm::event::{ KeyEvent, MouseEvent, MouseButton, MouseEventKind };
use crate::app::App;

pub struct MainMenu<'a> {
    pub buttons: [ Button<'a>; 3 ],
    pub selected_button_id: usize,
    pub should_quit: bool
}

impl<'a> MainMenu<'a> {
    pub fn new() -> MainMenu<'a> {
        MainMenu {
            buttons: [
                Button::new("New Game").action(String::from("NEWGAME")).theme(GREEN).state(State::Selected),
                Button::new("Load").action(String::from("LOAD")).theme(BLUE).state(State::Normal),
                Button::new("Exit").action(String::from("EXIT")).theme(RED).state(State::Normal)
            ],
            selected_button_id: 0,
            should_quit: false
        }
    }

    pub fn get_button(&self, index: usize) -> Option<Button> {
        self.buttons.get(index).cloned()
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, app: &mut App) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Up => self.change_selected_button((self.selected_button_id + self.buttons.len() - 1) % self.buttons.len()),
            KeyCode::Down => self.change_selected_button((self.selected_button_id + 1) % self.buttons.len()),
            KeyCode::Enter => self.handle_enter_press(app),
            _ => {}
        }
    }

    fn handle_enter_press(&mut self, app: &mut App) {
        if let Some(selected_button) = self.buttons.get_mut(self.selected_button_id) {
            self.should_quit = true;

            match selected_button.action.as_str() {
                "NEWGAME" => app.current_screen = CurrentScreen::GameScene,
                "LOAD" => app.current_screen = CurrentScreen::GameScene,
                "EXIT" => app.should_quit = true,
                _ => {}
            }
        }
    }

    fn change_selected_button(&mut self, new_button_id: usize) {
        if let Some(old_button) = self.buttons.get_mut(self.selected_button_id) {
            old_button.set_state(State::Normal);
        }

        if let Some(new_button) = self.buttons.get_mut(new_button_id) {
            new_button.set_state(State::Selected);
            self.selected_button_id = new_button_id;
        }
    }

    pub fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {
        match mouse_event.kind {
            MouseEventKind::Moved => {
                let old_selected_button = self.buttons.get(self.selected_button_id);

                self.selected_button_id = match mouse_event.column {
                    x if x < 15 => 0,
                    x if x < 30 => 1,
                    _ => 2,
                };
            }

            MouseEventKind::Down(MouseButton::Left) => {
            }
            _ => (),
        }
    }
}

