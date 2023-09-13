use crate::*;

pub trait FromYearNumber {
    fn from_number(number: u64) -> Self;
}

pub trait FromNumber {
    fn from_number(number: u8) -> Self;
}

pub trait FromRawNumber {
    fn from_raw_number(number: u64) -> (u64, Self);
}

pub(crate) trait IsCommon {
    fn is_common(number: u64) -> bool;
}

pub(crate) fn get_year_cycle_firstmonth_month() -> [u16; YEAR_CYCLE as usize] {
    let year_cycle_firstmonth_month = &mut [0; YEAR_CYCLE as usize];
    for i in 1..YEAR_CYCLE {
        year_cycle_firstmonth_month[i as usize] = year_cycle_firstmonth_month[(i - 1) as usize]
            + COMMON_YEAR_MONTH_COUNT as u16
            + if Year::is_common((i - 1) as u64) {
                0
            } else {
                1
            };
    }
    *year_cycle_firstmonth_month
}

pub(crate) fn get_month_cycle_firstday_day() -> [u8; MONTH_CYCLE as usize] {
    let month_cycle_firstday_day = &mut [0; MONTH_CYCLE as usize];
    for i in 1..MONTH_CYCLE {
        month_cycle_firstday_day[i as usize] = month_cycle_firstday_day[(i - 1) as usize]
            + COMMON_MONTH_DAY_COUNT
            + if Month::is_common((i - 1) as u64) {
                0
            } else {
                1
            };
    }
    *month_cycle_firstday_day
}

pub(crate) fn bit_num(number: u64) -> u8 {
    let mut number = number;
    let mut count = 0;
    while number > 0 {
        number = number / 10;
        count += 1;
    }
    count
}

pub(crate) fn to_chinese_number(number: u64) -> String {
    let mut c_number = number;
    let mut result = String::new();
    for _ in 0..bit_num(number) {
        result.insert_str(0, ARR_NUMBER_STRING[(c_number % 10) as usize]);
        c_number = c_number / 10;
    }
    result
}

pub(crate) fn year_str(number: u64) -> String {
    if number == 1 {
        return "世界树纪元元年".to_string();
    }
    format!("世界树纪元{}年", to_chinese_number(number))
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
