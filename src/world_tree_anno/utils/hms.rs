#[cfg(feature = "wasmbind")]
use wasm_bindgen::prelude::*;

use crate::*;

#[cfg_attr(feature = "wasmbind", wasm_bindgen(getter_with_clone))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hms {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    #[cfg(feature = "std")]
    pub str: String,
}

impl Hms {
    #[inline(always)]
    pub(crate) fn from_number(number: u32) -> Self {
        let hour = (number / 3600) as u8;
        let minute = ((number % 3600) / 60) as u8;
        let second = ((number % 3600) % 60) as u8;
        Self {
            hour,
            minute,
            second,
            #[cfg(feature = "std")]
            str: hms_str(hour, minute, second),
        }
    }
}

impl FromRawNumber for Hms {
    #[inline(always)]
    fn from_raw_number(number: u64) -> (u64, Self) {
        (
            number / SECONDS_PER_DAY as u64,
            Self::from_number((number % SECONDS_PER_DAY as u64) as u32),
        )
    }
}

impl Default for Hms {
    #[inline(always)]
    fn default() -> Self {
        Self::from_number(0)
    }
}
