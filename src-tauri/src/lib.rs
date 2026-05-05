use known_folders::{KnownFolder, get_known_folder_path};
use std::{
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};

#[derive(Debug, thiserror::Error, serde::Serialize, specta::Type)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

#[derive(serde::Serialize, specta::Type)]
pub struct SaveFile {
    path: PathBuf,
    storefront: String,
    #[serde(rename = "lastModified")]
    modified_secs: u32,
}

fn get_game_folder() -> PathBuf {
    get_known_folder_path(KnownFolder::LocalAppDataLow)
        .expect("LocalAppDataLow exists on Windows")
        .join("Gameloft")
        .join("Disney Dreamlight Valley")
}

fn get_modified_secs(path: &Path) -> Option<u32> {
    let meta = path.metadata().ok()?;
    let modified = meta.modified().ok()?;
    let duration = modified.duration_since(UNIX_EPOCH).ok()?;
    Some(duration.as_secs() as u32)
}

#[tauri::command]
#[specta::specta]
fn get_save_files() -> Result<Vec<SaveFile>, AppError> {
    let game_path = get_game_folder();

    let entries = match std::fs::read_dir(&game_path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
        result => result?,
    };

    let save_files: Vec<SaveFile> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_ok_and(|ft| ft.is_dir()))
        .filter_map(|entry| {
            let save_file_path = entry.path().join("profile.json");
            if !save_file_path.try_exists().unwrap_or_default() {
                return None;
            }
            let storefront = entry
                .file_name()
                .to_string_lossy()
                .split('_')
                .next()
                .unwrap()
                .to_string();
            let modified_secs = get_modified_secs(&save_file_path).unwrap_or(0);

            Some(SaveFile {
                path: save_file_path,
                storefront,
                modified_secs,
            })
        })
        .collect();

    Ok(save_files)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let specta_builder = tauri_specta::Builder::<tauri::Wry>::new()
        .commands(tauri_specta::collect_commands![get_save_files]);

    #[cfg(debug_assertions)]
    specta_builder
        .export(
            specta_typescript::Typescript::default(),
            "../src/lib/bindings.ts",
        )
        .expect("failed to export ts bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(specta_builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
