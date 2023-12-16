use color_eyre::owo_colors::colors::Yellow;
use ratatui::{
    prelude::{ Alignment, Frame },
    style::{ Color, Style },
    widgets::{ Block,  BorderType, Borders, Paragraph, Clear, Padding },
    layout::{ Layout, Direction, Constraint, Rect }
};

use crate::{screens::{ 
    main_menu::MainMenu, 
    game_screen::{ GameState, GameMainScreen },
    game_initialization_screen::{ GameInitializationScreen, GameInitializationStep, self }
}, custom_widgets::button::{Button, ButtonState, GREEN, BLUE, RED, YELLOW}, entities::pawn::PawnColor};

pub fn render_main_menu(main_menu: &mut MainMenu, frame: &mut Frame) {
    let area = centered_rect(20, 50, frame.size());

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(5),
            Constraint::Length(1),
            Constraint::Length(5),
            Constraint::Length(1),
            Constraint::Length(5),
            Constraint::Min(0)
        ])
        .split(area);
    
    let button_indexes = [ 1, 3, 5 ];
    for i in 0..main_menu.buttons.len() {
        if let Some(button) = main_menu.get_button(i) {
            frame.render_widget(button, layout[button_indexes[i]]);
        }
    }
}

pub fn render_game_initialization_screen(game_initialization_screen: &mut GameInitializationScreen, frame: &mut Frame) {
    let area = centered_rect(60, 70, frame.size());
    
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(4),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(5),
        ])
        .split(area);
   
    let section_ids = [ 0, 2, 4, 6, 8 ];

    render_heading(main_layout[section_ids[0]], frame);
    
    if game_initialization_screen.step >= GameInitializationStep::PLAYER_NUMBER_SELECTION {
        render_player_selection_buttons(game_initialization_screen, main_layout[section_ids[1]], frame);
    }

    if game_initialization_screen.step >= GameInitializationStep::PLAYER_PAWN_COLOR_SELECTION {
        render_player_selection_message(game_initialization_screen, main_layout[section_ids[2]], frame);
        render_player_color_selection(game_initialization_screen, main_layout[section_ids[3]], frame);
    }

    if game_initialization_screen.step >= GameInitializationStep::CONFIRMATION {
        render_game_initialization_confirmation(main_layout[section_ids[4]], frame);
    }

    render_pause_menu(game_initialization_screen.state, frame);
}

fn render_heading(layout: Rect, frame: &mut Frame) {
    let text = format!("
        Welcome to the game of \"Covjece ne ljudi se\"!
        Use <- and -> arrow to select the number of players and then press ⏎ to continue.

        Note: If you make any mistakes you can use Backspace to go back.
    ");

    let heading = Paragraph::new(text.trim())
        .block(
            Block::default()
                .title("Initialize The Game")
                .title_alignment(Alignment::Center)
                .padding(Padding::new(1, 0, 1, 0))
                .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);
    
    frame.render_widget(heading, layout);
}

fn render_player_selection_buttons(game_initialization_screen: &mut GameInitializationScreen, layout: Rect, frame: &mut Frame) {    
    let button_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), 
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(layout);
    
    let button_ids =  [ 1, 2, 3 ];

    for i in 0..game_initialization_screen.player_count_choice_buttons.len() {
        if let Some(button) = game_initialization_screen.get_button(i) {
            frame.render_widget(button, button_layout[button_ids[i]]);
        }
    }
}

fn render_player_selection_message(game_initialization_screen: &mut GameInitializationScreen, layout: Rect, frame: &mut Frame) {
    let text = format!("You have selected {} players! \n\n Please select color for player {}:",
        game_initialization_screen.selected_player_count_choice_id + 1,
        game_initialization_screen.selecting_player_id + 1
    );

    let confirmation_message = Paragraph::new(text.trim())
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT)
                .padding(Padding::new(1, 0, 1, 0))
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);
   
    frame.render_widget(confirmation_message, layout);
}

fn render_player_color_selection(game_initialization_screen: &mut GameInitializationScreen, layout: Rect, frame: &mut Frame) {
    
    let color_button_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10), 
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
        ])
        .split(layout);

    let button_ids = [ 1, 2, 3, 4 ];

    for i in 0..game_initialization_screen.player_color_choice_buttons.len() {
        if let Some(button) = game_initialization_screen.player_color_choice_buttons.get(i) {
            frame.render_widget(button.to_owned(), color_button_layout[button_ids[i]])
        }
    }
}

fn render_game_initialization_confirmation(layout: Rect, frame: &mut Frame) {
    let text = String::from("All players selected their colors, you are ready to go!. \n\n Press Enter to continue");

    let confirmation_message = Paragraph::new(text.trim())
        .block(
            Block::default()
                .padding(Padding::horizontal(1))
                .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);
   
    frame.render_widget(confirmation_message, layout);
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
                .borders(Borders::ALL)
                .style(Style::new().bg(Color::Black)),
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
