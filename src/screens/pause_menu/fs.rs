use chrono::{DateTime, Utc};
use std::fs;

#[derive(Debug)]
pub struct FileInfo {
    pub name: String,
    pub created: String,
    pub size: u64,
}

pub fn read_save_files(directory_path: &str) -> Result<Vec<FileInfo>, std::io::Error> {
    let entries = fs::read_dir(directory_path)?;

    let mut file_info_vec = Vec::new();

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            let metadata = fs::metadata(&path)?;
            let created: DateTime<Utc> = metadata.created().unwrap().into();

            let file_info = FileInfo {
                name: path.file_name().unwrap().to_string_lossy().to_string(),
                created: created.format("%Y-%m-%d %H:%M:%S").to_string(),
                size: metadata.len(),
            };

            file_info_vec.push(file_info);
        }
    }

    file_info_vec.sort_by(|a, b| b.created.cmp(&a.created));

    Ok(file_info_vec)
}
