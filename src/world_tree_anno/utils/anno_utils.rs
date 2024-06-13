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

#[inline(always)]
#[cfg(feature = "std")]
pub(crate) fn to_chinese_number(number: u64, capacity: usize) -> String {
    let mut c_number = number;
    let mut result = String::with_capacity(capacity);
    while c_number > 0 {
        let digit = c_number % 10;
        result.insert_str(0, ARR_NUMBER_STRING[digit as usize]);
        c_number /= 10;
    }
    result
}

#[inline(always)]
#[cfg(feature = "std")]
pub(crate) fn year_str(number: u64) -> String {
    let capacity = (number.ilog10() as usize + 1) * 3;
    let mut result = String::with_capacity(6 * 3 + capacity);
    match number {
        1 => result.push_str("世界树纪元元年"),
        _ => {
            result.push_str("世界树纪元");
            result.push_str(&to_chinese_number(number, capacity));
            result.push('年');
        }
    };
    result
}

#[inline(always)]
#[cfg(feature = "std")]
pub(crate) fn day_str(number: u8) -> String {
    let mut result = String::with_capacity(2 * 3);
    match (number / 10, number % 10 == 0) {
        (0, false) => {
            result.push('初');
            result.push_str(&to_chinese_number((number % 10) as u64, 3));
        }
        (1, true) => result.push_str("初十"),
        (1, false) => {
            result.push('十');
            result.push_str(&to_chinese_number((number % 10) as u64, 3));
        }
        (2, true) => result.push_str("二十"),
        (2, false) => {
            result.push('廿');
            result.push_str(&to_chinese_number((number % 10) as u64, 3));
        }
        _ => (),
    };
    result
}

#[inline(always)]
#[cfg(feature = "std")]
pub(crate) fn hms_str(hour: u8, minute: u8, second: u8) -> String {
    let mut result = [0u8; 8];
    let mut offset = 0;

    if hour >= 10 {
        result[0] = 0x30 + hour / 10;
        offset += 1;
    }
    result[offset] = 0x30 + hour % 10;
    result[1 + offset] = b':';
    result[2 + offset] = 0x30 + minute / 10;
    result[3 + offset] = 0x30 + minute % 10;
    result[4 + offset] = b':';
    result[5 + offset] = 0x30 + second / 10;
    result[6 + offset] = 0x30 + second % 10;

    String::from_utf8_lossy(&result).to_string()
}
