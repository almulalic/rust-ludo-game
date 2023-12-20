use crate::screens::{
    game_initialization_screen::player_order_state::RollState,
    game_main_screen::screen::{GameMainScreen, GameState},
    main_menu::MainMenu,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{Alignment, Frame},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph},
};
use std::{
    collections::{BTreeMap, HashMap},
    rc::Rc,
};

use crate::screens::game_initialization_screen::screen::{
    GameInitializationScreen, GameInitializationStep,
};

const MAIN_COLOR: Color = Color::Rgb(0, 255, 6);

fn get_border(borders: Borders) -> Block<'static> {
    return Block::default()
        .borders(borders)
        .style(Style::default().fg(MAIN_COLOR));
}

pub fn render_main_menu(main_menu: &mut MainMenu, frame: &mut Frame) {
    let area = centered_rect(20, 50, frame.size());

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Top Gap
            Constraint::Length(5), // First Button
            Constraint::Length(1), // Mid Gap
            Constraint::Length(5), // Mid Button
            Constraint::Length(1), // Bottom Gap
            Constraint::Length(5), // Bottom Button
            Constraint::Min(0),
        ])
        .split(area);

    let button_indexes = [1, 3, 5];
    for (i, button) in main_menu.buttons.iter().enumerate() {
        frame.render_widget(button.to_owned(), layout[button_indexes[i]]);
    }
}

pub fn render_game_initialization_screen(gis: &mut GameInitializationScreen, frame: &mut Frame) {
    let area = centered_rect(55, 90, frame.size());

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(15), // Main message
            Constraint::Length(3),  // Count Buttons
            Constraint::Length(5),  // Selected Message
            Constraint::Length(3),  // Order Message
            Constraint::Length(10), // Order list
            Constraint::Length(3),  // Confirmation
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
     &       String::from(
            "
                Welcome to the game of \"Covjece ne ljudi se\"!
                Use <- and -> arrows to select the number of players and then press Enter to continue.

                Note: If you make any mistakes you can use Backspace to go back.
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
                .padding(Padding::new(0, 0, 1, 2))
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

    for (i, button) in gis.count_state.options.iter().enumerate() {
        frame.render_widget(button.clone(), layout[button_ids[i]]);
    }
}

fn render_player_selection_message(
    gis: &mut GameInitializationScreen,
    layout: Rect,
    frame: &mut Frame,
) {
    let text = format!(
        "You have selected {} players! \n\n Please select color for player {}:",
        gis.count_state.selected_player_count, gis.pawn_color_state.label
    );

    let confirmation_message = Paragraph::new(text.trim())
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT)
                .padding(Padding::new(0, 0, 2, 2))
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

    text_builder.push_str("All players selected their colors! \n In this step, players will throw dice to determin the order.\n\n");

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
                .padding(Padding::horizontal(1))
                .borders(Borders::LEFT | Borders::RIGHT)
                .padding(Padding::new(0, 0, 1, 0))
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

    let mut text = format!("Final order of players: {}.", players.join(", "));

    text.push_str(&format!(
        "\n\nYou are ready to go! Press Enter to continue."
    ));

    let confirmation_message = Paragraph::new(text.trim())
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
                .padding(Padding::new(0, 0, 1, 1))
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(MAIN_COLOR))
        .alignment(Alignment::Center);

    frame.render_widget(confirmation_message, layout);
}

const BOARD: [[usize; 13]; 13] = [
    [4, 4, 0, 0, 1, 0, 1, 0, 1, 0, 0, 3, 3],
    [4, 4, 0, 0, 1, 0, 3, 0, 1, 0, 0, 3, 3],
    [0, 0, 0, 0, 1, 0, 3, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 3, 0, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 0, 3, 0, 1, 1, 1, 1, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 4, 4, 4, 4, 0, 0, 0, 2, 2, 2, 2, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 0, 5, 0, 1, 1, 1, 1, 1],
    [0, 0, 0, 0, 1, 0, 5, 0, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 5, 0, 1, 0, 0, 0, 0],
    [5, 5, 0, 0, 1, 0, 5, 0, 1, 0, 0, 2, 2],
    [5, 5, 0, 0, 1, 0, 1, 0, 1, 0, 0, 2, 2],
];

fn colorize_field<'a>(field: &'a Paragraph, board_id: usize) -> Paragraph<'a> {
    match board_id {
        1 => field.clone().fg(Color::White),
        2 => field.clone().fg(Color::Red),
        3 => field.clone().fg(Color::Green),
        4 => field.clone().fg(Color::Yellow),
        5 => field.clone().fg(Color::Blue),
        _ => field.clone(), // Adjust this case based on your requirements
    }
}

fn get_rows(frame: &mut Frame) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical) // Change to vertical direction
        .constraints([
            Constraint::Length(4), // Row 1
            Constraint::Length(4), // Row 2
            Constraint::Length(4), // Row 3
            Constraint::Length(4), // Row 4
            Constraint::Length(4), // Row 5
            Constraint::Length(2), // Row 6 (gap)
            Constraint::Length(4), // Row 7
            Constraint::Length(2), // Row 8 (gap)
            Constraint::Length(4), // Row 9
            Constraint::Length(4), // Row 10
            Constraint::Length(4), // Row 11
            Constraint::Length(4), // Row 12
            Constraint::Length(4), // Row 13
        ])
        .split(frame.size())
}

fn get_columns(row: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal) // Change to vertical direction
        .constraints([
            Constraint::Length(14), // Row 1
            Constraint::Length(14), // Row 2
            Constraint::Length(14), // Row 3
            Constraint::Length(14), // Row 4
            Constraint::Length(14), // Row 5
            Constraint::Length(0),  // Row 6 (gap)
            Constraint::Length(14), // Row 7
            Constraint::Length(0),  // Row 8 (gap)
            Constraint::Length(14), // Row 9
            Constraint::Length(14), // Row 10
            Constraint::Length(14), // Row 11
            Constraint::Length(14), // Row 12
            Constraint::Length(14), // Row 13
        ])
        .split(row)
}

fn get_path() -> HashMap<(usize, usize), usize> {
    return maplit::hashmap! {
    (0, 6) => 0,
    (0, 8) => 1,
    (1, 8) => 2,
    (2, 8) => 3,
    (3, 8) => 4,
    (4, 8) => 5,
    (4, 9) => 6,
    (4, 10) => 7,
    (4, 11) => 8,
    (4, 12) => 9,
    (6, 12) => 10,
    (8, 12) => 11,
    (8, 11) => 12,
    (8, 10) => 13,
    (8, 9) => 14,
    (8, 8) => 15,
    (9, 8) => 16,
    (10, 8) => 17,
    (11, 8) => 18,
    (12, 8) => 19,
    (12, 6) => 20,
    (12, 4) => 21,
    (11, 4) => 22,
    (10, 4) => 23,
    (9, 4) => 24,
    (8, 4) => 25,
    (8, 3) => 26,
    (8, 2) => 27,
    (8, 1) => 28,
    (8, 0) => 29,
    (6, 0) => 30,
    (4, 0) => 31,
    (4, 1) => 32,
    (4, 2) => 33,
    (4, 3) => 34,
    (4, 4) => 35,
    (3, 4) => 36,
    (2, 4) => 37,
    (1, 4) => 38,
    (0, 4) => 39,
    };
}

pub fn render_game_main_screen(gms: &mut GameMainScreen, frame: &mut Frame) {
    let rows = get_rows(frame);
    let path = get_path();

    for (i, row) in rows.iter().enumerate() {
        let columns = get_columns(*row);

        for (j, column) in columns.iter().enumerate() {
            let board_id = BOARD[i][j];

            let raw_field = &Paragraph::new(" ████ \n██  ██\n ████ ");

            if board_id == 1 {
                if let Some(path_id) = path.get(&(i, j)) {
                    if let Some(Some(pawn)) = gms.fields.get_mut(*path_id) {
                        frame.render_widget(raw_field.clone().fg(pawn.get_rgb_color()), *column)
                    } else {
                        frame.render_widget(colorize_field(raw_field, 1), *column);
                    }
                } else {
                    frame.render_widget(colorize_field(raw_field, 1), *column);
                }
            } else if board_id > 1 {
                frame.render_widget(colorize_field(raw_field, board_id), *column);
            }
        }
    }

    if let Some(rolled_number) = gms.current_player_roll {
        render_roll_message(rolled_number, frame);
    }

    render_pause_menu(gms.state, frame)
}

pub fn render_roll_message(rolled_number: usize, frame: &mut Frame) {
    let area = centered_rect(20, 20, frame.size());

    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(format!("U rolled {} !", rolled_number))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("ROLL")
                    .title_alignment(Alignment::Center),
            )
            .alignment(Alignment::Center),
        area,
    );
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
            area,
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
