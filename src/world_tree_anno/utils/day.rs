use crate::*;

#[derive(Debug)]
pub struct Day {
    pub number: u8,
    pub str: String,
}

impl FromNumber for Day {
    fn from_number(number: u8) -> Self {
        Self {
            number,
            str: day_str(number),
        }
    }
}

impl FromRawNumber for Day {
    fn from_raw_number(number: u64) -> (u64, Self) {
        let month_cycle_count = number / MONTH_CYCLE_DAY_COUNT as u64;
        let net_day = (number % MONTH_CYCLE_DAY_COUNT as u64) as u8;
        let mut current_cycle_month_count = 0;
        for i in *MONTH_CYCLE_FIRSTDAY_DAY {
            if net_day < i {
                break;
            }
            current_cycle_month_count += 1;
        }
        let month = month_cycle_count * MONTH_CYCLE as u64 + current_cycle_month_count - 1;
        let day =
            net_day - (*MONTH_CYCLE_FIRSTDAY_DAY)[(current_cycle_month_count - 1) as usize] + 1;
        (month, Self::from_number(day))
    }
}

impl Default for Day {
    fn default() -> Self {
        Self::from_number(1)
    }
}
