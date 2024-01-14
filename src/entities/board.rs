use crate::debug_log;

use super::field::{Field, FieldKind};
use std::collections::{BTreeMap, HashMap};

pub fn reorder_path_map(
    path_map: &BTreeMap<usize, (usize, usize)>,
    offset: usize,
) -> BTreeMap<usize, (usize, usize)> {
    path_map
        .keys()
        .into_iter()
        .map(|&key| (key as i8, path_map[&key]))
        .map(|(key, value)| (((key + 10 * (5 - offset as i8)) % 40) as usize, value))
        .collect::<BTreeMap<usize, (usize, usize)>>()
}

pub fn extend_safehouses(
    path_map: &BTreeMap<usize, (usize, usize)>,
    safehouses_pos: [(usize, usize); 4],
) -> BTreeMap<usize, (usize, usize)> {
    let mut new_path_map = path_map.clone();

    (40..44).zip(safehouses_pos.iter()).for_each(|(i, &pos)| {
        new_path_map.insert(i, pos);
    });

    debug_log!(format!(
        "add_safehouses() \n    - player safehouses: {:?}\n    - new path: {:?} \n",
        safehouses_pos, new_path_map
    ));

    new_path_map
}

pub fn get_path_map() -> BTreeMap<usize, (usize, usize)> {
    return maplit::btreemap! {
        0 => (4, 0),
        1 => (4, 1),
        2 => (4, 2),
        3 => (4, 3),
        4 => (4, 4),
        5 => (3, 4),
        6 => (2, 4),
        7 => (1, 4),
        8 => (0, 4),
        9 => (0, 6),
        10 => (0, 8),
        11 => (1, 8),
        12 => (2, 8),
        13 => (3, 8),
        14 => (4, 8),
        15 => (4, 9),
        16 => (4, 10),
        17 => (4, 11),
        18 => (4, 12),
        19 => (6, 12),
        20 => (8, 12),
        21 => (8, 11),
        22 => (8, 10),
        23 => (8, 9),
        24 => (8, 8),
        25 => (9, 8),
        26 => (10, 8),
        27 => (11, 8),
        28 => (12, 8),
        29 => (12, 6),
        30 => (12, 4),
        31 => (11, 4),
        32 => (10, 4),
        33 => (9, 4),
        34 => (8, 4),
        35 => (8, 3),
        36 => (8, 2),
        37 => (8, 1),
        38 => (8, 0),
        39 => (6, 0),
    };
}

pub fn initialize_board() -> [[Field; 13]; 13] {
    let mut board = [
        [
            Field::new(FieldKind::RedHome, true, false, None),
            Field::new(FieldKind::RedHome, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::GreenStart, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::GreenHome, true, false, None),
            Field::new(FieldKind::GreenHome, true, false, None),
        ],
        [
            Field::new(FieldKind::RedHome, true, false, None),
            Field::new(FieldKind::RedHome, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::GreenSafehouse, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::GreenHome, true, false, None),
            Field::new(FieldKind::GreenHome, true, false, None),
        ],
        [
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::GreenSafehouse, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
        ],
        [
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::GreenSafehouse, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
        ],
        [
            Field::new(FieldKind::RedStart, true, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::GreenSafehouse, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Path, true, false, None),
        ],
        [
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
        ],
        [
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::RedSafehouse, true, false, None),
            Field::new(FieldKind::RedSafehouse, true, false, None),
            Field::new(FieldKind::RedSafehouse, true, false, None),
            Field::new(FieldKind::RedSafehouse, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::BlueSafehouse, true, false, None),
            Field::new(FieldKind::BlueSafehouse, true, false, None),
            Field::new(FieldKind::BlueSafehouse, true, false, None),
            Field::new(FieldKind::BlueSafehouse, true, false, None),
            Field::new(FieldKind::Path, true, false, None),
        ],
        [
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
        ],
        [
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::YellowSafehouse, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::BlueStart, true, false, None),
        ],
        [
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::YellowSafehouse, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
        ],
        [
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::YellowSafehouse, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
        ],
        [
            Field::new(FieldKind::YellowHome, true, false, None),
            Field::new(FieldKind::YellowHome, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::YellowSafehouse, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::BlueHome, true, false, None),
            Field::new(FieldKind::BlueHome, true, false, None),
        ],
        [
            Field::new(FieldKind::YellowHome, true, false, None),
            Field::new(FieldKind::YellowHome, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::YellowStart, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Path, true, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::Gap, false, false, None),
            Field::new(FieldKind::BlueHome, true, false, None),
            Field::new(FieldKind::BlueHome, true, false, None),
        ],
    ];

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            board[i][j].set_position((i, j));
        }
    }

    return board;
}
