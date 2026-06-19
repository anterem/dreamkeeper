use cipher::{BlockModeDecrypt, KeyInit, block_padding::Pkcs7};
use known_folders::{KnownFolder, get_known_folder_path};
use std::{
    fs::read,
    io::{Cursor, Read},
    path::{Path, PathBuf},
    sync::Mutex,
    time::UNIX_EPOCH,
};

mod critters;
mod game_data;
mod today;

const KEY: [u8; 32] =
    hex_literal::hex!("62357168683873614a38556c444a557a545a5864325467366d626f3857386e35");

#[derive(Debug, thiserror::Error, serde::Serialize, specta::Type)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("{0}")]
    NotFound(String),
    #[error("No save file loaded")]
    NoSaveLoaded,
}

pub struct LoadedSave {
    pub contents: serde_json::Value,
    pub storefront: game_data::Storefront,
}

pub struct AppState {
    pub save: Mutex<Option<LoadedSave>>,
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

#[derive(serde::Serialize, specta::Type)]
pub struct SaveFile {
    path: PathBuf,
    storefront: game_data::Storefront,
    #[serde(rename = "lastModified")]
    modified_secs: u32,
}

fn decrypt_save_file(path: &Path) -> Result<serde_json::Value, AppError> {
    let ciphertext = read(path)?;

    let decryptor = ecb::Decryptor::<aes::Aes256>::new(&KEY.into());

    let plaintext = decryptor
        .decrypt_padded_vec::<Pkcs7>(&ciphertext)
        .map_err(|e| AppError::Parse(e.to_string()))?;

    let cursor = Cursor::new(plaintext);
    let mut archive = zip::ZipArchive::new(cursor).map_err(|e| AppError::Parse(e.to_string()))?;
    let mut save_file = archive
        .by_index(0)
        .map_err(|e| AppError::Parse(e.to_string()))?;

    let mut contents = String::new();
    save_file
        .read_to_string(&mut contents)
        .map_err(|e| AppError::Parse(e.to_string()))?;

    serde_json::from_str(&contents).map_err(|e| AppError::Parse(e.to_string()))
}

#[tauri::command]
#[specta::specta]
fn load_save_file(
    path: PathBuf,
    storefront: game_data::Storefront,
    state: tauri::State<AppState>,
) -> Result<(), AppError> {
    let contents = decrypt_save_file(&path)?;
    *state.save.lock().unwrap() = Some(LoadedSave {
        contents,
        storefront,
    });
    Ok(())
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
                .parse()
                .ok()?;
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
    let specta_builder =
        tauri_specta::Builder::<tauri::Wry>::new().commands(tauri_specta::collect_commands![
            get_save_files,
            load_save_file,
            game_data::get_item_names,
            critters::get_critters,
            today::get_today
        ]);

    #[cfg(debug_assertions)]
    specta_builder
        .export(
            specta_typescript::Typescript::default(),
            "../src/lib/bindings.ts",
        )
        .expect("failed to export ts bindings");

    tauri::Builder::default()
        .manage(AppState {
            save: Mutex::new(None),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(specta_builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
