use crate::*;

#[derive(Debug)]
pub struct Chord {
    pub number: u8,
    pub str: String,
}

impl Chord {
    pub fn from_raw_number(number: u64) -> Self {
        let chord = ((number % CHORD_DAY_COUNT as u64) + 1) as u8;
        Self::from_number(chord)
    }
}

impl FromNumber for Chord {
    fn from_number(number: u8) -> Self {
        Self {
            number,
            str: MEANING_OF_CHORD[(number - 1) as usize].to_string(),
        }
    }
}

impl Default for Chord {
    fn default() -> Self {
        Self::from_number(1)
    }
}
