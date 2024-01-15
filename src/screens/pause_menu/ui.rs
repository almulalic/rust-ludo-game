use std::rc::Rc;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Padding, Paragraph},
    Frame,
};

use crate::{
    constants::border_block, screens::game_initialization_screen::ui::MAIN_COLOR, ui::centered_rect,
};

use super::screen::{PauseMenu, PauseMenuState};

pub fn render_pause_menu(pause_menu: &mut PauseMenu, frame: &mut Frame) {
    let area = centered_rect(20, 50, frame.size());

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Top Gap
            Constraint::Length(5), // First Button
            Constraint::Length(1), // Mid Gap
            Constraint::Length(5), // Mid Button
            Constraint::Length(1), // Bottom Gap
            Constraint::Length(5), // Mid Button
            Constraint::Length(1), // Bottom Gap
            Constraint::Length(5), // Bottom Button
            Constraint::Min(0),
        ])
        .split(area);

    let button_indexes = [1, 3, 5, 7];
    for (i, button) in pause_menu.buttons.iter().enumerate() {
        frame.render_widget(button.to_owned(), layout[button_indexes[i]]);
    }

    if pause_menu.state == PauseMenuState::Saving || pause_menu.state == PauseMenuState::Saved {
        render_save_popup(pause_menu, frame);
    }

    if pause_menu.state == PauseMenuState::Loading {
        render_load_popup(pause_menu, frame);
    }
}

pub fn render_save_popup(pause_menu: &PauseMenu, frame: &mut Frame) {
    let popup_block = Block::default().style(Style::default().fg(Color::White));

    let area = centered_rect(50, 60, frame.size());
    frame.render_widget(Clear, area);

    frame.render_widget(popup_block, area);

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(2),
            Constraint::Percentage(10),
            Constraint::Percentage(3),
            Constraint::Percentage(75),
        ])
        .split(area);

    let message_block = Block::default()
        .title("Message")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(MAIN_COLOR))
        .title_alignment(Alignment::Center);

    frame.render_widget(
        Paragraph::new(pause_menu.save_state.message.clone())
            .alignment(Alignment::Center)
            .block(message_block),
        main_layout[0],
    );

    let file_name_block = Block::default()
        .title("Enter Save File Name")
        .borders(Borders::ALL)
        .padding(Padding::new(2, 0, 0, 0))
        .border_style(Style::default().fg(MAIN_COLOR))
        .title_alignment(Alignment::Center);

    frame.render_widget(
        Paragraph::new(format!("{}|", pause_menu.save_state.save_file_name.clone()))
            .block(file_name_block),
        main_layout[2],
    );

    render_file_table(pause_menu, main_layout[4], frame);
}

pub fn render_load_popup(pause_menu: &PauseMenu, frame: &mut Frame) {
    let popup_block = Block::default().style(Style::default().fg(Color::White));

    let area = centered_rect(50, 60, frame.size());
    frame.render_widget(Clear, area);

    frame.render_widget(popup_block, area);

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(2),
            Constraint::Percentage(10),
            Constraint::Percentage(3),
            Constraint::Percentage(75),
        ])
        .split(area);

    let message_block = Block::default()
        .title("Message")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(MAIN_COLOR))
        .title_alignment(Alignment::Center);

    frame.render_widget(
        Paragraph::new(pause_menu.load_state.message.clone())
            .alignment(Alignment::Center)
            .block(message_block),
        main_layout[0],
    );

    let file_name_block = Block::default()
        .title("Enter Save Name")
        .borders(Borders::ALL)
        .padding(Padding::new(2, 0, 0, 0))
        .border_style(Style::default().fg(MAIN_COLOR))
        .title_alignment(Alignment::Center);

    frame.render_widget(
        Paragraph::new(format!("{}|", pause_menu.load_state.load_file_name.clone()))
            .block(file_name_block),
        main_layout[2],
    );

    render_file_table(pause_menu, main_layout[4], frame);
}

pub fn get_columns(row: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(5),  // Left Margin
            Constraint::Percentage(50), // File Name
            Constraint::Percentage(20), // File Size
            Constraint::Percentage(20), // File Created Date
            Constraint::Percentage(5),  // Right Margin
        ])
        .split(row)
}

pub fn render_row(column: Rc<[Rect]>, labels: [&str; 3], frame: &mut Frame) {
    frame.render_widget(border_block(Borders::LEFT, String::new()), column[0]);

    frame.render_widget(Paragraph::new(labels[0]), column[1]);
    frame.render_widget(Paragraph::new(labels[1]), column[2]);
    frame.render_widget(Paragraph::new(labels[2]), column[3]);

    frame.render_widget(border_block(Borders::RIGHT, String::new()), column[4]);
}

pub fn render_file_table(pause_menu: &PauseMenu, layout: Rect, frame: &mut Frame) {
    let mut constraints: Vec<Constraint> = vec![Constraint::Percentage(5)];

    constraints.extend(
        pause_menu
            .save_state
            .save_files
            .iter()
            .map(|_| Constraint::Percentage(5))
            .collect::<Vec<Constraint>>(),
    );

    constraints.push(Constraint::Percentage(5));

    let file_names_table_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints.clone())
        .split(layout);

    frame.render_widget(
        border_block(Borders::TOP, String::from("Saved Files")),
        file_names_table_rows[0],
    );

    render_row(
        get_columns(file_names_table_rows[1]),
        ["File Name", "Size (kB)", "Created Date"],
        frame,
    );

    for (i, file) in pause_menu.save_state.save_files.iter().enumerate() {
        render_row(
            get_columns(file_names_table_rows[i + 2]),
            [
                &file.name.clone(),
                &format!("{} kB", file.size),
                &file.created.clone(),
            ],
            frame,
        );
    }

    frame.render_widget(
        border_block(Borders::BOTTOM, String::new()),
        file_names_table_rows[constraints.len() - 1],
    );
}
