use std::fs::remove_file;

use crossterm::event::{KeyCode, KeyEvent};

use crate::constants::SAVE_FILE_PATH;

use super::{
    fs::{read_save_files, FileInfo},
    screen::{PauseMenu, PauseMenuState},
};

#[derive(Debug, Default)]
pub struct SaveState {
    pub save_files: Vec<FileInfo>,
    pub save_file_name: String,
    pub message: String,
    pub is_deleting: bool,
}

impl SaveState {
    pub fn new() -> SaveState {
        SaveState {
            save_files: read_save_files(SAVE_FILE_PATH).unwrap(),
            save_file_name: String::from(""),
            message: String::from(
                "Enter save file name. Press ESC to go back, ENTER to confirm or - to enter delete mode.",
            ),
            is_deleting: false
        }
    }
}

pub struct SaveHandler;

impl SaveHandler {
    pub fn handle_save(pause_menu: &mut PauseMenu) {
        let save_files: Vec<FileInfo> = read_save_files(SAVE_FILE_PATH).unwrap();

        if save_files.len() > 10 {
            pause_menu.save_state.message = String::from("You can only save 20 games!");
            return;
        }

        if pause_menu.save_state.save_file_name.len() == 0 {
            pause_menu.save_state.message = String::from("Please enter a name!");
            return;
        }

        let full_file_name: String = format!("{}.json", pause_menu.save_state.save_file_name);

        let existing_save: Option<&FileInfo> = save_files.iter().find(|x| x.name == full_file_name);

        if existing_save.is_some() {
            pause_menu.save_state.message = String::from("File with that name already exists!");
        } else {
            pause_menu.save_state.message = String::from("Successfully saved!");
            pause_menu.save_state.save_files = read_save_files(SAVE_FILE_PATH).unwrap();
            pause_menu.state = PauseMenuState::Saved
        }
    }

    pub fn handle_delete(pause_menu: &mut PauseMenu) {
        let save_files: Vec<FileInfo> = read_save_files(SAVE_FILE_PATH).unwrap();

        if pause_menu.save_state.save_file_name.len() == 0 {
            pause_menu.save_state.message = String::from("Please enter a name!");
            return;
        }

        let full_file_name: String = format!("{}.json", pause_menu.save_state.save_file_name);

        let existing_save: Option<&FileInfo> = save_files.iter().find(|x| x.name == full_file_name);

        if existing_save.is_some() {
            let _ = remove_file(format!("{}/{}", SAVE_FILE_PATH, full_file_name));
            pause_menu.save_state.save_files = read_save_files(SAVE_FILE_PATH).unwrap();
            pause_menu.save_state.message = String::from("DELETE MODE: Successfully deleted!");
        } else {
            pause_menu.save_state.message =
                String::from("DELETE MODE: File with that name doesn't exist!");
        }
    }

    pub fn handle_key_event(pause_menu: &mut PauseMenu, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(c) if c.is_alphanumeric() || c == '_' => {
                pause_menu.save_state.save_file_name.push(c);
            }
            KeyCode::Backspace => {
                pause_menu.save_state.save_file_name.pop();
            }
            KeyCode::Char('+') => {
                pause_menu.save_state.message = String::from(
                    "ADD MODE: Enter the name for save file. Press - for delete mode.",
                );
                pause_menu.save_state.is_deleting = false;
                pause_menu.save_state.save_file_name = String::new();
            }
            KeyCode::Char('-') => {
                pause_menu.save_state.message = String::from(
                    "DELETE MODE: Enter the name of file you want to delete. Press + to go back.",
                );
                pause_menu.save_state.is_deleting = true;
                pause_menu.save_state.save_file_name = String::new();
            }
            KeyCode::Esc => {
                pause_menu.save_state = SaveState::new();
                pause_menu.state = PauseMenuState::Wait;
            }
            KeyCode::Enter => {
                if pause_menu.save_state.is_deleting {
                    Self::handle_delete(pause_menu)
                } else {
                    Self::handle_save(pause_menu)
                }
            }
            _ => {}
        }
    }
}
