use core::fmt;

use ratatui::{
    style::{Color, Stylize},
    widgets::Paragraph,
};
use serde::{de::Error, ser::SerializeStruct, Deserialize, Deserializer, Serialize};
use std::ops::Sub;

use crate::{ui::get_field, utils::hex_to_rgb};

use super::field::Field;

#[derive(
    Default, Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, Serialize, Deserialize, PartialOrd,
)]
pub enum PawnColor {
    #[default]
    RED,
    GREEN,
    BLUE,
    YELLOW,
}

impl Sub for PawnColor {
    type Output = isize;

    fn sub(self, other: PawnColor) -> isize {
        self as isize - other as isize
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct PawnColorPallet {
    primary: Color,
    hovered: Color,
    disabled: Color,
}

impl Serialize for PawnColorPallet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("PawnColorPallet", 3)?;
        let _ = state.serialize_field("primary", &self.primary.to_string());
        let _ = state.serialize_field("hovered", &self.hovered.to_string());
        let _ = state.serialize_field("disabled", &self.disabled.to_string());

        state.end()
    }
}

impl<'de> Deserialize<'de> for PawnColorPallet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PawnColorPalletVisitor;

        impl<'de> serde::de::Visitor<'de> for PawnColorPalletVisitor {
            type Value = PawnColorPallet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct PawnColorPallet")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut primary = None;
                let mut hovered = None;
                let mut disabled = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "primary" => {
                            primary = Some({
                                let hex_color: &str = map.next_value()?;
                                let (r, g, b) = hex_to_rgb(hex_color).map_err(Error::custom)?;
                                Color::Rgb(r, g, b)
                            })
                        }
                        "hovered" => {
                            hovered = Some({
                                let hex_color: &str = map.next_value()?;
                                let (r, g, b) = hex_to_rgb(hex_color).map_err(Error::custom)?;
                                Color::Rgb(r, g, b)
                            })
                        }
                        "disabled" => {
                            disabled = Some({
                                let hex_color: &str = map.next_value()?;
                                let (r, g, b) = hex_to_rgb(hex_color).map_err(Error::custom)?;
                                Color::Rgb(r, g, b)
                            })
                        }
                        _ => {}
                    }
                }

                let primary = primary.ok_or_else(|| Error::missing_field("primary"))?;
                let hovered = hovered.ok_or_else(|| Error::missing_field("hovered"))?;
                let disabled = disabled.ok_or_else(|| Error::missing_field("disabled"))?;

                Ok(PawnColorPallet {
                    primary,
                    hovered,
                    disabled,
                })
            }
        }

        deserializer.deserialize_struct(
            "PawnColorPallet",
            &["primary", "hovered", "disabled"],
            PawnColorPalletVisitor,
        )
    }
}

impl PawnColorPallet {
    pub fn new(primary: Color, hovered: Color, disabled: Color) -> PawnColorPallet {
        PawnColorPallet {
            primary,
            hovered,
            disabled,
        }
    }
}

fn get_color_pallete(color: &PawnColor) -> PawnColorPallet {
    match color {
        PawnColor::RED => PawnColorPallet {
            primary: Color::Rgb(255, 0, 0),
            hovered: Color::Rgb(139, 0, 0),
            disabled: Color::Rgb(139, 0, 0),
        },
        PawnColor::GREEN => PawnColorPallet {
            primary: Color::Rgb(0, 255, 0),
            hovered: Color::Rgb(1, 50, 32),
            disabled: Color::Rgb(1, 50, 32),
        },
        PawnColor::BLUE => PawnColorPallet {
            primary: Color::Rgb(0, 0, 255),
            hovered: Color::Rgb(0, 0, 139),
            disabled: Color::Rgb(0, 0, 139),
        },
        PawnColor::YELLOW => PawnColorPallet {
            primary: Color::Rgb(255, 255, 0),
            hovered: Color::Rgb(246, 190, 0),
            disabled: Color::Rgb(246, 190, 0),
        },
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pawn {
    pub id: usize,
    pub color: PawnColor,
    pub color_pallete: PawnColorPallet,
    pub player_id: usize,
    pub position: (usize, usize),
}

impl Pawn {
    pub fn new(id: usize, color: PawnColor, player_id: usize, position: (usize, usize)) -> Pawn {
        Pawn {
            id,
            color,
            player_id,
            color_pallete: get_color_pallete(&color),
            position,
        }
    }

    pub fn render(&mut self, field: &Field) -> Paragraph {
        let label: String = get_field(&format!(" {} ", &(self.id + 1).to_string()));

        if field.is_hovered {
            return Paragraph::new(label).fg(self.color_pallete.hovered);
        } else {
            return Paragraph::new(label).fg(self.color_pallete.primary);
        }
    }
}

impl fmt::Display for Pawn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for Pawn {
    fn default() -> Self {
        Pawn {
            id: Default::default(),
            color: Default::default(),
            player_id: Default::default(),
            position: Default::default(),
            color_pallete: Default::default(),
        }
    }
}
