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
    ("DesertArea02", "The Wastes"),
    ("DesertArea03", "The Oasis"),
    ("DesertArea04", "The Borderlands"),
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

static BASE_AREA_TYPE_TOKENS: &[(&str, &str)] = &[
    ("Beach", "BeachLevel"),
    ("Meadow", "MeadowLevel"),
    ("Forest", "ForestLevel"),
    ("Wetland", "WetlandsLevel"),
    ("Savannah", "SavannahLevel"),
    ("Cliff", "SnowLevel"),
    ("DarkMountains", "DarkLevel"),
    ("Urban", "UrbanLevel"),
];

static EXPANSION_AREA_TYPE_TOKENS: &[(&str, &str)] = &[
    ("AtlanteanOutpost", "OutpostArea"),
    ("Desert", "DesertArea"),
    ("Jungle", "JungleArea"),
    ("Mythology", "MythologyArea"),
    ("FairyTales", "FairyTalesArea"),
    ("Library", "LibraryArea"),
];

pub(crate) fn biome_of_area_type(area_type: &str) -> Option<&'static str> {
    let name = area_type.strip_prefix("VillageAreaType_")?;

    let named = |table: &[(&'static str, &'static str)], token: &str| {
        table
            .iter()
            .find(|&&(t, _)| t == token)
            .map(|&(_, biome)| biome)
    };

    if let Some(&(_, token)) = BASE_AREA_TYPE_TOKENS.iter().find(|&&(t, _)| t == name) {
        return named(BASE_BIOMES, token);
    }

    let number_at = name.find(|c: char| c.is_ascii_digit())?;
    let (prefix, number) = name.split_at(number_at);
    let token_prefix = EXPANSION_AREA_TYPE_TOKENS
        .iter()
        .find(|&&(t, _)| t == prefix)
        .map_or(prefix, |&(_, token)| token);
    named(EXPANSION_BIOMES, &format!("{token_prefix}{number}"))
}

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

#[cfg(test)]
mod tests {
    use super::biome_of_area_type;

    #[test]
    fn every_area_type_maps_to_a_biome() {
        for (name, biome) in [
            ("VillageAreaType_Beach", "Dazzle Beach"),
            ("VillageAreaType_Cliff", "Frosted Heights"),
            ("VillageAreaType_DarkMountains", "Forgotten Lands"),
            ("VillageAreaType_Wetland", "Glade of Trust"),
            ("VillageAreaType_AtlanteanOutpost01", "The Docks"),
            ("VillageAreaType_Desert02", "The Wastes"),
            ("VillageAreaType_Desert03", "The Oasis"),
            ("VillageAreaType_Jungle04", "The Lagoon"),
            ("VillageAreaType_FairyTales03", "Teapot Falls"),
            ("VillageAreaType_Rockies03", "Silver Summit"),
            ("VillageAreaType_Wishland04", "Hundred-Acre Fields"),
        ] {
            assert_eq!(biome_of_area_type(name), Some(biome), "{name}");
        }
    }
}
