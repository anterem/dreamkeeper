use std::{
    collections::HashMap,
    fs::read,
    io::{Cursor, Read},
    path::PathBuf,
    sync::{Arc, Mutex},
};
use winreg::{RegKey, enums};
use zip::ZipArchive;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "snake_case")]
pub enum Storefront {
    Steam,
    Epic,
    Microsoft,
}

impl std::str::FromStr for Storefront {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "steam" => Ok(Self::Steam),
            "epic" => Ok(Self::Epic),
            "microsoft" => Ok(Self::Microsoft),
            _ => Err(()),
        }
    }
}

// don't crash on unexpected format
fn read_varint(data: &[u8], pos: &mut usize) -> Option<u64> {
    let mut result = 0u64;
    let mut shift = 0;
    loop {
        let byte = *data.get(*pos)?;
        *pos += 1;
        result |= ((byte & 0x7f) as u64) << shift;
        if byte & 0x80 == 0 {
            return Some(result);
        }
        shift += 7;
        if shift >= 64 {
            return None;
        }
    }
}

fn read_bytes<'a>(data: &'a [u8], pos: &mut usize) -> Option<&'a [u8]> {
    let len = read_varint(data, pos)? as usize;
    let end = (*pos).checked_add(len)?;
    let slice = data.get(*pos..end)?;
    *pos = end;
    Some(slice)
}

fn parse_vdf_paths(content: &str) -> Vec<String> {
    content
        .lines()
        .filter(|line| line.trim().starts_with(r#""path""#))
        .filter_map(|line| {
            line.split('\t')
                .last()
                .map(|s| s.trim().trim_matches('"').to_string())
        })
        .collect()
}

fn parse_acf_field(content: &str, field: &str) -> Option<String> {
    let target = format!(r#""{}""#, field);
    content
        .lines()
        .find(|line| line.trim().starts_with(&target))
        .and_then(|line| {
            line.split('\t')
                .last()
                .map(|s| s.trim().trim_matches('"').to_string())
        })
}

fn find_steam_streaming_assets() -> Option<PathBuf> {
    let hkcu = RegKey::predef(enums::HKEY_CURRENT_USER);
    let steam_key = hkcu.open_subkey(r"Software\Valve\Steam").ok().or_else(|| {
        RegKey::predef(enums::HKEY_LOCAL_MACHINE)
            .open_subkey(r"Software\Valve\Steam")
            .ok()
    })?;
    let steam_path: String = steam_key.get_value("SteamPath").ok()?;

    let vdf_path = PathBuf::from(&steam_path)
        .join("config")
        .join("libraryfolders.vdf");
    let vdf_content = std::fs::read_to_string(&vdf_path).ok()?;
    let libraries = parse_vdf_paths(&vdf_content);

    for library in &libraries {
        let apps_dir = PathBuf::from(library).join("steamapps");
        let Ok(entries) = std::fs::read_dir(&apps_dir) else {
            continue;
        };

        for entry in entries {
            let Ok(entry) = entry else { continue };
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.starts_with("appmanifest_") || !name.ends_with(".acf") {
                continue;
            }
            let Ok(content) = std::fs::read_to_string(entry.path()) else {
                continue;
            };
            let Some(app_name) = parse_acf_field(&content, "name") else {
                continue;
            };
            if app_name == "Disney Dreamlight Valley" {
                let Some(installdir) = parse_acf_field(&content, "installdir") else {
                    continue;
                };
                let streaming_assets = PathBuf::from(library)
                    .join("steamapps")
                    .join("common")
                    .join(&installdir)
                    .join("ddv_Data")
                    .join("StreamingAssets");
                if streaming_assets.exists() {
                    return Some(streaming_assets);
                }
            }
        }
    }

    None
}

fn find_epic_streaming_assets() -> Option<PathBuf> {
    let programdata = std::env::var("PROGRAMDATA").ok()?;
    let manifests_dir = PathBuf::from(&programdata)
        .join("Epic")
        .join("EpicGamesLauncher")
        .join("Data")
        .join("Manifests");
    let Ok(entries) = std::fs::read_dir(&manifests_dir) else {
        return None;
    };

    for entry in entries {
        let Ok(entry) = entry else { continue };
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("item") {
            continue;
        }

        let Ok(content) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Ok(manifest) = serde_json::from_str::<serde_json::Value>(&content) else {
            continue;
        };
        if manifest.get("DisplayName").and_then(|v| v.as_str()) == Some("Disney Dreamlight Valley")
        {
            if let Some(install_location) = manifest.get("InstallLocation").and_then(|v| v.as_str())
            {
                let streaming_assets = PathBuf::from(install_location)
                    .join("ddv_Data")
                    .join("StreamingAssets");
                if streaming_assets.exists() {
                    return Some(streaming_assets);
                }
            }
        }
    }

    None
}

fn find_ms_streaming_assets() -> Option<PathBuf> {
    let hklm = RegKey::predef(enums::HKEY_LOCAL_MACHINE);
    let package_key = hklm
        .open_subkey(r"SOFTWARE\Microsoft\GamingServices\PackageRepository\Package")
        .ok()?;

    let app_ids: Vec<String> = package_key
        .enum_values()
        .filter_map(|r| r.ok().map(|(name, _)| name))
        .collect();

    for app_id in &app_ids {
        let packages_key_path = format!(
            r"SOFTWARE\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppModel\PackageRepository\Packages\{}",
            app_id
        );
        if let Ok(packages_key) = hklm.open_subkey(&packages_key_path) {
            if let Ok(path) = packages_key.get_value::<String, _>("Path") {
                let streaming_assets = PathBuf::from(path).join("ddv_Data").join("StreamingAssets");
                if streaming_assets.exists() {
                    return Some(streaming_assets);
                }
            }
        }
    }

    None
}

fn find_streaming_assets(storefront: &Storefront) -> Option<PathBuf> {
    match storefront {
        Storefront::Steam => find_steam_streaming_assets(),
        Storefront::Epic => find_epic_streaming_assets(),
        Storefront::Microsoft => find_ms_streaming_assets(),
    }
}

fn parse_id_map(bytes: &[u8]) -> HashMap<u32, String> {
    let mut map = HashMap::new();
    let mut pos = 0;

    while pos < bytes.len() {
        if bytes[pos] != 0x08 {
            pos += 1;
            continue;
        }
        let id_start = pos + 1;
        if id_start >= bytes.len() {
            break;
        }

        pos = id_start;
        let Some(id) = read_varint(bytes, &mut pos) else {
            break;
        };
        let id = id as u32;

        if !(120_000_000..120_300_000).contains(&id) {
            pos = id_start;
            continue;
        }

        if pos >= bytes.len() || bytes[pos] != 0x12 {
            pos = id_start;
            continue;
        }

        pos += 1;
        if pos >= bytes.len() {
            break;
        }

        let Some(data) = read_bytes(bytes, &mut pos) else {
            break;
        };
        let key = std::str::from_utf8(data).unwrap_or_default();
        if key.contains('!') {
            map.insert(id, key.to_string());
        }
    }

    map
}

fn parse_loc_map(bytes: &[u8]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut pos = 0;

    while pos < bytes.len() {
        let Some(outer_tag) = read_varint(bytes, &mut pos) else {
            break;
        };
        if outer_tag != 0x0a {
            if skip_field(bytes, &mut pos, outer_tag).is_none() {
                break;
            }
            continue;
        }

        let Some(submessage) = read_bytes(bytes, &mut pos) else {
            break;
        };
        let mut sub_pos = 0;
        let mut key = None;
        let mut name = None;

        while sub_pos < submessage.len() {
            let Some(inner_tag) = read_varint(submessage, &mut sub_pos) else {
                break;
            };
            match inner_tag {
                0x0a => {
                    let Some(data) = read_bytes(submessage, &mut sub_pos) else {
                        break;
                    };
                    key = Some(std::str::from_utf8(data).unwrap_or_default().to_string());
                }
                0x12 => {
                    let Some(data) = read_bytes(submessage, &mut sub_pos) else {
                        break;
                    };
                    name = Some(std::str::from_utf8(data).unwrap_or_default().to_string());
                }
                _ => {
                    if skip_field(submessage, &mut sub_pos, inner_tag).is_none() {
                        break;
                    }
                }
            }
        }

        if let (Some(k), Some(n)) = (key, name) {
            map.insert(k, n);
        }
    }

    map
}

// protobuf wire types
fn skip_field(data: &[u8], pos: &mut usize, tag: u64) -> Option<()> {
    match tag & 0x07 {
        0 => read_varint(data, pos).map(|_| ()),
        1 => {
            *pos += 8;
            Some(())
        }
        2 => read_bytes(data, pos).map(|_| ()),
        5 => {
            *pos += 4;
            Some(())
        }
        _ => None,
    }
}

fn load_item_names(storefront: &Storefront) -> Result<HashMap<u32, String>, super::AppError> {
    let streaming_assets = find_streaming_assets(storefront)
        .ok_or_else(|| super::AppError::NotFound("Could not locate game files".to_string()))?;

    let zip_path = streaming_assets
        .join("Localization")
        .join("LocDB_en-US.zip");
    let zip_bytes = read(&zip_path)?;
    let cursor = Cursor::new(zip_bytes);
    let mut archive = ZipArchive::new(cursor).map_err(|e| super::AppError::Parse(e.to_string()))?;

    let types = ["Companion", "Character", "ActivityItem"];
    let mut names = HashMap::new();

    for &t in &types {
        let item_path = streaming_assets
            .join("itemlist")
            .join(format!("{}.json", t));
        let item_bytes = read(&item_path)?;
        let id_map = parse_id_map(&item_bytes);

        let entry_name = format!("{}.locbin", t);
        let mut entry = archive.by_name(&entry_name).map_err(|_| {
            super::AppError::NotFound(format!("Could not find {} in LocDB archive", entry_name))
        })?;
        let mut loc_bytes = Vec::new();
        entry
            .read_to_end(&mut loc_bytes)
            .map_err(|e| super::AppError::Parse(e.to_string()))?;

        let loc_map = parse_loc_map(&loc_bytes);

        for (&id, key) in &id_map {
            let lookup_key = format!("{}_DisplayName", key);
            if let Some(display_name) = loc_map.get(&lookup_key) {
                names.insert(id, display_name.clone());
            }
        }
    }

    Ok(names)
}

static CACHE: Mutex<Option<(Storefront, Arc<HashMap<u32, String>>)>> = Mutex::new(None);

// cache item names to avoid re-parsing game files
pub fn cached_item_names(
    storefront: &Storefront,
) -> Result<Arc<HashMap<u32, String>>, super::AppError> {
    let mut cache = CACHE.lock().unwrap();
    if let Some((cached_storefront, names)) = cache.as_ref() {
        if cached_storefront == storefront {
            return Ok(names.clone());
        }
    }

    let names = Arc::new(load_item_names(storefront)?);
    *cache = Some((storefront.clone(), names.clone()));
    Ok(names)
}

#[tauri::command]
#[specta::specta]
pub fn get_item_names(storefront: Storefront) -> Result<HashMap<u32, String>, super::AppError> {
    Ok((*cached_item_names(&storefront)?).clone())
}
