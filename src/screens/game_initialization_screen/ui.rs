use std::collections::BTreeMap;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::Frame,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use crate::ui::{centered_rect, render_pause_menu};

use super::{
    player_order_state::RollState,
    screen::{GameInitializationScreen, GameInitializationStep},
};

const MAIN_COLOR: Color = Color::Rgb(0, 255, 6);

fn get_border(borders: Borders) -> Block<'static> {
    return Block::default()
        .borders(borders)
        .style(Style::default().fg(MAIN_COLOR));
}

pub fn render_game_initialization_screen(gis: &mut GameInitializationScreen, frame: &mut Frame) {
    let area = centered_rect(55, 70, frame.size());

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(15), // Main message
            Constraint::Length(3),  // Count Buttons
            Constraint::Length(6),  // Selected Message
            Constraint::Length(3),  // Order Message
            Constraint::Length(10), // Order list
            Constraint::Length(1),  // Confirmation
        ])
        .split(area);

    render_heading(main_layout[0], frame);

    if gis.step >= GameInitializationStep::PlayerNumberSelection {
        render_player_selection_buttons(gis, main_layout[1], frame);
    }

    if gis.step >= GameInitializationStep::PlayerPawnColorSelection {
        render_player_selection_message(gis, main_layout[2], frame);
        render_player_color_selection(gis, main_layout[3], frame);
    }

    if gis.step >= GameInitializationStep::PlayerOrderSelection {
        render_player_order_message(gis, main_layout[4], frame);
    }

    if gis.step >= GameInitializationStep::Confirmation {
        render_game_initialization_confirmation(gis, main_layout[5], frame);
    }

    render_pause_menu(gis.state, frame);
}

fn render_heading(layout: Rect, frame: &mut Frame) {
    let mut text = String::from(
    "
    ███████████████████████████████████████████████████████████████████████████████▀█████████████████████
    █▄─▄█▄─▀█▄─▄█▄─▄█─▄─▄─█▄─▄██▀▄─██▄─▄███▄─▄█░▄▄░▄█▄─▄▄─███─▄─▄─█─█─█▄─▄▄─███─▄▄▄▄██▀▄─██▄─▀█▀─▄█▄─▄▄─█
    ██─███─█▄▀─███─████─████─███─▀─███─██▀██─███▀▄█▀██─▄█▀█████─███─▄─██─▄█▀███─██▄─██─▀─███─█▄█─███─▄█▀█
    ▀▄▄▄▀▄▄▄▀▀▄▄▀▄▄▄▀▀▄▄▄▀▀▄▄▄▀▄▄▀▄▄▀▄▄▄▄▄▀▄▄▄▀▄▄▄▄▄▀▄▄▄▄▄▀▀▀▀▄▄▄▀▀▄▀▄▀▄▄▄▄▄▀▀▀▄▄▄▄▄▀▄▄▀▄▄▀▄▄▄▀▄▄▄▀▄▄▄▄▄▀
    
    "
    );

    text.push_str(
        &String::from(
            "
                Welcome to the game of \"Covjece ne ljudi se\"! \n
                Use <- and -> arrows to select the number of players and then press Enter to continue.
                Use space to roll the dice and backspace if you want to go one step back.
            ",
        )
    );

    let trimmed_text: String = text
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<&str>>()
        .join("\n");

    let heading = Paragraph::new(trimmed_text)
        .block(
            Block::default()
                .padding(Padding::new(0, 0, 1, 1))
                .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(MAIN_COLOR))
        .alignment(Alignment::Center);

    frame.render_widget(heading, layout);
}

fn render_player_selection_buttons(
    gis: &mut GameInitializationScreen,
    layout: Rect,
    frame: &mut Frame,
) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(18), // Left Padding/Border
            Constraint::Percentage(20), // Button 1
            Constraint::Percentage(2),  // Separator
            Constraint::Percentage(20), // Button 2
            Constraint::Percentage(2),  // Separator
            Constraint::Percentage(20), // Button 3
            Constraint::Percentage(18), // Right Padding/Border
        ])
        .split(layout);

    let button_ids = [1, 3, 5];

    frame.render_widget(get_border(Borders::LEFT), layout[0]);
    frame.render_widget(get_border(Borders::RIGHT), layout[6]);

    for (i, button) in gis.player_count_state.options.iter().enumerate() {
        frame.render_widget(button.clone(), layout[button_ids[i]]);
    }
}

fn render_player_selection_message(
    gis: &mut GameInitializationScreen,
    layout: Rect,
    frame: &mut Frame,
) {
    let text = format!(
        "You have selected {} players! \n\n\n Please select color for Player {}:",
        gis.player_count_state.selected_player_count, gis.pawn_color_state.label
    );

    let confirmation_message = Paragraph::new(text.trim())
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT)
                .padding(Padding::new(0, 0, 1, 1))
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(MAIN_COLOR))
        .alignment(Alignment::Center);

    frame.render_widget(confirmation_message, layout);
}

fn render_player_color_selection(
    gis: &mut GameInitializationScreen,
    layout: Rect,
    frame: &mut Frame,
) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10), // Left Padding/Border
            Constraint::Percentage(20), // First Button
            Constraint::Percentage(20), // Second Button
            Constraint::Percentage(20), // Third Button
            Constraint::Percentage(20), // Fourth Button
            Constraint::Percentage(10), // Right Padding/Border
        ])
        .split(layout);

    let button_ids = [1, 2, 3, 4];

    frame.render_widget(get_border(Borders::LEFT), layout[0]);
    frame.render_widget(get_border(Borders::RIGHT), layout[5]);

    for (i, button) in gis.pawn_color_state.options.iter().enumerate() {
        frame.render_widget(button.to_owned(), layout[button_ids[i]])
    }
}

fn render_player_order_message(
    gis: &mut GameInitializationScreen,
    layout: Rect,
    frame: &mut Frame,
) {
    let mut text_builder = String::new();

    text_builder.push_str("All players selected their colors! \n\n Players will now throw the dice to determin the order: \n\n");
    for (i, player) in gis.players.iter().enumerate() {
        let message = &format!("Player {} ({}): ", i + 1, player.pawn_color);

        if let Some(rolled_number) = gis.player_order_state.rolled_numbers.get(&i) {
            text_builder.push_str(&format!("{} Rolled {}!\n", message, rolled_number));
        } else {
            if i == gis.player_order_state.curr_id {
                text_builder.push_str(&format!(" {} Rolling...\n", message));
            } else {
                text_builder.push_str(&format!(" {} Waiting...\n", message));
            }
        }
    }

    if gis.player_order_state.roll_state == RollState::Rethrow {
        text_builder.push_str("\n Some players rolled the same number. Press Space to reroll.");
    }

    let confirmation_message = Paragraph::new(text_builder.trim())
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT)
                .padding(Padding::new(0, 0, 1, 1))
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(MAIN_COLOR))
        .alignment(Alignment::Center);

    frame.render_widget(confirmation_message, layout);
}

fn render_game_initialization_confirmation(
    gis: &mut GameInitializationScreen,
    layout: Rect,
    frame: &mut Frame,
) {
    let sorted_by_roll = gis
        .player_order_state
        .rolled_numbers
        .iter()
        .map(|(k, v)| (*v, *k))
        .collect::<BTreeMap<usize, usize>>();

    let mut players = Vec::new();

    for (_, player_id) in sorted_by_roll {
        if let Some(player) = gis.players.get(player_id) {
            players.push(format!("Player {} ({})", player_id + 1, player.pawn_color));
        }
    }

    let mut text = format!("Final order of players: {}. \n\n\n", players.join(", "));

    text.push_str(&format!("You are ready to go! Press Enter to continue."));

    let confirmation_message = Paragraph::new(text.trim())
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(MAIN_COLOR))
        .alignment(Alignment::Center);

    frame.render_widget(confirmation_message, layout);
}
