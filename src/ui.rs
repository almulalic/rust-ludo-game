use crate::{
    entities::{
        field::{Field, FieldKind},
        pawn::PawnColor,
    },
    screens::{game_main_screen::screen::GameMainScreen, main_menu::MainMenu},
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{Alignment, Frame},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph},
};
use std::rc::Rc;

const MAIN_COLOR: Color = Color::Rgb(0, 255, 6);

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

fn get_rows(layout: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical) // Change to vertical direction
        .constraints([
            //            Constraint::Length(1), // Top Border
            Constraint::Length(4), // Row 1
            Constraint::Length(4), // Row 2
            Constraint::Length(4), // Row 3
            Constraint::Length(4), // Row 4
            Constraint::Length(4), // Row 5
            Constraint::Length(1), // Row 6 (gap)
            Constraint::Length(4), // Row 7
            Constraint::Length(1), // Row 8 (gap)
            Constraint::Length(4), // Row 9
            Constraint::Length(4), // Row 10
            Constraint::Length(4), // Row 11
            Constraint::Length(4), // Row 12
            Constraint::Length(4), // Row 13
                                   //           Constraint::Length(1), // Bottom Border
        ])
        .split(layout)
}

fn get_columns(row: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal) // Change to vertical direction
        .constraints([
            //           Constraint::Length(1),  // Left Border
            Constraint::Length(14), // Column 1
            Constraint::Length(14), // Column 2
            Constraint::Length(14), // Column 3
            Constraint::Length(14), // Column 4
            Constraint::Length(14), // Column 5
            Constraint::Length(0),  // Column 6 (gap)
            Constraint::Length(14), // Column 7
            Constraint::Length(0),  // Column 8 (gap)
            Constraint::Length(14), // Column 9
            Constraint::Length(14), // Column 10
            Constraint::Length(14), // Column 11
            Constraint::Length(14), // Column 12
            Constraint::Length(14), // Column 13
                                    //           Constraint::Length(1),  // Left Border
        ])
        .split(row)
}

pub fn get_field(middle: &str) -> String {
    format!("  ███\n██{}██\n  ███", middle)
}

pub fn render_game_main_screen(gms: &mut GameMainScreen, frame: &mut Frame) {
    let area = centered_rect(60, 100, frame.size());

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(18), // Game State
            Constraint::Percentage(2),  // Gap
            Constraint::Percentage(80), // Main Board
        ])
        .split(area);

    let game_state = Paragraph::new(format!(
        "\n CURRENT PLAYER: Player {} ({}) \n\n Roll: {} \n\n Message: \n {}",
        gms.get_current_player().pawn_color,
        gms.curr_player.id + 1,
        gms.curr_player
            .roll
            .map(|r| r.to_string())
            .unwrap_or_else(|| "Rolling...".to_string()),
        gms.message
    ))
    .block(
        Block::default()
            .title("Game State")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .style(Style::default().fg(MAIN_COLOR)),
    )
    .alignment(Alignment::Center);

    frame.render_widget(game_state, layout[0]);

    let rows = get_rows(layout[2]);

    for (i, row) in rows.iter().enumerate() {
        let columns = get_columns(*row);

        for (j, column) in columns.iter().enumerate() {
            let empty_field = &Paragraph::new(get_field("███"));

            let field: Field = gms.board[i][j];

            if !field.is_visible {
                continue;
            }
            if let Some(mut pawn) = field.pawn {
                match field.kind {
                    FieldKind::Gap => {}
                    _ => {
                        frame.render_widget(pawn.render(&field), *column);
                    }
                }
            } else {
                match field.kind {
                    FieldKind::Path => {
                        if field.is_hovered {
                            frame.render_widget(empty_field.clone().fg(Color::DarkGray), *column);
                        } else {
                            frame.render_widget(empty_field.clone().fg(Color::White), *column);
                        }
                    }
                    FieldKind::RedHome | FieldKind::RedStart | FieldKind::RedSafehouse => frame
                        .render_widget(
                            empty_field.clone().fg(
                                if field.is_hovered || !gms.playing_colors.contains(&PawnColor::RED)
                                {
                                    Color::Rgb(139, 0, 0)
                                } else {
                                    Color::Red
                                },
                            ),
                            *column,
                        ),
                    FieldKind::GreenHome | FieldKind::GreenStart | FieldKind::GreenSafehouse => {
                        frame.render_widget(
                            empty_field.clone().fg(
                                if field.is_hovered
                                    || !gms.playing_colors.contains(&PawnColor::GREEN)
                                {
                                    Color::Rgb(1, 50, 32)
                                } else {
                                    Color::Green
                                },
                            ),
                            *column,
                        )
                    }
                    FieldKind::BlueHome | FieldKind::BlueStart | FieldKind::BlueSafehouse => frame
                        .render_widget(
                            empty_field.clone().fg(
                                if field.is_hovered
                                    || !gms.playing_colors.contains(&PawnColor::BLUE)
                                {
                                    Color::Rgb(0, 0, 139)
                                } else {
                                    Color::Blue
                                },
                            ),
                            *column,
                        ),
                    FieldKind::YellowHome | FieldKind::YellowStart | FieldKind::YellowSafehouse => {
                        frame.render_widget(
                            empty_field.clone().fg(
                                if field.is_hovered
                                    || !gms.playing_colors.contains(&PawnColor::YELLOW)
                                {
                                    Color::Rgb(246, 190, 0)
                                } else {
                                    Color::Yellow
                                },
                            ),
                            *column,
                        )
                    }

                    _ => {}
                }
            }
        }
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
