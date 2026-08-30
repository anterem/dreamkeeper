use std::collections::HashSet;

use serde_json::Value;

use super::areas::biome_of;
use super::{AppError, LoadedSave};

// ActivityItem LootPresentForagingOnline is the daily moonstone chest
const MOONSTONE_CHEST_ITEM_ID: u64 = 31_400_039;

// ActivityItem TimeBending!NormalRift is an open time rift
const RIFT_ITEM_ID: u64 = 31_300_076;

// 5 moonstones per vote, 50 maximum
const DREAMSNAP_VOTE_REWARD_CAP: u64 = 10;

#[derive(Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ChecklistFacts {
    moonstone_chest_biomes: Vec<Option<String>>,
    rift_biomes: Vec<Option<String>>,
    dream_snaps: Option<DreamSnaps>,
    scrooge_stores: Vec<ScroogeStore>,
}

#[derive(Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ScroogeStore {
    location: Option<String>,
    count: u32,
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
        dream_snaps: dream_snaps(save),
        scrooge_stores: scrooge_stores(loaded),
    })
}

fn building_grid(save: &Value, building_id: u64) -> Option<(u64, String)> {
    save.pointer("/World/GridCollection/Grids")
        .and_then(|v| v.as_object())
        .and_then(|grids| {
            grids.iter().find_map(|(key, grid)| {
                let grid_id = key.parse::<u64>().ok()?;
                let has_building =
                    grid.get("Objects")
                        .and_then(|v| v.as_object())
                        .is_some_and(|objects| {
                            objects.values().any(|obj| {
                                obj.get("ItemID").and_then(|v| v.as_u64()) == Some(building_id)
                            })
                        });
                has_building.then(|| {
                    let path = grid
                        .get("GridDataPath")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    (grid_id, path)
                })
            })
        })
}

fn grid_village(save: &Value, grid_id: u64) -> Option<u64> {
    save.pointer("/World/Villages")
        .and_then(|v| v.as_array())
        .and_then(|villages| {
            villages.iter().find(|village| {
                village
                    .get("Areas")
                    .and_then(|a| a.as_object())
                    .is_some_and(|areas| {
                        areas.values().any(|area| {
                            area.get("GridIDs")
                                .and_then(|g| g.as_array())
                                .is_some_and(|ids| {
                                    ids.iter().any(|id| id.as_u64() == Some(grid_id))
                                })
                        })
                    })
            })
        })
        .and_then(|village| village.get("SceneItemId"))
        .and_then(|v| v.as_u64())
}

fn building_zone(loaded: &LoadedSave, building_id: u64) -> Option<String> {
    let save = &loaded.contents;
    let (grid_id, grid_path) = building_grid(save, building_id)?;

    let map_name = grid_village(save, grid_id).and_then(|scene_id| {
        super::game_data::cached_menu_labels(&loaded.storefront)
            .ok()
            .and_then(|labels| labels.get(&format!("label_village_{scene_id}")).cloned())
    });

    map_name.or_else(|| biome_of(&grid_path).map(str::to_string))
}

fn scrooge_stores(loaded: &LoadedSave) -> Vec<ScroogeStore> {
    let save = &loaded.contents;
    let Some(stores) = save.pointer("/World/Stores").and_then(|v| v.as_array()) else {
        return Vec::new();
    };

    let owned = owned_item_ids(save);
    let mut result = Vec::new();

    for store in stores {
        let Some(displays) = store.get("Displays").and_then(|v| v.as_array()) else {
            continue;
        };

        let count = displays
            .iter()
            .filter_map(|display| {
                display
                    .pointer("/DisplayInfo/Slots")
                    .and_then(|v| v.as_array())
            })
            .flat_map(|slots| slots.iter())
            .filter(|slot| {
                slot.get("IsAvailable").and_then(|v| v.as_bool()) == Some(true)
                    && slot
                        .pointer("/Item/id")
                        .and_then(|v| v.as_u64())
                        .is_some_and(|id| !owned.contains(&id))
            })
            .count() as u32;

        if count > 0 {
            let location = store
                .get("BuildingItemID")
                .and_then(|v| v.as_u64())
                .and_then(|id| building_zone(loaded, id));
            result.push(ScroogeStore { location, count });
        }
    }
    result
}
fn owned_item_ids(save: &Value) -> HashSet<u64> {
    let Some(sets) = save
        .pointer("/Player/CollectionSets")
        .and_then(|v| v.as_array())
    else {
        return HashSet::new();
    };

    let mut owned = HashSet::new();
    for set in sets {
        let Some(groups) = set.get("GroupData").and_then(|v| v.as_array()) else {
            continue;
        };
        for group in groups {
            let Some(items) = group
                .get("GroupsCollectionItems")
                .and_then(|v| v.as_object())
            else {
                continue;
            };
            for (key, value) in items {
                if value.as_bool() != Some(true) {
                    continue;
                }
                let Ok(id) = key.parse::<u64>() else {
                    continue;
                };
                owned.insert(id);
            }
        }
    }
    owned
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
