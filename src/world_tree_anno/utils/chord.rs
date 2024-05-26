#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

use crate::*;

#[cfg_attr(target_family = "wasm", wasm_bindgen(getter_with_clone))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Chord {
    pub number: u8,
    pub str: String,
}

impl Chord {
    #[inline(always)]
    pub(crate) fn from_raw_number(number: u64) -> Self {
        let chord = ((number % CHORD_DAY_COUNT as u64) + 1) as u8;
        Self::from_number(chord)
    }
}

impl FromNumber for Chord {
    #[inline(always)]
    fn from_number(number: u8) -> Self {
        Self {
            number,
            str: MEANING_OF_CHORD[(number - 1) as usize].to_string(),
        }
    }
}

impl Default for Chord {
    #[inline(always)]
    fn default() -> Self {
        Self::from_number(1)
    }
}
