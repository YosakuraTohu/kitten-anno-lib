#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

use crate::*;

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct IsCommonSt {
    pub year: bool,
    pub month: bool,
}

impl IsCommonSt {
    #[inline(always)]
    pub(crate) fn from_raw_number(raw_year: u64, raw_month: u64) -> Self {
        let year = Year::is_common(raw_year - 1);
        let month = Month::is_common(raw_month);
        Self { year, month }
    }
}
