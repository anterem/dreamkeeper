use std::collections::HashSet;

use serde_json::Value;

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
    ("LibraryArea01", "The Library of Lore"),
    ("LibraryArea02", "The Bind"),
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

pub(crate) fn biome_of(grid_path: &str) -> Option<&'static str> {
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

pub(crate) fn unlocked_biomes(save: &Value) -> Option<HashSet<&'static str>> {
    let villages = save.pointer("/World/Villages")?.as_array()?;
    let grids = save
        .pointer("/World/GridCollection/Grids")
        .and_then(|v| v.as_object());

    let mut unlocked = HashSet::new();
    for village in villages {
        let Some(areas) = village.get("Areas").and_then(|v| v.as_object()) else {
            continue;
        };
        for area in areas.values() {
            if area.get("Unlocked").and_then(|v| v.as_bool()) != Some(true) {
                continue;
            }
            if let Some(name) = biome_containing(area, grids) {
                unlocked.insert(name);
            }
        }
    }
    Some(unlocked)
}

#[cfg(test)]
pub(crate) fn known_biome_names() -> HashSet<&'static str> {
    BASE_BIOMES
        .iter()
        .chain(EXPANSION_BIOMES)
        .map(|&(_, name)| name)
        .collect()
}

fn biome_containing(
    area: &Value,
    grids: Option<&serde_json::Map<String, Value>>,
) -> Option<&'static str> {
    let grids = grids?;
    area.get("GridIDs")?
        .as_array()?
        .iter()
        .filter_map(|id| grids.get(&id.as_u64()?.to_string()))
        .filter_map(|grid| grid.get("GridDataPath")?.as_str())
        .find_map(biome_of)
}
