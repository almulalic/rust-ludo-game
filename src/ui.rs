use std::collections::BTreeMap;

use ratatui::{
    style::{ Color, Style },
    prelude::{ Alignment, Frame },
    layout::{ Layout, Direction, Constraint, Rect },
    widgets::{ Block,  BorderType, Borders, Paragraph, Clear, Padding }
};

use crate::{screens::{ 
    main_menu::MainMenu, 
    game_screen::{ GameState, GameMainScreen }
}};
    
use crate::screens::game_initialization_screen::screen::{ GameInitializationScreen, GameInitializationStep };

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
    for (i, button) in main_menu.buttons.iter().enumerate() {
        frame.render_widget(button.to_owned(), layout[button_indexes[i]]);
    }
}

pub fn render_game_initialization_screen(gis: &mut GameInitializationScreen, frame: &mut Frame) {
    let area = centered_rect(60, 80, frame.size());
    
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(8),
            Constraint::Length(1),
            Constraint::Length(2)
        ])
        .split(area);
   
    let section_ids = [ 0, 2, 4, 6, 8, 10 ];

    render_heading(main_layout[section_ids[0]], frame);
    
    if gis.step >= GameInitializationStep::PLAYER_NUMBER_SELECTION {
        render_player_selection_buttons(gis, main_layout[section_ids[1]], frame);
    }

    if gis.step >= GameInitializationStep::PLAYER_PAWN_COLOR_SELECTION {
        render_player_selection_message(gis, main_layout[section_ids[2]], frame);
        render_player_color_selection(gis, main_layout[section_ids[3]], frame);
    }

    if gis.step >= GameInitializationStep::PLAYER_ORDER_SELECTION {
        render_player_order_message(gis, main_layout[section_ids[4]], frame);
    }

    if gis.step >= GameInitializationStep::CONFIRMATION {
        render_game_initialization_confirmation(gis, main_layout[section_ids[5]], frame);
    }

    render_pause_menu(gis.state, frame);
}

fn render_heading(layout: Rect, frame: &mut Frame) {
    let text = format!("
        Welcome to the game of \"Covjece ne ljudi se\"!
        Use <- and -> arrows to select the number of players and then press Enter to continue.

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

fn render_player_selection_buttons(gis: &mut GameInitializationScreen, layout: Rect, frame: &mut Frame) {    
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

    for (i, button) in gis.count_state.options.iter().enumerate() {
        frame.render_widget(button.clone(), button_layout[button_ids[i]]);
    }
}

fn render_player_selection_message(gis: &mut GameInitializationScreen, layout: Rect, frame: &mut Frame) {
    let text = format!("You have selected {} players! \n\n Please select color for player {}:",
        gis.count_state.selected_player_count,
        gis.pawn_color_state.label
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

fn render_player_color_selection(gis: &mut GameInitializationScreen, layout: Rect, frame: &mut Frame) {
    
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

    for (i, button) in gis.pawn_color_state.options.iter().enumerate() {
        frame.render_widget(button.to_owned(), color_button_layout[button_ids[i]])
    }
}

fn render_player_order_message(gis: &mut GameInitializationScreen, layout: Rect, frame: &mut Frame) {
    let mut text_builder = String::new();

    text_builder.push_str("All players selected their colors! \n\n In this step, players will throw dice to determin the order. \n\n");
    
    for (i, player) in gis.players.iter().enumerate() {
        let message = &format!("Player {} ({}): ", i + 1, player.pawn_color);
        
        if let Some(rolled_number) = gis.player_order_state.rolled_numbers.get(&i) {
            text_builder.push_str(&format!("{} Rolled {}!\n", message, rolled_number));
        } else {
            if i == gis.player_order_state.curr_id {
                text_builder.push_str(&format!(" {}: Rolling...\n", message));
            } else {
                text_builder.push_str(&format!(" {}: Waiting...\n", message));
            }
        }
    }

    let confirmation_message = Paragraph::new(text_builder.trim())
        .block(
            Block::default()
                .padding(Padding::horizontal(1))
                .borders(Borders::LEFT | Borders::RIGHT)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);
   
    frame.render_widget(confirmation_message, layout);
}

fn render_game_initialization_confirmation(gis: &mut GameInitializationScreen, layout: Rect, frame: &mut Frame) {
    let mut text = String::from("Final order of players: ");

    let sorted_by_roll = gis.player_order_state.rolled_numbers.iter().map(|(k ,v)| (*v, *k)).collect::<BTreeMap<usize, usize>>();
    
    for (_, player_id) in sorted_by_roll {
        if let Some(player) = gis.players.get(player_id) {
            text.push_str(&format!("Player {} ({}), ", player_id + 1, player.pawn_color));
        }
    }
    
    text.push_str("\n");
    text.push_str(&format!("You are now ready to go! Press Enter to continue."));

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
