#[cfg(feature = "wasmbind")]
use wasm_bindgen::prelude::*;

use crate::*;

#[cfg_attr(feature = "wasmbind", wasm_bindgen(getter_with_clone))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Month {
    pub number: u8,
    #[cfg(feature = "std")]
    pub str: String,
    #[cfg(feature = "std")]
    pub flower: String,
    #[cfg(feature = "std")]
    pub elemental: String,
    #[cfg(feature = "std")]
    pub imagery: String,
}

impl IsCommon for Month {
    #[inline(always)]
    fn is_common(number: u64) -> bool {
        !matches!((number % MONTH_CYCLE as u64) as u8, 1 | 4 | 8)
    }
}

impl FromNumber for Month {
    #[inline(always)]
    fn from_number(number: u8) -> Self {
        #[cfg(feature = "std")]
        let [month_str, elemental, imagery, flower] = MEANING_OF_MONTH[number as usize];
        Self {
            number,
            #[cfg(feature = "std")]
            str: month_str.to_string(),
            #[cfg(feature = "std")]
            flower: flower.to_string(),
            #[cfg(feature = "std")]
            elemental: elemental.to_string(),
            #[cfg(feature = "std")]
            imagery: imagery.to_string(),
        }
    }
}

impl FromRawNumber for Month {
    #[inline(always)]
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
    #[inline(always)]
    fn reverse(number: u64) -> u64 {
        let number = number - 1;
        let month_cycle_count = number / MONTH_CYCLE as u64;
        let current_cycle_month_count = number % MONTH_CYCLE as u64;
        let current_cycle_day_count = MONTH_CYCLE_FIRSTDAY_DAY[current_cycle_month_count as usize];
        current_cycle_day_count as u64 + month_cycle_count * MONTH_CYCLE_DAY_COUNT as u64 + 1
    }
}

impl Default for Month {
    #[inline(always)]
    fn default() -> Self {
        Self::from_number(0)
    }
}
