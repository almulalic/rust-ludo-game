use std::fs;

use crate::constants::SAVE_FILE_PATH;

use super::screen::GameMainScreen;

pub fn save_game(gms: &GameMainScreen, file_name: &str) -> Result<&'static str, &'static str> {
    let serialized = serde_json::to_string_pretty(gms).expect("Serialization failed");
    fs::write(
        format!("{}/{}.json", SAVE_FILE_PATH, file_name),
        &serialized,
    )
    .expect("Failed to write file");

    Ok("good")
}

pub fn load_game(file_name: &str) -> Result<GameMainScreen<'static>, &'static str> {
    let loaded_data = fs::read_to_string(format!("{}/{}.json", SAVE_FILE_PATH, file_name))
        .expect("Failed to read file");
    let deserialized: GameMainScreen =
        serde_json::from_str(&loaded_data).expect("Deserialization failed");

    Ok(deserialized)
}
