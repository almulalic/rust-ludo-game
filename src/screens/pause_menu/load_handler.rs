use crossterm::event::{KeyCode, KeyEvent};

use crate::constants::SAVE_FILE_PATH;

use super::{
    fs::{read_save_files, FileInfo},
    screen::{PauseMenu, PauseMenuState},
};

#[derive(Debug, Default)]
pub struct LoadState {
    pub save_files: Vec<FileInfo>,
    pub load_file_name: String,
    pub message: String,
}

impl LoadState {
    pub fn new() -> LoadState {
        LoadState {
            save_files: read_save_files(SAVE_FILE_PATH).unwrap(),
            load_file_name: String::from(""),
            message: String::from(
                "Enter save file name to load. Press ESC to go back or ENTER to confirm.",
            ),
        }
    }
}

pub struct LoadHandler;

impl LoadHandler {
    pub fn handle_load(pause_menu: &mut PauseMenu) {
        let save_files: Vec<FileInfo> = read_save_files(SAVE_FILE_PATH).unwrap();

        if pause_menu.load_state.load_file_name.len() == 0 {
            pause_menu.load_state.message = String::from("Please enter a name!");
            return;
        }

        let full_file_name: String = format!("{}.json", pause_menu.load_state.load_file_name);

        let existing_save: Option<&FileInfo> = save_files.iter().find(|x| x.name == full_file_name);

        if existing_save.is_some() {
            pause_menu.load_state.message = String::from("Successfully loaded!");
            pause_menu.state = PauseMenuState::Loaded;
        } else {
            pause_menu.load_state.message = String::from("File with that name doesn't exist.");
            pause_menu.load_state.load_file_name = String::new();
        }
    }

    pub fn handle_key_event(pause_menu: &mut PauseMenu, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(c) if c.is_alphanumeric() || c == '_' => {
                pause_menu.load_state.load_file_name.push(c);
            }
            KeyCode::Backspace => {
                pause_menu.load_state.load_file_name.pop();
            }
            KeyCode::Esc => {
                pause_menu.load_state = LoadState::new();
                pause_menu.state = PauseMenuState::Wait;
            }
            KeyCode::Enter => Self::handle_load(pause_menu),
            _ => {}
        }
    }
}
