#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

use crate::*;

#[cfg_attr(target_family = "wasm", wasm_bindgen(getter_with_clone))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Month {
    pub number: u8,
    pub str: String,
    pub flower: String,
    pub elemental: String,
    pub imagery: String,
}

impl IsCommon for Month {
    fn is_common(number: u64) -> bool {
        match (number % MONTH_CYCLE as u64) as u8 {
            1 | 4 | 8 => false,
            _ => true,
        }
    }
}

impl FromNumber for Month {
    fn from_number(number: u8) -> Self {
        let [month_str, elemental, imagery, flower] = MEANING_OF_MONTH[number as usize];
        Self {
            number,
            str: month_str.to_string(),
            flower: flower.to_string(),
            elemental: elemental.to_string(),
            imagery: imagery.to_string(),
        }
    }
}

impl FromRawNumber for Month {
    fn from_raw_number(number: u64) -> (u64, Self) {
        let year_cycle_count = number / YEAR_CYCLE_MONTH_COUNT as u64;
        let net_month = (number % YEAR_CYCLE_MONTH_COUNT as u64) as u16;

        let mut current_cycle_year_count = 0;
        for i in YEAR_CYCLE_FIRSTMONTH_MONTH.iter() {
            if net_month < *i {
                break;
            }
            current_cycle_year_count += 1;
        }

        let year = year_cycle_count * YEAR_CYCLE as u64 + current_cycle_year_count - 1;
        let mut month = (net_month
            - YEAR_CYCLE_FIRSTMONTH_MONTH[current_cycle_year_count as usize - 1]
            + 1) as u8;

        if !Year::is_common(year) {
            month -= 1;
        }

        (year + 1, Self::from_number(month))
    }
}

impl Reverse for Month {
    fn reverse(number: u64) -> u64 {
        let number = number - 1;
        let month_cycle_count = number / MONTH_CYCLE as u64;
        let current_cycle_month_count = number % MONTH_CYCLE as u64;
        let current_cycle_day_count = MONTH_CYCLE_FIRSTDAY_DAY[current_cycle_month_count as usize];
        current_cycle_day_count as u64 + month_cycle_count * MONTH_CYCLE_DAY_COUNT as u64 + 1
    }
}

impl Default for Month {
    fn default() -> Self {
        Self::from_number(0)
    }
}
