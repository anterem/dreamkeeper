use super::critters::{self, Critter};
use super::villagers::{self, Villager};

#[derive(serde::Serialize, specta::Type)]
#[serde(tag = "status", rename_all = "camelCase")]
pub enum Section<T> {
    Ok { data: T },
    Error { error: String },
}

impl<T> From<Result<T, super::AppError>> for Section<T> {
    fn from(result: Result<T, super::AppError>) -> Self {
        match result {
            Ok(data) => Section::Ok { data },
            Err(error) => Section::Error {
                error: error.to_string(),
            },
        }
    }
}

#[derive(serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Today {
    critters: Section<Vec<Critter>>,
    villagers: Section<Vec<Villager>>,
}

#[tauri::command]
#[specta::specta]
pub fn get_today(
    state: tauri::State<super::AppState>,
    now_utc_secs: i64,
) -> Result<Today, super::AppError> {
    let guard = state.save.lock().unwrap();
    let loaded = guard.as_ref().ok_or(super::AppError::NoSaveLoaded)?;

    Ok(Today {
        critters: critters::collect(loaded, now_utc_secs).into(),
        villagers: villagers::collect(loaded, now_utc_secs).into(),
    })
}
