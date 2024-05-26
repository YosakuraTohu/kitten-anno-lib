use chrono::Local;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

use crate::*;

#[cfg_attr(target_family = "wasm", wasm_bindgen(getter_with_clone))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Anno {
    pub timestamp: u64,
    pub year: Year,
    pub month: Month,
    pub day: Day,
    pub hms: Hms,
    pub chord: Chord,
    pub is_common: IsCommonSt,
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
impl Anno {
    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    #[inline]
    pub fn from_number(number: u64) -> Self {
        let (raw_day, hms) = Hms::from_raw_number(number);
        let (raw_month, day) = Day::from_raw_number(raw_day);
        let (raw_year, month) = Month::from_raw_number(raw_month);
        let year = Year::from_number(raw_year);
        let chord = Chord::from_raw_number(raw_day);
        let is_common = IsCommonSt::from_raw_number(raw_year, raw_month);

        Self {
            timestamp: number,
            year,
            month,
            day,
            hms,
            chord,
            is_common,
        }
    }

    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    #[inline]
    pub fn from_timestamp(timestamp: u64) -> Self {
        Self::from_number(timestamp)
    }

    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    #[inline(always)]
    pub fn from_day(raw_day: u64) -> Self {
        let timestamp = Day::reverse(raw_day);

        Self::from_timestamp(timestamp)
    }

    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    #[inline(always)]
    pub fn from_month(raw_month: u64) -> Self {
        let raw_day = Month::reverse(raw_month);

        Self::from_day(raw_day)
    }

    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    #[inline(always)]
    pub fn from_year(raw_year: u64) -> Self {
        let raw_month = Year::reverse(raw_year);

        Self::from_month(raw_month)
    }

    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    #[inline]
    pub fn from_time(
        raw_year: u64,
        raw_month: u64,
        raw_day: u64,
        raw_hour: u64,
        raw_minute: u64,
        raw_second: u64,
    ) -> Self {
        let raw_month = Year::reverse(raw_year) + raw_month;
        let raw_day = Month::reverse(raw_month - 1) + raw_day;
        let timestamp = Day::reverse(raw_day - 1) + raw_hour * 3600 + raw_minute * 60 + raw_second;

        Self::from_timestamp(timestamp)
    }

    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    #[inline(always)]
    pub fn get_anno() -> Self {
        let unix = Local::now();
        let wta_unix: u64 = 72 * (unix.timestamp() as u64 - *KITTEN_TIME)
            + (unix.timestamp_micros() % 1000000 * 72 / 1000000) as u64;
        Self::from_timestamp(wta_unix)
    }

    #[cfg(target_family = "wasm")]
    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}\u{3000}{}\u{3000}{}",
            self.year.str, self.month.str, self.day.str, self.hms.str, self.chord.str,
        )
    }

    #[cfg(target_family = "wasm")]
    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    pub fn default() -> Self {
        Self::from_timestamp(0)
    }
}

#[cfg(not(target_family = "wasm"))]
impl ToString for Anno {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}\u{3000}{}\u{3000}{}",
            self.year.str, self.month.str, self.day.str, self.hms.str, self.chord.str,
        )
    }
}

#[cfg(not(target_family = "wasm"))]
impl Default for Anno {
    fn default() -> Self {
        Self::from_timestamp(0)
    }
}
