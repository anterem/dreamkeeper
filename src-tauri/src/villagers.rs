use std::collections::HashMap;

use serde_json::Value;

use super::{AppError, LoadedSave};

// hardcoded for in-game order and to skip non villagers
static VILLAGER_ROSTER: &[u32] = &[
    // --- Valley ---
    10000130, // Aladdin
    10000131, // Jasmine
    10000132, // Alice
    10000133, // Cheshire Cat
    10000002, // Belle
    10000025, // Cogsworth
    10000007, // The Beast
    10000020, // Lumiere
    10000040, // Cinderella
    10000063, // The Fairy Godmother
    10000098, // The Forgotten
    10000099, // Mirabel
    10000034, // Anna
    10000044, // Elsa
    10000070, // Kristoff
    10000029, // Olaf
    10000187, // Hercules
    10000186, // Phil
    10000156, // Joy
    10000157, // Sadness
    10000179, // Lady
    10000180, // Tramp
    10000073, // Stitch
    10000113, // Daisy
    10000011, // Donald Duck
    10000004, // Goofy
    10000003, // Mickey Mouse
    10000046, // Minnie Mouse
    10000055, // Scrooge McDuck
    10000019, // Maui
    10000016, // Moana
    10000110, // Mike Wazowski
    10000109, // Sulley
    10000115, // Mulan
    10000116, // Mushu
    10000137, // Peter Pan
    10000182, // Pocahontas
    10000018, // Remy
    10000062, // Mother Gothel
    10000033, // Nala
    10000009, // Pumbaa
    10000061, // Scar
    10000030, // Simba
    10000010, // Timon
    10000053, // Ariel
    10000052, // Prince Eric
    10000060, // Ursula
    10000125, // Tiana
    10000035, // Merlin
    10000105, // Jack Skellington
    10000129, // Sally
    10000013, // Buzz Lightyear
    10000032, // Woody
    10000014, // WALL·E
    10000031, // Vanellope
    // --- Eternity Isle ---
    10000103, // Jafar
    10000043, // Gaston
    10000114, // Oswald
    10000027, // Rapunzel
    10000038, // EVE
    // --- Storybook Vale ---
    10000117, // Merida
    10000119, // Hades
    10000135, // Aurora
    10000127, // Maleficent
    10000059, // Flynn
    // --- Wishblossom Ranch ---
    10000162, // Cruella
    10000160, // Tinker Bell
    10000158, // Snow White
    10000166, // Tigger
];

#[derive(Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    Gardening,
    Fishing,
    Mining,
    Foraging,
    Digging,
    Timebending,
    SnippetCatching,
}

fn role_from_profession_id(id: u64) -> Option<Role> {
    match id {
        1_500_000_001 => Some(Role::Gardening),
        1_500_000_002 => Some(Role::Fishing),
        1_500_000_003 => Some(Role::Mining),
        1_500_000_004 => Some(Role::Foraging),
        1_500_000_005 => Some(Role::Digging),
        1_500_000_006 => Some(Role::Timebending),
        1_500_000_007 => Some(Role::SnippetCatching),
        _ => None,
    }
}

#[derive(Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum CollectionStatus {
    InVillage,
    InRealm,
    Locked,
}

#[derive(Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum GiftCategory {
    Produce,
    Meal,
    Flower,
    Fish,
    AnimalProduct,
    Gem,
    Material,
    Other,
}

// gift category is encoded in the item id band
fn gift_category(item_id: u32) -> GiftCategory {
    match item_id / 100_000 {
        302 => GiftCategory::Produce,
        303 => GiftCategory::Meal,
        308 => GiftCategory::Flower,
        310 => GiftCategory::Fish,
        311 => GiftCategory::AnimalProduct,
        316 => GiftCategory::Gem,
        317 => GiftCategory::Material,
        _ => GiftCategory::Other,
    }
}

#[derive(Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PreferredGift {
    pub item_id: u32,
    pub name: String,
    pub category: GiftCategory,
    pub discovered: bool,
    pub gifted: bool,
}

#[derive(Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Villager {
    pub id: u32,
    pub name: String,
    pub status: CollectionStatus,
    pub role: Option<Role>,
    pub friendship_level: u8,
    pub friendship_xp: u32,
    pub is_maxed: bool,
    pub gifts: Vec<PreferredGift>,
    pub last_gift_secs: Option<i64>,
}

fn status_of(entry: Option<&Value>) -> CollectionStatus {
    match entry.and_then(|c| c.get("Status")).and_then(|s| s.as_str()) {
        Some("CharacterStatus_InVillage") => CollectionStatus::InVillage,
        Some("CharacterStatus_InRealm") => CollectionStatus::InRealm,
        _ => CollectionStatus::Locked,
    }
}

pub(crate) fn collect(loaded: &LoadedSave) -> Result<Vec<Villager>, AppError> {
    let save = &loaded.contents;
    let storefront = &loaded.storefront;

    let names = super::game_data::cached_item_names(storefront)?;

    let by_id: HashMap<u32, &Value> = save
        .pointer("/World/Characters")
        .and_then(|v| v.as_array())
        .map(|chars| {
            chars
                .iter()
                .filter_map(|c| Some((c.pointer("/Base/id")?.as_u64()? as u32, c)))
                .collect()
        })
        .unwrap_or_default();

    let villagers = VILLAGER_ROSTER
        .iter()
        .map(|&id| {
            let name = names.get(&id).cloned().unwrap_or_default();
            let entry = by_id.get(&id).copied();
            let status = status_of(entry);

            let friendship_level = entry
                .and_then(|e| e.get("FriendshipLevel"))
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as u8;
            let friendship_xp = entry
                .and_then(|e| e.get("Friendship"))
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as u32;
            let role = entry
                .and_then(|e| e.get("ProfessionID"))
                .and_then(|v| v.as_u64())
                .and_then(role_from_profession_id);

            let last_gift_secs = entry
                .and_then(|e| e.get("LastGiftDate"))
                .and_then(|v| v.as_str())
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.timestamp());

            let discovered = entry
                .and_then(|e| e.get("PreferredItemStatus"))
                .and_then(|v| v.as_object());
            let gifts = entry
                .and_then(|e| e.get("PreferredItemSlots"))
                .and_then(|v| v.as_array())
                .map(|slots| {
                    slots
                        .iter()
                        .filter_map(|slot| {
                            let item_id = slot.get("PreferredItemId")?.as_u64()? as u32;
                            let gifted =
                                slot.get("Gifted").and_then(|v| v.as_bool()).unwrap_or(false);
                            let discovered = discovered
                                .and_then(|d| d.get(&item_id.to_string()))
                                .and_then(|s| s.as_str())
                                == Some("PreferredItemStatus_Discovered");
                            Some(PreferredGift {
                                item_id,
                                name: names.get(&item_id).cloned().unwrap_or_default(),
                                category: gift_category(item_id),
                                discovered,
                                gifted,
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();

            Villager {
                id,
                name,
                status,
                role,
                friendship_level,
                friendship_xp,
                is_maxed: friendship_level == 10,
                gifts,
                last_gift_secs,
            }
        })
        .collect();

    Ok(villagers)
}
