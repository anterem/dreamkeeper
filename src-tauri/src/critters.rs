use std::collections::{HashMap, HashSet};
use std::ops::Range;

// tamed companions have different ids
const WILD_CRITTER_ITEM_IDS: Range<u32> = 120_100_000..120_200_000;

// bitmask - 0 to 23 (local time)
type AvailableHours = u32;

const fn hours(start: u32, end: u32) -> AvailableHours {
    assert!(start < end && end <= 24);
    (1u32 << end) - (1 << start)
}

enum Note {
    AriftInTime,
    SparkOfImagination,
    StorybookVale,
    WishblossomRanch,
    WishblossomRanchClearWeather,
    WishblossomRanchRainyWeather,
}

impl Note {
    fn as_str(&self) -> &'static str {
        match self {
            Note::AriftInTime => "Requires A Rift In Time expansion",
            Note::SparkOfImagination => "Requires The Spark of Imagination quest (A Rift In Time)",
            Note::StorybookVale => "Requires The Storybook Vale expansion",
            Note::WishblossomRanch => "Requires Wishblossom Ranch expansion",
            Note::WishblossomRanchClearWeather => {
                "Only during clear or light cloudy weather (Wishblossom Ranch)"
            }
            Note::WishblossomRanchRainyWeather => {
                "Only during heavy clouds or rainy weather (Wishblossom Ranch)"
            }
        }
    }
}

struct CritterDef {
    name: &'static str,
    species: &'static str,
    biome: &'static str,
    days: [AvailableHours; 7],
    note: Option<Note>,
}

const NA: AvailableHours = 0;
const AM: AvailableHours = hours(0, 12);
const PM: AvailableHours = hours(12, 24);
const ALL: AvailableHours = hours(0, 24);

static CRITTER_DATA: &[CritterDef] = &[
    CritterDef {
        name: "Emerald Sunbird",
        species: "Sunbirds",
        biome: "Sunlit Plateau",
        days: [PM, NA, ALL, ALL, NA, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Golden Sunbird",
        species: "Sunbirds",
        biome: "Sunlit Plateau",
        days: [AM, NA, ALL, NA, ALL, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Orchid Sunbird",
        species: "Sunbirds",
        biome: "Sunlit Plateau",
        days: [NA, NA, NA, NA, NA, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Red Sunbird",
        species: "Sunbirds",
        biome: "Sunlit Plateau",
        days: [PM, ALL, NA, NA, ALL, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Turquoise Sunbird",
        species: "Sunbirds",
        biome: "Sunlit Plateau",
        days: [AM, ALL, NA, ALL, NA, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Black Squirrel",
        species: "Squirrels",
        biome: "Plaza",
        days: [AM, NA, ALL, NA, ALL, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Classic Squirrel",
        species: "Squirrels",
        biome: "Plaza",
        days: [PM, ALL, NA, NA, ALL, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Gray Squirrel",
        species: "Squirrels",
        biome: "Plaza",
        days: [PM, NA, ALL, ALL, NA, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Red Squirrel",
        species: "Squirrels",
        biome: "Plaza",
        days: [AM, ALL, NA, ALL, NA, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "White Squirrel",
        species: "Squirrels",
        biome: "Plaza",
        days: [ALL, NA, NA, NA, NA, NA, NA],
        note: None,
    },
    CritterDef {
        name: "Blue Crocodile",
        species: "Crocodiles",
        biome: "Glade of Trust",
        days: [AM, ALL, NA, ALL, NA, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Classic Crocodile",
        species: "Crocodiles",
        biome: "Glade of Trust",
        days: [PM, ALL, NA, NA, ALL, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Golden Crocodile",
        species: "Crocodiles",
        biome: "Glade of Trust",
        days: [AM, NA, ALL, NA, ALL, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Pink Crocodile",
        species: "Crocodiles",
        biome: "Glade of Trust",
        days: [NA, NA, NA, NA, NA, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Red Crocodile",
        species: "Crocodiles",
        biome: "Glade of Trust",
        days: [PM, NA, ALL, ALL, NA, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "White Crocodile",
        species: "Crocodiles",
        biome: "Glade of Trust",
        days: [ALL, NA, NA, NA, NA, NA, NA],
        note: None,
    },
    CritterDef {
        name: "Black Rabbit",
        species: "Rabbits",
        biome: "Peaceful Meadow",
        days: [AM, ALL, NA, ALL, NA, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Brown Rabbit",
        species: "Rabbits",
        biome: "Peaceful Meadow",
        days: [PM, NA, ALL, ALL, NA, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Calico Rabbit",
        species: "Rabbits",
        biome: "Peaceful Meadow",
        days: [NA, NA, NA, NA, ALL, NA, NA],
        note: None,
    },
    CritterDef {
        name: "Classic Rabbit",
        species: "Rabbits",
        biome: "Peaceful Meadow",
        days: [PM, ALL, NA, NA, ALL, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "White Rabbit",
        species: "Rabbits",
        biome: "Peaceful Meadow",
        days: [AM, NA, ALL, NA, ALL, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Blue Raven",
        species: "Ravens",
        biome: "Forgotten Lands",
        days: [AM, NA, ALL, NA, ALL, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Brown Raven",
        species: "Ravens",
        biome: "Forgotten Lands",
        days: [NA, NA, ALL, NA, NA, NA, NA],
        note: None,
    },
    CritterDef {
        name: "Classic Raven",
        species: "Ravens",
        biome: "Forgotten Lands",
        days: [PM, ALL, NA, NA, ALL, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Red Raven",
        species: "Ravens",
        biome: "Forgotten Lands",
        days: [AM, ALL, NA, ALL, NA, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "White Raven",
        species: "Ravens",
        biome: "Forgotten Lands",
        days: [PM, NA, ALL, ALL, NA, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Black Sea Turtle",
        species: "Sea Turtles",
        biome: "Dazzle Beach",
        days: [NA, ALL, NA, NA, NA, NA, NA],
        note: None,
    },
    CritterDef {
        name: "Brown Sea Turtle",
        species: "Sea Turtles",
        biome: "Dazzle Beach",
        days: [AM, ALL, NA, ALL, NA, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Classic Sea Turtle",
        species: "Sea Turtles",
        biome: "Dazzle Beach",
        days: [PM, ALL, NA, NA, ALL, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Purple Sea Turtle",
        species: "Sea Turtles",
        biome: "Dazzle Beach",
        days: [AM, NA, ALL, NA, ALL, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "White Sea Turtle",
        species: "Sea Turtles",
        biome: "Dazzle Beach",
        days: [PM, NA, ALL, ALL, NA, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Black Fox",
        species: "Foxes",
        biome: "Frosted Heights",
        days: [PM, NA, ALL, ALL, NA, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Blue Fox",
        species: "Foxes",
        biome: "Frosted Heights",
        days: [AM, NA, ALL, NA, ALL, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Classic Fox",
        species: "Foxes",
        biome: "Frosted Heights",
        days: [AM, ALL, NA, ALL, NA, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Red Fox",
        species: "Foxes",
        biome: "Frosted Heights",
        days: [NA, NA, NA, NA, NA, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "White Fox",
        species: "Foxes",
        biome: "Frosted Heights",
        days: [PM, ALL, NA, NA, ALL, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Black Raccoon",
        species: "Raccoons",
        biome: "Forest of Valor",
        days: [PM, NA, ALL, ALL, NA, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Blue Raccoon",
        species: "Raccoons",
        biome: "Forest of Valor",
        days: [NA, NA, NA, ALL, NA, NA, NA],
        note: None,
    },
    CritterDef {
        name: "Classic Raccoon",
        species: "Raccoons",
        biome: "Forest of Valor",
        days: [PM, ALL, NA, NA, ALL, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Red Raccoon",
        species: "Raccoons",
        biome: "Forest of Valor",
        days: [AM, ALL, NA, ALL, NA, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "White Raccoon",
        species: "Raccoons",
        biome: "Forest of Valor",
        days: [AM, NA, ALL, NA, ALL, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Black and White Capybara",
        species: "Capybaras",
        biome: "The Promenade",
        days: [PM, NA, ALL, NA, ALL, PM, PM],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Blue Striped Capybara",
        species: "Capybaras",
        biome: "The Grove",
        days: [NA, AM, AM, AM, ALL, NA, ALL],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Classic Capybara",
        species: "Capybaras",
        biome: "The Grasslands",
        days: [ALL; 7],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Gray Spotted Capybara",
        species: "Capybaras",
        biome: "The Lagoon",
        days: [ALL, PM, PM, PM, NA, ALL, NA],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Red and White Striped Capybara",
        species: "Capybaras",
        biome: "The Lagoon",
        days: [NA, NA, NA, NA, NA, NA, ALL],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Toon Capybara",
        species: "Capybaras",
        biome: "The Grove",
        days: [ALL; 7],
        note: Some(Note::SparkOfImagination),
    },
    CritterDef {
        name: "Blue and Red Striped Cobra",
        species: "Cobras",
        biome: "The Wastes",
        days: [ALL, PM, PM, NA, ALL, NA, PM],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Classic Cobra",
        species: "Cobras",
        biome: "The Plains",
        days: [ALL; 7],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Green and White Striped Cobra",
        species: "Cobras",
        biome: "The Borderlands",
        days: [NA, NA, NA, NA, ALL, NA, NA],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Pink Spotted Cobra",
        species: "Cobras",
        biome: "The Oasis",
        days: [AM, ALL, NA, AM, AM, ALL, NA],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Toon Cobra",
        species: "Cobras",
        biome: "The Wastes",
        days: [ALL; 7],
        note: Some(Note::SparkOfImagination),
    },
    CritterDef {
        name: "Yellow and Purple Striped Cobra",
        species: "Cobras",
        biome: "The Borderlands",
        days: [PM, NA, ALL, PM, PM, NA, ALL],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Beige Monkey",
        species: "Monkeys",
        biome: "The Ruins",
        days: [ALL, PM, NA, PM, ALL, PM, NA],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Black and Brown Monkey",
        species: "Monkeys",
        biome: "The Courtyard",
        days: [PM, NA, PM, ALL, PM, NA, ALL],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Black and Gray Monkey",
        species: "Monkeys",
        biome: "The Ruins",
        days: [NA, NA, ALL, NA, NA, NA, NA],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Classic Monkey",
        species: "Monkeys",
        biome: "The Docks",
        days: [ALL; 7],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Red and Beige Monkey",
        species: "Monkeys",
        biome: "The Overlook",
        days: [NA, AM, ALL, AM, NA, AM, ALL],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Toon Monkey",
        species: "Monkeys",
        biome: "The Docks",
        days: [ALL; 7],
        note: Some(Note::SparkOfImagination),
    },
    CritterDef {
        name: "Brown Owl",
        species: "Owls",
        biome: "The Library of Lore",
        days: [NA, ALL, ALL, ALL, ALL, ALL, NA],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Dark Owl",
        species: "Owls",
        biome: "The Library of Lore",
        days: [ALL, NA, NA, NA, NA, NA, ALL],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "White Owl",
        species: "Owls",
        biome: "The Bind",
        days: [hours(15, 20); 7],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Purple Owl",
        species: "Owls",
        biome: "The Bind",
        days: [hours(0, 9); 7],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Blue Baby Dragon",
        species: "Baby Dragons",
        biome: "The Wild Woods",
        days: [NA, NA, ALL, ALL, ALL, ALL, NA],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Green Baby Dragon",
        species: "Baby Dragons",
        biome: "The Fallen Fortress",
        days: [
            hours(10, 18),
            hours(10, 18),
            hours(10, 18),
            hours(10, 18),
            hours(10, 18),
            NA,
            NA,
        ],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Purple Baby Dragon",
        species: "Baby Dragons",
        biome: "The Beanstalk Marshes",
        days: [AM; 7],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Red Baby Dragon",
        species: "Baby Dragons",
        biome: "Teapot Falls",
        days: [ALL, NA, NA, NA, NA, NA, ALL],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Teal Baby Dragon",
        species: "Baby Dragons",
        biome: "The Wild Woods",
        days: [ALL, NA, NA, ALL, ALL, ALL, ALL],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Black Pegasus",
        species: "Pegasi",
        biome: "The Elysian Fields",
        days: [ALL, NA, NA, ALL, ALL, ALL, ALL],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Blue Pegasus",
        species: "Pegasi",
        biome: "The Fiery Plains",
        days: [NA, ALL, ALL, ALL, NA, NA, NA],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Peach Pegasus",
        species: "Pegasi",
        biome: "Mount Olympus",
        days: [hours(6, 14); 7],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Pink Pegasus",
        species: "Pegasi",
        biome: "The Elysian Fields",
        days: [ALL, NA, NA, NA, ALL, ALL, ALL],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Yellow Pegasus",
        species: "Pegasi",
        biome: "The Statue's Shadow",
        days: [
            NA,
            NA,
            hours(18, 24),
            hours(18, 24),
            hours(18, 24),
            hours(18, 24),
            hours(18, 24),
        ],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Black Goose",
        species: "Geese",
        biome: "Silver Summit",
        days: [hours(5, 11); 7],
        note: Some(Note::WishblossomRanch),
    },
    CritterDef {
        name: "Blue Goose",
        species: "Geese",
        biome: "Wishing Way",
        days: [hours(18, 20); 7],
        note: Some(Note::WishblossomRanch),
    },
    CritterDef {
        name: "Golden Goose",
        species: "Geese",
        biome: "Delver Dale",
        days: [hours(7, 8) | hours(19, 20); 7],
        note: Some(Note::WishblossomRanch),
    },
    CritterDef {
        name: "Goose",
        species: "Geese",
        biome: "Wishblossom Ranch",
        days: [ALL, ALL, NA, ALL, NA, ALL, NA],
        note: Some(Note::WishblossomRanch),
    },
    CritterDef {
        name: "True North Goose",
        species: "Geese",
        biome: "Ranch Highlands",
        days: [NA, NA, ALL, NA, ALL, NA, ALL],
        note: Some(Note::WishblossomRanch),
    },
    CritterDef {
        name: "Brown Skunk",
        species: "Skunks",
        biome: "Paisley Park",
        days: [PM, PM, NA, PM, NA, PM, NA],
        note: Some(Note::WishblossomRanch),
    },
    CritterDef {
        name: "Patterned Skunk",
        species: "Skunks",
        biome: "Haute Plateau",
        days: [ALL; 7],
        note: Some(Note::WishblossomRanchRainyWeather),
    },
    CritterDef {
        name: "Skunk",
        species: "Skunks",
        biome: "Runway River",
        days: [AM; 7],
        note: Some(Note::WishblossomRanch),
    },
    CritterDef {
        name: "White Skunk",
        species: "Skunks",
        biome: "Modish Marsh",
        days: [PM, NA, PM, NA, PM, NA, PM],
        note: Some(Note::WishblossomRanch),
    },
    CritterDef {
        name: "Blue Sweet Bee",
        species: "Sweet Bees",
        biome: "Hunny Falls",
        days: [ALL; 7],
        note: Some(Note::WishblossomRanchClearWeather),
    },
    CritterDef {
        name: "Pink Sweet Bee",
        species: "Sweet Bees",
        biome: "Pixie Flats",
        days: [ALL; 7],
        note: Some(Note::WishblossomRanch),
    },
    CritterDef {
        name: "Sweet Bee",
        species: "Sweet Bees",
        biome: "Sundae Shores",
        days: [hours(7, 20); 7],
        note: Some(Note::WishblossomRanch),
    },
    CritterDef {
        name: "White Sweet Bee",
        species: "Sweet Bees",
        biome: "Hundred-Acre Fields",
        days: [hours(20, 24); 7],
        note: Some(Note::WishblossomRanch),
    },
];

// sort by in game order
fn species_rank(species: &str) -> u32 {
    CRITTER_DATA
        .iter()
        .position(|c| c.species == species)
        .unwrap_or(usize::MAX) as u32
}

#[derive(Clone, Copy, PartialEq, Debug, serde::Serialize, specta::Type)]
pub(crate) struct Schedule {
    start: u8,
    end: u8,
}

fn day_schedule(day: AvailableHours) -> Vec<Schedule> {
    let mut schedule = Vec::new();
    let mut hour: u8 = 0;
    while hour < 24 {
        if day >> hour & 1 == 0 {
            hour += 1;
            continue;
        }
        let start = hour;
        while hour < 24 && day >> hour & 1 == 1 {
            hour += 1;
        }
        schedule.push(Schedule { start, end: hour });
    }
    schedule
}

#[derive(Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Critter {
    pub item_id: u32,
    pub name: String,
    pub species: String,
    pub species_rank: u32,
    pub note: Option<String>,
    pub schedule: Vec<Vec<Schedule>>,
    pub tamed: bool,
    pub reachable: bool,
    pub last_feeding_secs: Option<i64>,
}

pub(crate) fn collect(loaded: &super::LoadedSave) -> Result<Vec<Critter>, super::AppError> {
    let save = &loaded.contents;
    let storefront = &loaded.storefront;

    let name_map = super::game_data::cached_item_names(storefront)?;

    let unlocked_biomes = super::areas::unlocked_biomes(save);

    let name_to_id: HashMap<&str, u32> = name_map
        .iter()
        .filter(|&(id, _)| WILD_CRITTER_ITEM_IDS.contains(id))
        .map(|(&id, name)| (name.as_str(), id))
        .collect();

    let tamed_names: HashSet<&str> = save
        .pointer("/Player/Pets")
        .and_then(|v| v.as_array())
        .map(|pets| {
            pets.iter()
                .filter_map(|pet| pet.get("PetItemID")?.as_u64())
                .filter_map(|id| name_map.get(&(id as u32)))
                .map(String::as_str)
                .collect()
        })
        .unwrap_or_default();

    // recorded as UTC but actually local time
    let last_feeding_by_id: HashMap<u32, i64> = save
        .pointer("/World/Critters")
        .and_then(|v| v.as_array())
        .map(|critters| {
            critters
                .iter()
                .filter_map(|c| {
                    let id = c.get("CritterItemID")?.as_u64()? as u32;
                    let time_str = c.get("LastFeedingTime")?.as_str()?;
                    let ts = chrono::DateTime::parse_from_rfc3339(time_str)
                        .ok()?
                        .timestamp();
                    Some((id, ts))
                })
                .collect()
        })
        .unwrap_or_default();

    let critters = CRITTER_DATA
        .iter()
        .filter_map(|entry| {
            let &wild_id = name_to_id.get(entry.name)?;
            let reachable = unlocked_biomes
                .as_ref()
                .is_none_or(|unlocked| unlocked.contains(entry.biome));
            let schedule: Vec<Vec<Schedule>> =
                entry.days.iter().map(|&day| day_schedule(day)).collect();
            Some(Critter {
                item_id: wild_id,
                name: entry.name.to_string(),
                species: entry.species.to_string(),
                species_rank: species_rank(entry.species),
                note: entry.note.as_ref().map(|n| n.as_str().to_string()),
                schedule,
                tamed: tamed_names.contains(entry.name),
                reachable,
                last_feeding_secs: last_feeding_by_id.get(&wild_id).copied(),
            })
        })
        .collect();

    Ok(critters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_critter_maps_to_a_known_biome() {
        let known = crate::areas::known_biome_names();
        for def in CRITTER_DATA {
            assert!(known.contains(def.biome), "{} -> {}", def.name, def.biome);
        }
    }

    #[test]
    fn mask_decodes_to_day_schedule() {
        assert!(day_schedule(NA).is_empty());
        assert_eq!(day_schedule(ALL), vec![Schedule { start: 0, end: 24 }]);
        assert_eq!(
            day_schedule(hours(7, 8) | hours(19, 20)),
            vec![
                Schedule { start: 7, end: 8 },
                Schedule { start: 19, end: 20 },
            ]
        );
        assert_eq!(
            day_schedule(hours(20, 24)),
            vec![Schedule { start: 20, end: 24 }]
        );
    }
}
