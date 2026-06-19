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
        days: [PM, NA, ALL, ALL, NA, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Golden Sunbird",
        species: "Sunbirds",
        days: [AM, NA, ALL, NA, ALL, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Orchid Sunbird",
        species: "Sunbirds",
        days: [NA, NA, NA, NA, NA, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Red Sunbird",
        species: "Sunbirds",
        days: [PM, ALL, NA, NA, ALL, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Turquoise Sunbird",
        species: "Sunbirds",
        days: [AM, ALL, NA, ALL, NA, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Black Squirrel",
        species: "Squirrels",
        days: [AM, NA, ALL, NA, ALL, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Classic Squirrel",
        species: "Squirrels",
        days: [PM, ALL, NA, NA, ALL, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Gray Squirrel",
        species: "Squirrels",
        days: [PM, NA, ALL, ALL, NA, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Red Squirrel",
        species: "Squirrels",
        days: [AM, ALL, NA, ALL, NA, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "White Squirrel",
        species: "Squirrels",
        days: [ALL, NA, NA, NA, NA, NA, NA],
        note: None,
    },
    CritterDef {
        name: "Blue Crocodile",
        species: "Crocodiles",
        days: [AM, ALL, NA, ALL, NA, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Classic Crocodile",
        species: "Crocodiles",
        days: [PM, ALL, NA, NA, ALL, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Golden Crocodile",
        species: "Crocodiles",
        days: [AM, NA, ALL, NA, ALL, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Pink Crocodile",
        species: "Crocodiles",
        days: [NA, NA, NA, NA, NA, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Red Crocodile",
        species: "Crocodiles",
        days: [PM, NA, ALL, ALL, NA, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "White Crocodile",
        species: "Crocodiles",
        days: [ALL, NA, NA, NA, NA, NA, NA],
        note: None,
    },
    CritterDef {
        name: "Black Rabbit",
        species: "Rabbits",
        days: [AM, ALL, NA, ALL, NA, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Brown Rabbit",
        species: "Rabbits",
        days: [PM, NA, ALL, ALL, NA, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Calico Rabbit",
        species: "Rabbits",
        days: [NA, NA, NA, NA, ALL, NA, NA],
        note: None,
    },
    CritterDef {
        name: "Classic Rabbit",
        species: "Rabbits",
        days: [PM, ALL, NA, NA, ALL, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "White Rabbit",
        species: "Rabbits",
        days: [AM, NA, ALL, NA, ALL, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Blue Raven",
        species: "Ravens",
        days: [AM, NA, ALL, NA, ALL, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Brown Raven",
        species: "Ravens",
        days: [NA, NA, ALL, NA, NA, NA, NA],
        note: None,
    },
    CritterDef {
        name: "Classic Raven",
        species: "Ravens",
        days: [PM, ALL, NA, NA, ALL, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Red Raven",
        species: "Ravens",
        days: [AM, ALL, NA, ALL, NA, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "White Raven",
        species: "Ravens",
        days: [PM, NA, ALL, ALL, NA, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Black Sea Turtle",
        species: "Sea Turtles",
        days: [NA, ALL, NA, NA, NA, NA, NA],
        note: None,
    },
    CritterDef {
        name: "Brown Sea Turtle",
        species: "Sea Turtles",
        days: [AM, ALL, NA, ALL, NA, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Classic Sea Turtle",
        species: "Sea Turtles",
        days: [PM, ALL, NA, NA, ALL, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Purple Sea Turtle",
        species: "Sea Turtles",
        days: [AM, NA, ALL, NA, ALL, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "White Sea Turtle",
        species: "Sea Turtles",
        days: [PM, NA, ALL, ALL, NA, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Black Fox",
        species: "Foxes",
        days: [PM, NA, ALL, ALL, NA, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Blue Fox",
        species: "Foxes",
        days: [AM, NA, ALL, NA, ALL, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Classic Fox",
        species: "Foxes",
        days: [AM, ALL, NA, ALL, NA, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Red Fox",
        species: "Foxes",
        days: [NA, NA, NA, NA, NA, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "White Fox",
        species: "Foxes",
        days: [PM, ALL, NA, NA, ALL, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Black Raccoon",
        species: "Raccoons",
        days: [PM, NA, ALL, ALL, NA, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Blue Raccoon",
        species: "Raccoons",
        days: [NA, NA, NA, ALL, NA, NA, NA],
        note: None,
    },
    CritterDef {
        name: "Classic Raccoon",
        species: "Raccoons",
        days: [PM, ALL, NA, NA, ALL, NA, ALL],
        note: None,
    },
    CritterDef {
        name: "Red Raccoon",
        species: "Raccoons",
        days: [AM, ALL, NA, ALL, NA, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "White Raccoon",
        species: "Raccoons",
        days: [AM, NA, ALL, NA, ALL, ALL, NA],
        note: None,
    },
    CritterDef {
        name: "Black and White Capybara",
        species: "Capybaras",
        days: [PM, NA, ALL, NA, ALL, PM, PM],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Blue Striped Capybara",
        species: "Capybaras",
        days: [NA, AM, AM, AM, ALL, NA, ALL],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Classic Capybara",
        species: "Capybaras",
        days: [ALL; 7],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Gray Spotted Capybara",
        species: "Capybaras",
        days: [ALL, PM, PM, PM, NA, ALL, NA],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Red and White Striped Capybara",
        species: "Capybaras",
        days: [NA, NA, NA, NA, NA, NA, ALL],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Toon Capybara",
        species: "Capybaras",
        days: [ALL; 7],
        note: Some(Note::SparkOfImagination),
    },
    CritterDef {
        name: "Blue and Red Striped Cobra",
        species: "Cobras",
        days: [ALL, PM, PM, NA, ALL, NA, PM],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Classic Cobra",
        species: "Cobras",
        days: [ALL; 7],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Green and White Striped Cobra",
        species: "Cobras",
        days: [NA, NA, NA, NA, ALL, NA, NA],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Pink Spotted Cobra",
        species: "Cobras",
        days: [AM, ALL, NA, AM, AM, ALL, NA],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Toon Cobra",
        species: "Cobras",
        days: [ALL; 7],
        note: Some(Note::SparkOfImagination),
    },
    CritterDef {
        name: "Yellow and Purple Striped Cobra",
        species: "Cobras",
        days: [PM, NA, ALL, PM, PM, NA, ALL],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Beige Monkey",
        species: "Monkeys",
        days: [ALL, PM, NA, PM, ALL, PM, NA],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Black and Brown Monkey",
        species: "Monkeys",
        days: [PM, NA, PM, ALL, PM, NA, ALL],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Black and Gray Monkey",
        species: "Monkeys",
        days: [NA, NA, ALL, NA, NA, NA, NA],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Classic Monkey",
        species: "Monkeys",
        days: [ALL; 7],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Red and Beige Monkey",
        species: "Monkeys",
        days: [NA, AM, ALL, AM, NA, AM, ALL],
        note: Some(Note::AriftInTime),
    },
    CritterDef {
        name: "Toon Monkey",
        species: "Monkeys",
        days: [ALL; 7],
        note: Some(Note::SparkOfImagination),
    },
    CritterDef {
        name: "Brown Owl",
        species: "Owls",
        days: [NA, ALL, ALL, ALL, ALL, ALL, NA],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Dark Owl",
        species: "Owls",
        days: [ALL, NA, NA, NA, NA, NA, ALL],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "White Owl",
        species: "Owls",
        days: [hours(15, 20); 7],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Purple Owl",
        species: "Owls",
        days: [hours(0, 9); 7],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Blue Baby Dragon",
        species: "Baby Dragons",
        days: [NA, NA, ALL, ALL, ALL, ALL, NA],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Green Baby Dragon",
        species: "Baby Dragons",
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
        days: [AM; 7],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Red Baby Dragon",
        species: "Baby Dragons",
        days: [ALL, NA, NA, NA, NA, NA, ALL],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Teal Baby Dragon",
        species: "Baby Dragons",
        days: [ALL, NA, NA, ALL, ALL, ALL, ALL],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Black Pegasus",
        species: "Pegasi",
        days: [ALL, NA, NA, ALL, ALL, ALL, ALL],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Blue Pegasus",
        species: "Pegasi",
        days: [NA, ALL, ALL, ALL, NA, NA, NA],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Peach Pegasus",
        species: "Pegasi",
        days: [hours(6, 14); 7],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Pink Pegasus",
        species: "Pegasi",
        days: [ALL, NA, NA, NA, ALL, ALL, ALL],
        note: Some(Note::StorybookVale),
    },
    CritterDef {
        name: "Yellow Pegasus",
        species: "Pegasi",
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
        days: [hours(5, 11); 7],
        note: Some(Note::WishblossomRanch),
    },
    CritterDef {
        name: "Blue Goose",
        species: "Geese",
        days: [hours(18, 20); 7],
        note: Some(Note::WishblossomRanch),
    },
    CritterDef {
        name: "Golden Goose",
        species: "Geese",
        days: [hours(7, 8) | hours(19, 20); 7],
        note: Some(Note::WishblossomRanch),
    },
    CritterDef {
        name: "Goose",
        species: "Geese",
        days: [ALL, ALL, NA, ALL, NA, ALL, NA],
        note: Some(Note::WishblossomRanch),
    },
    CritterDef {
        name: "True North Goose",
        species: "Geese",
        days: [NA, NA, ALL, NA, ALL, NA, ALL],
        note: Some(Note::WishblossomRanch),
    },
    CritterDef {
        name: "Brown Skunk",
        species: "Skunks",
        days: [PM, PM, NA, PM, NA, PM, NA],
        note: Some(Note::WishblossomRanch),
    },
    CritterDef {
        name: "Patterned Skunk",
        species: "Skunks",
        days: [ALL; 7],
        note: Some(Note::WishblossomRanchRainyWeather),
    },
    CritterDef {
        name: "Skunk",
        species: "Skunks",
        days: [AM; 7],
        note: Some(Note::WishblossomRanch),
    },
    CritterDef {
        name: "White Skunk",
        species: "Skunks",
        days: [PM, NA, PM, NA, PM, NA, PM],
        note: Some(Note::WishblossomRanch),
    },
    CritterDef {
        name: "Blue Sweet Bee",
        species: "Sweet Bees",
        days: [ALL; 7],
        note: Some(Note::WishblossomRanchClearWeather),
    },
    CritterDef {
        name: "Pink Sweet Bee",
        species: "Sweet Bees",
        days: [ALL; 7],
        note: Some(Note::WishblossomRanch),
    },
    CritterDef {
        name: "Sweet Bee",
        species: "Sweet Bees",
        days: [hours(7, 20); 7],
        note: Some(Note::WishblossomRanch),
    },
    CritterDef {
        name: "White Sweet Bee",
        species: "Sweet Bees",
        days: [hours(20, 24); 7],
        note: Some(Note::WishblossomRanch),
    },
];

// position in CRITTER_DATA is the canonical species order, used as a sort tiebreaker
fn species_rank(species: &str) -> u32 {
    CRITTER_DATA
        .iter()
        .position(|c| c.species == species)
        .unwrap_or(usize::MAX) as u32
}

// 0 to 6 - starts sunday
fn weekday_index(local_secs: i64) -> usize {
    (local_secs.div_euclid(86400) + 4).rem_euclid(7) as usize
}

fn is_available_at(days: &[AvailableHours; 7], local_secs: i64) -> bool {
    let hour = (local_secs % 86400) / 3600;
    days[weekday_index(local_secs)] >> hour & 1 == 1
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

#[derive(serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Critter {
    pub item_id: u32,
    pub name: String,
    pub species: String,
    pub species_rank: u32,
    pub note: Option<String>,
    pub schedule: Vec<Vec<Schedule>>,
    pub available_now: bool,
    pub tamed: bool,
    pub fed_today: bool,
    pub needs_feeding: bool,
}

#[tauri::command]
#[specta::specta]
pub fn get_critters(
    state: tauri::State<super::AppState>,
    now_utc_secs: i64,
) -> Result<Vec<Critter>, super::AppError> {
    let guard = state.save.lock().unwrap();
    let loaded = guard.as_ref().ok_or(super::AppError::NoSaveLoaded)?;
    collect(loaded, now_utc_secs)
}

pub(crate) fn collect(
    loaded: &super::LoadedSave,
    now_utc_secs: i64,
) -> Result<Vec<Critter>, super::AppError> {
    let save = &loaded.contents;
    let storefront = &loaded.storefront;

    let tz_offset = save
        .pointer("/World/TimeZoneOffset")
        .and_then(|v| {
            v.as_i64()
                .or_else(|| v.as_str()?.trim_end_matches('s').parse::<i64>().ok())
        })
        .unwrap_or(0);

    let local_now = now_utc_secs + tz_offset;
    let local_today_secs = local_now % 86400;
    let local_midnight = local_now - local_today_secs;

    let name_map = super::game_data::cached_item_names(storefront)?;

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

    let fed_today_ids: HashSet<u32> = save
        .pointer("/World/Critters")
        .and_then(|v| v.as_array())
        .map(|critters| {
            critters
                .iter()
                .filter_map(|c| {
                    let id = c.get("CritterItemID")?.as_u64()? as u32;
                    // last feeding time is local time, ignore the Z
                    let time_str = c.get("LastFeedingTime")?.as_str()?;
                    let ts = chrono::DateTime::parse_from_rfc3339(time_str)
                        .map(|dt| dt.timestamp())
                        .ok()?;
                    if ts >= local_midnight { Some(id) } else { None }
                })
                .collect()
        })
        .unwrap_or_default();

    let critters = CRITTER_DATA
        .iter()
        .filter_map(|entry| {
            let &wild_id = name_to_id.get(entry.name)?;
            let available_now = is_available_at(&entry.days, local_now);
            let tamed = tamed_names.contains(entry.name);
            let fed_today = fed_today_ids.contains(&wild_id);
            let needs_feeding = available_now && !tamed && !fed_today;
            let schedule: Vec<Vec<Schedule>> =
                entry.days.iter().map(|&day| day_schedule(day)).collect();
            Some(Critter {
                item_id: wild_id,
                name: entry.name.to_string(),
                species: entry.species.to_string(),
                species_rank: species_rank(entry.species),
                note: entry.note.as_ref().map(|n| n.as_str().to_string()),
                schedule,
                available_now,
                tamed,
                fed_today,
                needs_feeding,
            })
        })
        .collect();

    Ok(critters)
}

#[cfg(test)]
mod tests {
    use super::*;

    // seconds to midnight
    const SUN: i64 = 259200;
    const MON: i64 = 345600;
    const THU: i64 = 0;
    const SAT: i64 = 172800;

    fn at(base: i64, hour: u8) -> i64 {
        base + hour as i64 * 3600
    }

    #[test]
    fn na_day_is_always_unavailable() {
        let entry = &CRITTER_DATA[9];
        assert_eq!(entry.name, "White Squirrel");
        assert!(!is_available_at(&entry.days, at(MON, 12)));
        assert!(!is_available_at(&entry.days, at(THU, 0)));
        assert!(!is_available_at(&entry.days, at(SAT, 23)));
    }

    #[test]
    fn all_day_window_covers_full_day() {
        let entry = &CRITTER_DATA[9];
        assert!(is_available_at(&entry.days, at(SUN, 0)));
        assert!(is_available_at(&entry.days, at(SUN, 12)));
        assert!(is_available_at(&entry.days, at(SUN, 23)));
    }

    #[test]
    fn am_window_matches_midnight_to_noon() {
        let entry = &CRITTER_DATA[5];
        assert_eq!(entry.name, "Black Squirrel");
        assert!(is_available_at(&entry.days, at(SUN, 0)));
        assert!(is_available_at(&entry.days, at(SUN, 11)));
        assert!(!is_available_at(&entry.days, at(SUN, 12)));
        assert!(!is_available_at(&entry.days, at(SUN, 23)));
    }

    #[test]
    fn pm_window_matches_noon_to_midnight() {
        let entry = &CRITTER_DATA[6]; // Classic Squirrel: PM on Sunday
        assert_eq!(entry.name, "Classic Squirrel");
        assert!(!is_available_at(&entry.days, at(SUN, 0)));
        assert!(!is_available_at(&entry.days, at(SUN, 11)));
        assert!(is_available_at(&entry.days, at(SUN, 12)));
        assert!(is_available_at(&entry.days, at(SUN, 23)));
    }

    #[test]
    fn custom_window_boundaries() {
        let owl = CRITTER_DATA.iter().find(|e| e.name == "White Owl").unwrap();
        assert!(!is_available_at(&owl.days, at(SUN, 14)));
        assert!(is_available_at(&owl.days, at(SUN, 15)));
        assert!(is_available_at(&owl.days, at(SUN, 19)));
        assert!(!is_available_at(&owl.days, at(SUN, 20)));
    }

    #[test]
    fn dual_window_golden_goose() {
        let goose = CRITTER_DATA
            .iter()
            .find(|e| e.name == "Golden Goose")
            .unwrap();
        assert!(is_available_at(&goose.days, at(SUN, 7)));
        assert!(!is_available_at(&goose.days, at(SUN, 8)));
        assert!(is_available_at(&goose.days, at(SUN, 19)));
        assert!(!is_available_at(&goose.days, at(SUN, 20)));
        assert!(!is_available_at(&goose.days, at(SUN, 12)));
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

    #[test]
    fn day_of_week_gating() {
        let entry = CRITTER_DATA
            .iter()
            .find(|e| e.name == "Brown Raven")
            .unwrap();
        assert!(!is_available_at(&entry.days, at(SUN, 12)));
        assert!(!is_available_at(&entry.days, at(MON, 12)));
        assert!(is_available_at(&entry.days, at(MON + 86400, 12)));
    }
}
