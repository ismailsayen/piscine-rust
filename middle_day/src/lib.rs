use chrono::prelude::*;
pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    let leep_year = NaiveDate::from_ymd_opt(year as i32, 2, 29).is_some();
    if leep_year {
        return None;
    }
    NaiveDate::from_yo_opt(year as i32, 365 / 2 + 1).map(|d| d.weekday())
}
