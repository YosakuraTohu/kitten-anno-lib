use chrono::Local;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

use crate::*;

#[cfg_attr(target_family = "wasm", wasm_bindgen(getter_with_clone))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Anno {
    pub year: Year,
    pub month: Month,
    pub day: Day,
    pub hms: Hms,
    pub chord: Chord,
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
impl Anno {
    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    pub fn from_number(number: u64) -> Self {
        let (raw_day, hms) = Hms::from_raw_number(number);
        let (raw_month, day) = Day::from_raw_number(raw_day);
        let (raw_year, month) = Month::from_raw_number(raw_month);
        let year = Year::from_number(raw_year);
        let chord = Chord::from_raw_number(raw_day);

        Self {
            year,
            month,
            day,
            hms,
            chord,
        }
    }

    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    pub fn from_timestamp(timestamp: u64) -> Self {
        Self::from_number(timestamp)
    }

    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    pub fn from_day(raw_day: u64) -> Self {
        let timestamp = Day::reverse(raw_day);

        Self::from_timestamp(timestamp)
    }

    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    pub fn from_month(raw_month: u64) -> Self {
        let raw_day = Month::reverse(raw_month);

        Self::from_day(raw_day)
    }

    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    pub fn from_year(raw_year: u64) -> Self {
        let raw_month = Year::reverse(raw_year);

        Self::from_month(raw_month)
    }

    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    pub fn get_anno() -> Self {
        let unix = Local::now();
        let wta_unix: u64 = 72 * (unix.timestamp() as u64 - *KITTEN_TIME)
            + (unix.timestamp_micros() % 1000000 * 72 / 1000000) as u64;
        Self::from_timestamp(wta_unix)
    }

    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    pub fn to_string(&self) -> String {
        let fill_in = |number: u8| -> String {
            if number < 10 {
                return format!("0{}", number);
            }
            format!("{}", number)
        };
        format!(
            "{}{}{} {}:{}:{}",
            self.year.str,
            self.month.str,
            self.day.str,
            self.hms.hour,
            fill_in(self.hms.minute),
            fill_in(self.hms.second)
        )
    }

    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    pub fn default() -> Self {
        Self::from_timestamp(0)
    }
}
