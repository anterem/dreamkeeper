use known_folders::{KnownFolder, get_known_folder_path};
use std::path::PathBuf;

#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

fn get_game_folder() -> PathBuf {
    get_known_folder_path(KnownFolder::LocalAppDataLow)
        .expect("LocalAppDataLow exists on Windows")
        .join("Gameloft")
        .join("Disney Dreamlight Valley")
}

#[tauri::command]
fn get_save_files() -> Result<Vec<PathBuf>, AppError> {
    let game_path = get_game_folder();

    let entries = match std::fs::read_dir(&game_path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
        result => result?,
    };

    let save_files: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_ok_and(|ft| ft.is_dir()))
        .filter_map(|entry| {
            let path = entry.path();
            let profile = path.join("profile.json");
            profile.try_exists().unwrap_or(false).then_some(profile)
        })
        .collect();

    Ok(save_files)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_save_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
