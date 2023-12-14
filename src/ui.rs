use ratatui::{
    prelude::{ Alignment, Frame },
    style::{ Color, Style },
    widgets::{ Block,  BorderType, Borders, Paragraph, Clear },
    layout::{ Layout, Direction, Constraint, Rect }
};

use crate::{screens::game_screen::{ GameScreen, GameState }, screens::game_screen::{GameInitializationScreen, GameMainScreen}};
use crate::screens::main_menu::MainMenu;

pub fn render_main_menu(main_menu: &mut MainMenu, frame: &mut Frame) {
    let area = centered_rect(30, 45, frame.size());

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Min(0)
        ])
        .split(area);
    
    for i in 0..main_menu.buttons.len() {
        if let Some(button) = main_menu.get_button(i) {
            frame.render_widget(button, layout[i]);
        }
    }
}

pub fn render_game_initialization_screen(game_initialization_screen: &mut GameInitializationScreen, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(format!("Game initialization screen"))
        .block(
            Block::default()
                    .title("Initialization screen")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center), 
        frame.size()
    );

    render_pause_menu(game_initialization_screen.state, frame)
}

pub fn render_game_main_screen(game_main_screen: &mut GameMainScreen, frame: &mut Frame) {
     frame.render_widget(
        Paragraph::new(format!("Game Main screen"))
        .block(
            Block::default()
                    .title("Main screen")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center), 
        frame.size()
    );

    render_pause_menu(game_main_screen.state, frame)
 }

pub fn render_pause_menu(game_state: GameState, frame: &mut Frame) {
   if game_state == GameState::PAUSED {
        let area = centered_rect(60, 20, frame.size());

        frame.render_widget(Clear, area);
        frame.render_widget(
            Block::default()
                .title("Pause Menu")
                .borders(Borders::ALL),
            area
        );
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
            .split(popup_layout[1])[1]
}
