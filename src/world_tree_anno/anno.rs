use chrono::Local;

use crate::*;

#[derive(Debug)]
pub struct Anno {
    pub year: Year,
    pub month: Month,
    pub day: Day,
    pub hms: Hms,
    pub chord: Chord,
}

impl Anno {
    pub fn from_number(number: u64) -> Self {
        let (raw_day, hms) = Hms::from_raw_number(number);
        let (raw_month, day) = Day::from_raw_number(raw_day);
        let (raw_year, month) = Month::from_raw_number(raw_month);
        let year = Year::from_number(raw_year);
        let chord = Chord::from_raw_number(raw_day);

        Anno {
            year,
            month,
            day,
            hms,
            chord,
        }
    }

    pub fn from_timestamp(timestamp: u64) -> Self {
        Self::from_number(timestamp)
    }

    pub fn get_anno() -> Self {
        let unix = Local::now();
        let wta_unix: u64 = 72 * (unix.timestamp() as u64 - *KITTEN_TIME)
            + (unix.timestamp_micros() % 1000000 * 72 / 1000000) as u64;
        Self::from_timestamp(wta_unix)
    }
}

impl ToString for Anno {
    fn to_string(&self) -> String {
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
}

impl Default for Anno {
    fn default() -> Self {
        Self::from_timestamp(0)
    }
}
