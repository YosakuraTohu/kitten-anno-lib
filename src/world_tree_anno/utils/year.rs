#[cfg(feature = "wasmbind")]
use wasm_bindgen::prelude::*;

use crate::*;

#[cfg_attr(feature = "wasmbind", wasm_bindgen(getter_with_clone))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Year {
    pub number: u64,
    #[cfg(feature = "std")]
    pub str: String,
}

impl IsCommon for Year {
    #[inline(always)]
    fn is_common(number: u64) -> bool {
        !matches!(
            (number % YEAR_CYCLE as u64) as u8,
            1 | 4 | 7 | 10 | 13 | 15 | 18 | 21 | 24 | 27
        )
    }
}

impl FromYearNumber for Year {
    #[inline(always)]
    fn from_number(number: u64) -> Self {
        Self {
            number,
            #[cfg(feature = "std")]
            str: year_str(number),
        }
    }
}

impl Reverse for Year {
    #[inline(always)]
    fn reverse(number: u64) -> u64 {
        let number = number - 1;
        let year_cycle_count = number / YEAR_CYCLE as u64;
        let current_cycle_year_count = number % YEAR_CYCLE as u64;
        let current_cycle_month_count =
            YEAR_CYCLE_FIRSTMONTH_MONTH[current_cycle_year_count as usize];
        current_cycle_month_count as u64 + year_cycle_count * YEAR_CYCLE_MONTH_COUNT as u64 + 1
    }
}

impl Default for Year {
    fn default() -> Self {
        Year::from_number(1)
    }
}
