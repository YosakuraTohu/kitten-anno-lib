use crate::*;

#[derive(Debug)]
pub struct Hms {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Hms {
    pub fn from_number(number: u32) -> Self {
        Self {
            hour: (number / 3600) as u8,
            minute: ((number % 3600) / 60) as u8,
            second: ((number % 3600) % 60) as u8,
        }
    }
}

impl FromRawNumber for Hms {
    fn from_raw_number(number: u64) -> (u64, Self) {
        (
            number / SECONDS_PER_DAY as u64,
            Self::from_number((number % SECONDS_PER_DAY as u64) as u32),
        )
    }
}

impl Default for Hms {
    fn default() -> Self {
        Self::from_number(0)
    }
}