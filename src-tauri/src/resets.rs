use serde_json::Value;

const SECS_PER_MINUTE: i64 = 60;
const SECS_PER_HOUR: i64 = 3600;
const SECS_PER_DAY: i64 = 86_400;

pub(crate) enum ResetSchedule {
    DailyUtc { hour: u32, minute: u32 },
}

pub(crate) const SCROOGE_STORE_RESET: ResetSchedule =
    ResetSchedule::DailyUtc { hour: 8, minute: 0 };

pub(crate) fn latest_reset_secs(
    now_secs: i64,
    _timezone_offset_secs: i64,
    schedule: &ResetSchedule,
) -> i64 {
    match schedule {
        ResetSchedule::DailyUtc { hour, minute } => daily_reset_secs(now_secs, *hour, *minute),
    }
}

pub(crate) fn daily_reset_secs(now_secs: i64, hour: u32, minute: u32) -> i64 {
    let day_midnight_secs = now_secs - now_secs.rem_euclid(SECS_PER_DAY);
    let today_reset_secs =
        day_midnight_secs + hour as i64 * SECS_PER_HOUR + minute as i64 * SECS_PER_MINUTE;
    if now_secs >= today_reset_secs {
        today_reset_secs
    } else {
        today_reset_secs - SECS_PER_DAY
    }
}

pub(crate) fn timestamp_from_text(timestamp_text: &str) -> Option<i64> {
    chrono::DateTime::parse_from_rfc3339(timestamp_text)
        .ok()
        .map(|timestamp| timestamp.timestamp())
}

pub(crate) fn timestamp_from_field(entry: &Value, field: &str) -> Option<i64> {
    timestamp_from_text(entry.get(field)?.as_str()?)
}

pub(crate) fn is_after_reset(refresh_secs: Option<i64>, reset_secs: i64) -> bool {
    refresh_secs.is_some_and(|refresh| refresh >= reset_secs)
}
