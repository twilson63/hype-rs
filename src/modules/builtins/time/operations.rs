use super::error::TimeError;
use chrono::{DateTime, Datelike, Local, TimeZone, Timelike, Utc};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

pub fn now_seconds() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

pub fn now_nanos() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as i64
}

pub fn format_timestamp(timestamp_ms: i64, format: &str) -> Result<String, TimeError> {
    let dt = timestamp_to_datetime(timestamp_ms)?;
    Ok(dt.format(format).to_string())
}

pub fn parse_timestamp(date_str: &str, format: &str) -> Result<i64, TimeError> {
    let dt = DateTime::parse_from_str(date_str, format)
        .map_err(|e| TimeError::ParseError(e.to_string()))?;
    Ok(dt.timestamp_millis())
}

pub fn to_iso(timestamp_ms: i64) -> Result<String, TimeError> {
    let dt = timestamp_to_datetime(timestamp_ms)?;
    Ok(dt.to_rfc3339())
}

pub fn from_iso(iso_str: &str) -> Result<i64, TimeError> {
    let dt =
        DateTime::parse_from_rfc3339(iso_str).map_err(|e| TimeError::ParseError(e.to_string()))?;
    Ok(dt.timestamp_millis())
}

pub struct DateComponents {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub weekday: u32,
}

pub fn date(timestamp_ms: Option<i64>) -> Result<DateComponents, TimeError> {
    let ts = timestamp_ms.unwrap_or_else(now);
    let dt = timestamp_to_datetime(ts)?;

    Ok(DateComponents {
        year: dt.year(),
        month: dt.month(),
        day: dt.day(),
        hour: dt.hour(),
        minute: dt.minute(),
        second: dt.second(),
        weekday: dt.weekday().num_days_from_monday(),
    })
}

pub fn year(timestamp_ms: Option<i64>) -> Result<i32, TimeError> {
    let ts = timestamp_ms.unwrap_or_else(now);
    let dt = timestamp_to_datetime(ts)?;
    Ok(dt.year())
}

pub fn month(timestamp_ms: Option<i64>) -> Result<u32, TimeError> {
    let ts = timestamp_ms.unwrap_or_else(now);
    let dt = timestamp_to_datetime(ts)?;
    Ok(dt.month())
}

pub fn day(timestamp_ms: Option<i64>) -> Result<u32, TimeError> {
    let ts = timestamp_ms.unwrap_or_else(now);
    let dt = timestamp_to_datetime(ts)?;
    Ok(dt.day())
}

pub fn hour(timestamp_ms: Option<i64>) -> Result<u32, TimeError> {
    let ts = timestamp_ms.unwrap_or_else(now);
    let dt = timestamp_to_datetime(ts)?;
    Ok(dt.hour())
}

pub fn minute(timestamp_ms: Option<i64>) -> Result<u32, TimeError> {
    let ts = timestamp_ms.unwrap_or_else(now);
    let dt = timestamp_to_datetime(ts)?;
    Ok(dt.minute())
}

pub fn second(timestamp_ms: Option<i64>) -> Result<u32, TimeError> {
    let ts = timestamp_ms.unwrap_or_else(now);
    let dt = timestamp_to_datetime(ts)?;
    Ok(dt.second())
}

pub fn sleep(ms: u64) {
    std::thread::sleep(Duration::from_millis(ms));
}

pub fn elapsed(start_ms: i64) -> i64 {
    now() - start_ms
}

pub fn format_duration(ms: i64) -> String {
    if ms < 0 {
        return format!("-{}", format_duration(-ms));
    }

    let seconds = ms / 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;

    if days > 0 {
        let h = hours % 24;
        let m = minutes % 60;
        format!("{}d {}h {}m", days, h, m)
    } else if hours > 0 {
        let m = minutes % 60;
        let s = seconds % 60;
        format!("{}h {}m {}s", hours, m, s)
    } else if minutes > 0 {
        let s = seconds % 60;
        format!("{}m {}s", minutes, s)
    } else if seconds > 0 {
        let ms_remainder = ms % 1000;
        format!("{}.{}s", seconds, ms_remainder)
    } else {
        format!("{}ms", ms)
    }
}

fn timestamp_to_datetime(timestamp_ms: i64) -> Result<DateTime<Utc>, TimeError> {
    let seconds = timestamp_ms / 1000;
    let nanos = ((timestamp_ms % 1000) * 1_000_000) as u32;

    Utc.timestamp_opt(seconds, nanos)
        .single()
        .ok_or_else(|| TimeError::InvalidTimestamp(timestamp_ms))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now() {
        let t = now();
        assert!(t > 0);
    }

    #[test]
    fn test_now_seconds() {
        let t = now_seconds();
        assert!(t > 0);
        assert!(t < now());
    }

    #[test]
    fn test_now_nanos() {
        let t = now_nanos();
        assert!(t > 0);
    }

    #[test]
    fn test_to_iso_and_from_iso() {
        let timestamp = 1609459200000i64;
        let iso = to_iso(timestamp).unwrap();
        assert!(iso.contains("2021"));
        let parsed = from_iso(&iso).unwrap();
        assert_eq!(parsed, timestamp);
    }

    #[test]
    fn test_date_components() {
        let timestamp = 1609459200000i64;
        let components = date(Some(timestamp)).unwrap();
        assert_eq!(components.year, 2021);
        assert_eq!(components.month, 1);
        assert_eq!(components.day, 1);
    }

    #[test]
    fn test_year() {
        let timestamp = 1609459200000i64;
        assert_eq!(year(Some(timestamp)).unwrap(), 2021);
    }

    #[test]
    fn test_month() {
        let timestamp = 1609459200000i64;
        assert_eq!(month(Some(timestamp)).unwrap(), 1);
    }

    #[test]
    fn test_day() {
        let timestamp = 1609459200000i64;
        assert_eq!(day(Some(timestamp)).unwrap(), 1);
    }

    #[test]
    fn test_hour() {
        let timestamp = 1609459200000i64;
        let h = hour(Some(timestamp)).unwrap();
        assert!(h < 24);
    }

    #[test]
    fn test_minute() {
        let timestamp = 1609459200000i64;
        let m = minute(Some(timestamp)).unwrap();
        assert!(m < 60);
    }

    #[test]
    fn test_second() {
        let timestamp = 1609459200000i64;
        let s = second(Some(timestamp)).unwrap();
        assert!(s < 60);
    }

    #[test]
    fn test_elapsed() {
        let start = now();
        std::thread::sleep(Duration::from_millis(10));
        let diff = elapsed(start);
        assert!(diff >= 10);
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(500), "500ms");
        assert_eq!(format_duration(1500), "1.500s");
        assert_eq!(format_duration(65000), "1m 5s");
        assert_eq!(format_duration(3665000), "1h 1m 5s");
        assert_eq!(format_duration(90065000), "1d 1h 1m");
    }

    #[test]
    fn test_format_timestamp() {
        let timestamp = 1609459200000i64;
        let formatted = format_timestamp(timestamp, "%Y-%m-%d").unwrap();
        assert_eq!(formatted, "2021-01-01");
    }
}
