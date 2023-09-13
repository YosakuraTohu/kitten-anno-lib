use crate::*;

#[derive(Debug)]
pub struct Year {
    pub number: u64,
    pub str: String,
}

impl IsCommon for Year {
    fn is_common(number: u64) -> bool {
        match (number % YEAR_CYCLE as u64) as u8 {
            1 | 4 | 7 | 10 | 13 | 15 | 18 | 21 | 24 | 27 => false,
            _ => true,
        }
    }
}

impl FromYearNumber for Year {
    fn from_number(number: u64) -> Self {
        Self {
            number,
            str: year_str(number),
        }
    }
}

impl Default for Year {
    fn default() -> Self {
        Year::from_number(1)
    }
}
