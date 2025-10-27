use super::operations::*;
use mlua::{Lua, Result as LuaResult, Table, Value};

pub fn create_time_module(lua: &Lua) -> LuaResult<Table> {
    let time = lua.create_table()?;

    let now_fn = lua.create_function(|_, ()| Ok(now()))?;
    time.set("now", now_fn)?;

    let now_seconds_fn = lua.create_function(|_, ()| Ok(now_seconds()))?;
    time.set("nowSeconds", now_seconds_fn)?;

    let now_nanos_fn = lua.create_function(|_, ()| Ok(now_nanos()))?;
    time.set("nowNanos", now_nanos_fn)?;

    let format_fn = lua.create_function(|_, (timestamp, format): (i64, String)| {
        match format_timestamp(timestamp, &format) {
            Ok(s) => Ok(s),
            Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
        }
    })?;
    time.set("format", format_fn)?;

    let parse_fn = lua.create_function(|_, (date_str, format): (String, String)| {
        match parse_timestamp(&date_str, &format) {
            Ok(ts) => Ok(ts),
            Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
        }
    })?;
    time.set("parse", parse_fn)?;

    let to_iso_fn = lua.create_function(|_, timestamp: i64| match to_iso(timestamp) {
        Ok(s) => Ok(s),
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    time.set("toISO", to_iso_fn)?;

    let from_iso_fn = lua.create_function(|_, iso_str: String| match from_iso(&iso_str) {
        Ok(ts) => Ok(ts),
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    time.set("fromISO", from_iso_fn)?;

    let date_fn = lua.create_function(|lua, timestamp: Option<i64>| match date(timestamp) {
        Ok(components) => {
            let table = lua.create_table()?;
            table.set("year", components.year)?;
            table.set("month", components.month)?;
            table.set("day", components.day)?;
            table.set("hour", components.hour)?;
            table.set("minute", components.minute)?;
            table.set("second", components.second)?;
            table.set("weekday", components.weekday)?;
            Ok(table)
        }
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    time.set("date", date_fn)?;

    let year_fn = lua.create_function(|_, timestamp: Option<i64>| match year(timestamp) {
        Ok(y) => Ok(y),
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    time.set("year", year_fn)?;

    let month_fn = lua.create_function(|_, timestamp: Option<i64>| match month(timestamp) {
        Ok(m) => Ok(m),
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    time.set("month", month_fn)?;

    let day_fn = lua.create_function(|_, timestamp: Option<i64>| match day(timestamp) {
        Ok(d) => Ok(d),
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    time.set("day", day_fn)?;

    let hour_fn = lua.create_function(|_, timestamp: Option<i64>| match hour(timestamp) {
        Ok(h) => Ok(h),
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    time.set("hour", hour_fn)?;

    let minute_fn = lua.create_function(|_, timestamp: Option<i64>| match minute(timestamp) {
        Ok(m) => Ok(m),
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    time.set("minute", minute_fn)?;

    let second_fn = lua.create_function(|_, timestamp: Option<i64>| match second(timestamp) {
        Ok(s) => Ok(s),
        Err(e) => Err(mlua::Error::RuntimeError(e.to_string())),
    })?;
    time.set("second", second_fn)?;

    let sleep_fn = lua.create_function(|_, ms: u64| {
        sleep(ms);
        Ok(())
    })?;
    time.set("sleep", sleep_fn)?;

    let elapsed_fn = lua.create_function(|_, start: i64| Ok(elapsed(start)))?;
    time.set("elapsed", elapsed_fn)?;

    let duration_fn = lua.create_function(|_, ms: i64| Ok(format_duration(ms)))?;
    time.set("duration", duration_fn)?;

    Ok(time)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::Lua;

    #[test]
    fn test_create_time_module() {
        let lua = Lua::new();
        let result = create_time_module(&lua);
        assert!(result.is_ok());

        let time = result.unwrap();
        assert!(time.contains_key("now").unwrap());
        assert!(time.contains_key("nowSeconds").unwrap());
        assert!(time.contains_key("nowNanos").unwrap());
        assert!(time.contains_key("format").unwrap());
        assert!(time.contains_key("parse").unwrap());
        assert!(time.contains_key("toISO").unwrap());
        assert!(time.contains_key("fromISO").unwrap());
        assert!(time.contains_key("date").unwrap());
        assert!(time.contains_key("year").unwrap());
        assert!(time.contains_key("month").unwrap());
        assert!(time.contains_key("day").unwrap());
        assert!(time.contains_key("hour").unwrap());
        assert!(time.contains_key("minute").unwrap());
        assert!(time.contains_key("second").unwrap());
        assert!(time.contains_key("sleep").unwrap());
        assert!(time.contains_key("elapsed").unwrap());
        assert!(time.contains_key("duration").unwrap());
    }

    #[test]
    fn test_time_now() {
        let lua = Lua::new();
        let time = create_time_module(&lua).unwrap();
        lua.globals().set("time", time).unwrap();

        let result: i64 = lua.load("return time.now()").eval().unwrap();
        assert!(result > 0);
    }

    #[test]
    fn test_time_to_iso() {
        let lua = Lua::new();
        let time = create_time_module(&lua).unwrap();
        lua.globals().set("time", time).unwrap();

        let result: String = lua.load("return time.toISO(1609459200000)").eval().unwrap();
        assert!(result.contains("2021"));
    }

    #[test]
    fn test_time_date() {
        let lua = Lua::new();
        let time = create_time_module(&lua).unwrap();
        lua.globals().set("time", time).unwrap();

        lua.load(
            r#"
local d = time.date(1609459200000)
assert(d.year == 2021)
assert(d.month == 1)
assert(d.day == 1)
"#,
        )
        .exec()
        .unwrap();
    }
}
