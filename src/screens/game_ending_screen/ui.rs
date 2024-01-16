use ratatui::{
    layout::Alignment,
    style::Style,
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

use crate::{screens::game_initialization_screen::ui::MAIN_COLOR, ui::centered_rect};

use super::screen::GameEndingScreen;

pub fn render_game_ending_screen(ges: &mut GameEndingScreen, frame: &mut Frame) {
    let area = centered_rect(50, 35, frame.size());
    let mut text = String::from(
        "
            █████▀██████████████████████████████████████████████
            █─▄▄▄▄██▀▄─██▄─▀█▀─▄█▄─▄▄─███─▄▄─█▄─█─▄█▄─▄▄─█▄─▄▄▀█
            █─██▄─██─▀─███─█▄█─███─▄█▀███─██─██▄▀▄███─▄█▀██─▄─▄█
            ▀▄▄▄▄▄▀▄▄▀▄▄▀▄▄▄▀▄▄▄▀▄▄▄▄▄▀▀▀▄▄▄▄▀▀▀▄▀▀▀▄▄▄▄▄▀▄▄▀▄▄▀
        ",
    );

    text.push_str(
        &format!(
            "
                \n
                GAME WINNER: Player {} - {} \n\n
                Thank you for playing our implementation of \"Covjece ne ljudi se\" written in rust! \n
                The project was written in 2024 as a part of Programming Languages course on International Burch University. \n
                Made by: Almir Mulalic & Elmin Softic
            ",
            ges.player.id + 1,
            ges.player.pawn_color
        )
    );

    let trimmed_text: String = text
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<&str>>()
        .join("\n");

    let content: Paragraph = Paragraph::new(trimmed_text)
        .block(
            Block::default()
                .padding(Padding::new(0, 0, 1, 1))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(MAIN_COLOR))
        .alignment(Alignment::Center);

    frame.render_widget(content, area);
}
