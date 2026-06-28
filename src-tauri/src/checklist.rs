use serde_json::Value;

use super::areas::biome_of;
use super::{AppError, LoadedSave};

// ActivityItem LootPresentForagingOnline is the daily moonstone chest
const MOONSTONE_CHEST_ITEM_ID: u64 = 31_400_039;

// 5 moonstones per vote, 50 maximum
const DREAMSNAP_VOTE_REWARD_CAP: u64 = 10;

#[derive(Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ChecklistFacts {
    moonstone_chest_biomes: Vec<Option<String>>,
    dream_snaps: Option<DreamSnaps>,
}

#[derive(Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DreamSnaps {
    submit_needed: bool,
    vote_needed: bool,
}

pub(crate) fn collect(loaded: &LoadedSave) -> Result<ChecklistFacts, AppError> {
    let save = &loaded.contents;
    Ok(ChecklistFacts {
        moonstone_chest_biomes: moonstone_chest_biomes(save),
        dream_snaps: dream_snaps(save),
    })
}

fn moonstone_chest_biomes(save: &Value) -> Vec<Option<String>> {
    let Some(grids) = save
        .pointer("/World/GridCollection/Grids")
        .and_then(|v| v.as_object())
    else {
        return Vec::new();
    };

    let mut biomes = Vec::new();
    for grid in grids.values() {
        let Some(objects) = grid.get("Objects").and_then(|v| v.as_object()) else {
            continue;
        };

        let path = grid
            .get("GridDataPath")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let biome = biome_of(path);

        for object in objects.values() {
            if is_moonstone_chest(object) {
                biomes.push(biome.map(|name| name.to_string()));
            }
        }
    }
    biomes
}

fn is_moonstone_chest(object: &Value) -> bool {
    object.get("ItemID").and_then(|v| v.as_u64()) == Some(MOONSTONE_CHEST_ITEM_ID)
        && object.get("State").is_none_or(|s| s.is_null())
}

fn dream_snaps(save: &Value) -> Option<DreamSnaps> {
    let stats = save
        .pointer("/Player/DesignChallenge/Stats")
        .and_then(|v| v.as_object())?;
    if stats.is_empty() {
        return None;
    }
    let active = active_dreamsnaps(save);
    Some(DreamSnaps {
        submit_needed: active.first().is_some_and(|c| count_field(c, "SubmitCount") < 1),
        vote_needed: active
            .get(1)
            .is_some_and(|c| count_field(c, "VoteCount") < DREAMSNAP_VOTE_REWARD_CAP),
    })
}

// newest competition for submission, second newest for voting
fn active_dreamsnaps(save: &Value) -> Vec<&Value> {
    let Some(stats) = save
        .pointer("/Player/DesignChallenge/Stats")
        .and_then(|v| v.as_object())
    else {
        return Vec::new();
    };

    let mut active: Vec<(u32, &Value)> = stats
        .iter()
        .filter_map(|(key, entry)| {
            let number = challenge_number(key)?;
            let reward_sent = entry
                .get("RewardMessageSent")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            (!reward_sent).then_some((number, entry))
        })
        .collect();
    active.sort_by(|a, b| b.0.cmp(&a.0));
    active.into_iter().map(|(_, entry)| entry).collect()
}

fn count_field(entry: &Value, field: &str) -> u64 {
    entry.get(field).and_then(|v| v.as_u64()).unwrap_or(0)
}

fn challenge_number(key: &str) -> Option<u32> {
    key.strip_prefix("DreamSnap")?
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .ok()
}
