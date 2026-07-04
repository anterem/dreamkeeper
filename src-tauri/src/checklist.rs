use serde_json::Value;

use super::areas::{biome_of, biome_of_area_type};
use super::{AppError, LoadedSave};

// ActivityItem LootPresentForagingOnline is the daily moonstone chest
const MOONSTONE_CHEST_ITEM_ID: u64 = 31_400_039;

// ActivityItem TimeBending!NormalRift is an open time rift
const RIFT_ITEM_ID: u64 = 31_300_076;

// RecurringEvent ids for time rift spawns - Valley and Eternity Isle
const RIFT_SPAWN_AND_AREA_CHOICE_IDS: [(u64, u64); 2] = [
    (2_010_000_093, 2_010_700_002),
    (2_010_000_120, 2_010_700_004),
];

// 5 moonstones per vote, 50 maximum
const DREAMSNAP_VOTE_REWARD_CAP: u64 = 10;

#[derive(Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ChecklistFacts {
    moonstone_chest_biomes: Vec<Option<String>>,
    rift_biomes: Vec<Option<String>>,
    upcoming_rifts: Vec<UpcomingRift>,
    dream_snaps: Option<DreamSnaps>,
}

#[derive(Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpcomingRift {
    biome: Option<String>,
    spawn_secs: i64,
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
        moonstone_chest_biomes: placed_object_biomes(save, is_moonstone_chest),
        rift_biomes: placed_object_biomes(save, is_rift),
        upcoming_rifts: upcoming_rifts(save),
        dream_snaps: dream_snaps(save),
    })
}

fn placed_object_biomes(save: &Value, matches: fn(&Value) -> bool) -> Vec<Option<String>> {
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
            if matches(object) {
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

fn is_rift(object: &Value) -> bool {
    object.get("ItemID").and_then(|v| v.as_u64()) == Some(RIFT_ITEM_ID)
}

fn upcoming_rifts(save: &Value) -> Vec<UpcomingRift> {
    let Some(events) = save
        .pointer("/World/RecurringEvents")
        .and_then(|v| v.as_object())
    else {
        return Vec::new();
    };

    let mut rifts: Vec<UpcomingRift> = RIFT_SPAWN_AND_AREA_CHOICE_IDS
        .iter()
        .filter_map(|&(spawn_id, area_choice_id)| {
            let next = recurring_event(events, "ItemSpawning", spawn_id)?
                .get("NextOccurrence")?
                .as_str()?;
            let spawn_secs = chrono::DateTime::parse_from_rfc3339(next).ok()?.timestamp();
            let biome = recurring_event(events, "ChooseVillageArea", area_choice_id)
                .and_then(|event| event.get("ChosenVillageArea")?.as_str())
                .and_then(biome_of_area_type)
                .map(String::from);
            Some(UpcomingRift { biome, spawn_secs })
        })
        .collect();
    rifts.sort_by_key(|rift| rift.spawn_secs);
    rifts
}

fn recurring_event<'a>(
    events: &'a serde_json::Map<String, Value>,
    kind: &str,
    id: u64,
) -> Option<&'a Value> {
    events
        .values()
        .filter_map(|entry| entry.get(kind))
        .find(|event| event.get("RecurringEventItemID").and_then(|v| v.as_u64()) == Some(id))
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
        submit_needed: active
            .first()
            .is_some_and(|c| count_field(c, "SubmitCount") < 1),
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
