use ratatui::{
    layout::Alignment,
    style::Style,
    widgets::{Block, Borders},
};

use crate::screens::game_initialization_screen::ui::MAIN_COLOR;

pub const SAVE_FILE_PATH: &str = "./save_files";

pub fn border_block(borders: Borders, title: String) -> Block<'static> {
    Block::new()
        .borders(borders)
        .border_style(Style::default().fg(MAIN_COLOR))
        .title_alignment(Alignment::Center)
        .title(title)
}
