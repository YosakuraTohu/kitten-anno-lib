#[cfg(feature = "wasmbind")]
use wasm_bindgen::prelude::*;

use crate::*;
#[cfg_attr(feature = "wasmbind", wasm_bindgen(getter_with_clone))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Day {
    pub number: u8,
    #[cfg(feature = "std")]
    pub str: String,
}

impl FromNumber for Day {
    #[inline(always)]
    fn from_number(number: u8) -> Self {
        Self {
            number,
            #[cfg(feature = "std")]
            str: day_str(number),
        }
    }
}

impl FromRawNumber for Day {
    #[inline(always)]
    fn from_raw_number(number: u64) -> (u64, Self) {
        let month_cycle_count = number / MONTH_CYCLE_DAY_COUNT as u64;
        let net_day = (number % MONTH_CYCLE_DAY_COUNT as u64) as u8;

        let mut current_cycle_month_count = 0;
        for i in MONTH_CYCLE_FIRSTDAY_DAY.iter() {
            if net_day < *i {
                break;
            }
            current_cycle_month_count += 1;
        }

        let month = month_cycle_count * MONTH_CYCLE as u64 + current_cycle_month_count - 1;
        let day = net_day - MONTH_CYCLE_FIRSTDAY_DAY[current_cycle_month_count as usize - 1] + 1;

        (month, Self::from_number(day))
    }
}

impl Reverse for Day {
    #[inline(always)]
    fn reverse(number: u64) -> u64 {
        let number = number - 1;

        number * SECONDS_PER_DAY as u64
    }
}

impl Default for Day {
    #[inline(always)]
    fn default() -> Self {
        Self::from_number(1)
    }
}
