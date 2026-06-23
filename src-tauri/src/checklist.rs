use serde_json::Value;

use super::{AppError, LoadedSave};

// ActivityItem LootPresentForagingOnline is the daily moonstone chest
const MOONSTONE_CHEST_ITEM_ID: u64 = 31_400_039;

// 5 moonstones per vote, 50 maximum
const DREAMSNAP_VOTE_REWARD_CAP: u64 = 10;

static BASE_BIOMES: &[(&str, &str)] = &[
    ("BeachLevel", "Dazzle Beach"),
    ("MeadowLevel", "Peaceful Meadow"),
    ("ForestLevel", "Forest of Valor"),
    ("WetlandsLevel", "Glade of Trust"),
    ("SavannahLevel", "Sunlit Plateau"),
    ("SnowLevel", "Frosted Heights"),
    ("DarkLevel", "Forgotten Lands"),
    ("UrbanLevel", "Plaza"),
];

static EXPANSION_REGIONS: &[(&str, &str)] = &[
    ("Outpost", "Ancient's Landing"),
    ("Desert", "Glittering Dunes"),
    ("Jungle", "Wild Tangle"),
    ("Mythology", "Mythopia"),
    ("FairyTales", "Everafter"),
    ("Library", "The Bind"),
    ("Rockies", "Wishing Alps"),
    ("Fashion", "Glamour Gulch"),
    ("Wishland", "Pixie Acres"),
];

// to be confirmed
static EXPANSION_BIOMES: &[(&str, &str)] = &[
    // Eternity Isle — Ancient's Landing
    ("OutpostArea01", "The Docks"),
    ("OutpostArea02", "The Courtyard"),
    ("OutpostArea03", "The Overlook"),
    ("OutpostArea04", "The Ruins"),
    // Eternity Isle — Glittering Dunes
    ("DesertArea01", "The Plains"),
    ("DesertArea02", "The Oasis"),
    ("DesertArea03", "The Borderlands"),
    ("DesertArea04", "The Wastes"),
    // Eternity Isle — Wild Tangle
    ("JungleArea01", "The Grasslands"),
    ("JungleArea02", "The Promenade"),
    ("JungleArea03", "The Grove"),
    ("JungleArea04", "The Lagoon"),
    // Storybook Vale — The Bind
    ("LibraryArea01", "The Bind"),
    ("LibraryArea02", "The Library of Lore"),
    // Storybook Vale — Mythopia
    ("MythologyArea01", "The Elysian Fields"),
    ("MythologyArea02", "The Fiery Plains"),
    ("MythologyArea03", "The Statue's Shadow"),
    ("MythologyArea04", "Mount Olympus"),
    // Storybook Vale — Everafter
    ("FairyTalesArea01", "The Wild Woods"),
    ("FairyTalesArea02", "The Fallen Fortress"),
    ("FairyTalesArea03", "Teapot Falls"),
    ("FairyTalesArea04", "The Beanstalk Marshes"),
    // Wishblossom — Wishing Alps
    ("Rockies01", "Wishblossom Ranch"),
    ("Rockies02", "Ranch Highlands"),
    ("Rockies03", "Silver Summit"),
    ("Rockies04", "Delver Dale"),
    ("Rockies05", "Wishing Way"),
    // Wishblossom — Glamour Gulch
    ("Fashion01", "Runway River"),
    ("Fashion02", "Paisley Park"),
    ("Fashion03", "Modish Marsh"),
    ("Fashion04", "Haute Plateau"),
    // Wishblossom — Pixie Acres
    ("Wishland01", "Sundae Shores"),
    ("Wishland02", "Pixie Flats"),
    ("Wishland03", "Hunny Falls"),
    ("Wishland04", "Hundred-Acre Fields"),
];

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

fn biome_of(grid_path: &str) -> Option<&'static str> {
    let named = |table: &[(&'static str, &'static str)]| {
        table
            .iter()
            .find(|(token, _)| grid_path.contains(token))
            .map(|&(_, name)| name)
    };

    named(BASE_BIOMES)
        .or_else(|| named(EXPANSION_BIOMES))
        .or_else(|| named(EXPANSION_REGIONS))
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
