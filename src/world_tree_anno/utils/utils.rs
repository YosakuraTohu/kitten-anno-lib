use crate::*;

pub(crate) trait FromYearNumber {
    fn from_number(number: u64) -> Self;
}

pub(crate) trait FromNumber {
    fn from_number(number: u8) -> Self;
}

pub(crate) trait FromRawNumber {
    fn from_raw_number(number: u64) -> (u64, Self);
}

pub(crate) trait Reverse {
    fn reverse(number: u64) -> u64;
}

pub(crate) trait IsCommon {
    fn is_common(number: u64) -> bool;
}

pub(crate) fn get_year_cycle_firstmonth_month() -> [u16; YEAR_CYCLE as usize] {
    let mut year_cycle_firstmonth_month = [0; YEAR_CYCLE as usize];
    for i in 1..YEAR_CYCLE {
        let prev_value = year_cycle_firstmonth_month[i as usize - 1];
        let additional_month = if Year::is_common(i as u64 - 1) { 0 } else { 1 };
        year_cycle_firstmonth_month[i as usize] =
            prev_value + COMMON_YEAR_MONTH_COUNT as u16 + additional_month;
    }
    year_cycle_firstmonth_month
}

pub(crate) fn get_month_cycle_firstday_day() -> [u8; MONTH_CYCLE as usize] {
    let mut month_cycle_firstday_day = [0; MONTH_CYCLE as usize];
    for i in 1..MONTH_CYCLE {
        let prev_value = month_cycle_firstday_day[i as usize - 1];
        let additional_day = if Month::is_common(i as u64 - 1) { 0 } else { 1 };
        month_cycle_firstday_day[i as usize] = prev_value + COMMON_MONTH_DAY_COUNT + additional_day;
    }
    month_cycle_firstday_day
}

pub(crate) fn to_chinese_number(number: u64) -> String {
    let mut c_number = number;
    let mut result = String::new();
    while c_number > 0 {
        let digit = c_number % 10;
        result.insert_str(0, ARR_NUMBER_STRING[digit as usize]);
        c_number = c_number / 10;
    }
    result
}

pub(crate) fn year_str(number: u64) -> String {
    match number {
        1 => "世界树纪元元年".to_string(),
        _ => format!("世界树纪元{}年", to_chinese_number(number)),
    }
}

pub(crate) fn day_str(number: u8) -> String {
    match (number / 10, number % 10 == 0) {
        (0, false) => format!("初{}", to_chinese_number((number % 10) as u64)),
        (1, true) => "初十".to_string(),
        (1, false) => format!("十{}", to_chinese_number((number % 10) as u64)),
        (2, true) => "二十".to_string(),
        (2, false) => format!("廿{}", to_chinese_number((number % 10) as u64)),
        _ => "".to_string(),
    }
}
