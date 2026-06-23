use serde_json::Value;

use super::areas::biome_of;
use super::{AppError, LoadedSave};

// ActivityItem LootPresentForagingOnline is the daily moonstone chest
const MOONSTONE_CHEST_ITEM_ID: u64 = 31_400_039;

// 5 moonstones per vote, 50 maximum
const DREAMSNAP_VOTE_REWARD_CAP: u64 = 10;

#[derive(Clone, serde::Serialize, specta::Type)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ChecklistItem {
    MoonstoneChest { biome: Option<String> },
    DreamSnapSubmission,
    DreamSnapVoting,
}

pub(crate) fn collect(loaded: &LoadedSave) -> Result<Vec<ChecklistItem>, AppError> {
    let save = &loaded.contents;
    let mut items = moonstone_chests(save);
    let active = active_dreamsnaps(save);
    if let Some(submitting) = active.first() {
        if count_field(submitting, "SubmitCount") < 1 {
            items.push(ChecklistItem::DreamSnapSubmission);
        }
    }
    if let Some(voting) = active.get(1) {
        if count_field(voting, "VoteCount") < DREAMSNAP_VOTE_REWARD_CAP {
            items.push(ChecklistItem::DreamSnapVoting);
        }
    }
    Ok(items)
}

fn moonstone_chests(save: &Value) -> Vec<ChecklistItem> {
    let Some(grids) = save
        .pointer("/World/GridCollection/Grids")
        .and_then(|v| v.as_object())
    else {
        return Vec::new();
    };

    let mut chests = Vec::new();
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
            if !is_moonstone_chest(object) {
                continue;
            }
            chests.push(ChecklistItem::MoonstoneChest {
                biome: biome.map(|name| name.to_string()),
            });
        }
    }
    chests
}

fn is_moonstone_chest(object: &Value) -> bool {
    object.get("ItemID").and_then(|v| v.as_u64()) == Some(MOONSTONE_CHEST_ITEM_ID)
        && object.get("State").is_none_or(|s| s.is_null())
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
