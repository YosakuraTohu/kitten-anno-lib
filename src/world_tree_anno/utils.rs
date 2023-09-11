use chrono::Local;
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Year(pub u64);
impl Year {
    pub(crate) fn is_common_year(&self) -> bool {
        match (self.0 % YEAR_CYCLE as u64) as u8 {
            1 | 4 | 7 | 10 | 13 | 15 | 18 | 21 | 24 | 27 => false,
            _ => true,
        }
    }

    pub(crate) fn get_year_cycle_firstmonth_month() -> [u16; YEAR_CYCLE as usize] {
        let year_cycle_firstmonth_month: &mut [u16; 29] = &mut [0; YEAR_CYCLE as usize];
        for i in 1..YEAR_CYCLE {
            year_cycle_firstmonth_month[i as usize] = year_cycle_firstmonth_month[(i - 1) as usize]
                + COMMON_YEAR_MONTH_COUNT as u16
                + if Year((i - 1) as u64).is_common_year() {
                    0
                } else {
                    1
                };
        }
        *year_cycle_firstmonth_month
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Month(pub u64);
impl Month {
    pub(crate) fn is_common_month(self) -> bool {
        match (self.0 % MONTH_CYCLE as u64) as u8 {
            1 | 4 | 8 => false,
            _ => true,
        }
    }

    pub(crate) fn get_month_cycle_firstday_day() -> [u8; MONTH_CYCLE as usize] {
        let month_cycle_firstday_day: &mut [u8; 10] = &mut [0; MONTH_CYCLE as usize];
        for i in 1..MONTH_CYCLE {
            month_cycle_firstday_day[i as usize] = month_cycle_firstday_day[(i - 1) as usize]
                + COMMON_MONTH_DAY_COUNT
                + if Month((i - 1) as u64).is_common_month() {
                    0
                } else {
                    1
                };
        }
        *month_cycle_firstday_day
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Day(pub u64);
impl Day {
    pub(crate) fn get_month_day(self) -> (Month, u8) {
        let month_cycle_count: u64 = self.0 / MONTH_CYCLE_DAY_COUNT as u64;
        let net_day: u8 = (self.0 % MONTH_CYCLE_DAY_COUNT as u64) as u8;
        let mut current_cycle_month_count: u8 = 0;

        for i in *MONTH_CYCLE_FIRSTDAY_DAY {
            if net_day < i {
                break;
            }
            current_cycle_month_count += 1;
        }
        let month: Month =
            Month(month_cycle_count * MONTH_CYCLE as u64 + current_cycle_month_count as u64 - 1);
        let date: u8 =
            net_day - (*MONTH_CYCLE_FIRSTDAY_DAY)[(current_cycle_month_count - 1) as usize] + 1;
        (month, date)
    }

    pub(crate) fn get_day_anno(self) -> Anno {
        let (month, date) = self.get_month_day();
        let (year, month_number) = {
            let year_cycle_count: u64 = month.0 / YEAR_CYCLE_MONTH_COUNT as u64;
            let net_month: u16 = (month.0 % YEAR_CYCLE_MONTH_COUNT as u64) as u16;
            let mut current_cycle_year_count: u8 = 0;

            for i in *YEAR_CYCLE_FIRSTMONTH_MONTH {
                if net_month < i {
                    break;
                }
                current_cycle_year_count += 1;
            }
            let year: Year =
                Year(year_cycle_count * YEAR_CYCLE as u64 + current_cycle_year_count as u64 - 1);
            let mut month_number: u8 = (net_month
                - (*YEAR_CYCLE_FIRSTMONTH_MONTH)[(current_cycle_year_count - 1) as usize]
                + 1) as u8;
            if !year.is_common_year() {
                month_number -= 1;
            }
            (year, month_number)
        };
        let year_number: u64 = year.0 + 1;
        let [month_str, elemental, imagery, flower] = MEANING_OF_MONTH[month_number as usize];
        let chord_number: u8 = ((self.0 % CHORD_DAY_COUNT as u64) + 1) as u8;
        let chord_str: &str = MEANING_OF_CHORD[chord_number as usize - 1];

        let bit_num = |number: u64| -> u8 {
            let mut number: u64 = number;
            let mut count: u8 = 0;
            while number > 0 {
                number = number / 10;
                count += 1;
            }
            count
        };

        let to_chinese_number = |number: u64| -> String {
            let mut c_number: u64 = number;
            let mut result: String = String::new();
            for _ in 0..bit_num(number) {
                result.insert_str(0, ARR_NUMBER_STRING[(c_number % 10) as usize]);
                c_number = c_number / 10;
            }
            result
        };

        let year_str = |number: u64| -> String {
            if number == 1 {
                return "世界树纪元元年".to_string();
            }
            format!("世界树纪元{}年", to_chinese_number(number))
        };

        let day_str = |number: u8| -> String {
            match (number / 10, number % 10 == 0) {
                (0, false) => format!("初{}", to_chinese_number((number % 10) as u64)),
                (1, true) => "初十".to_string(),
                (1, false) => format!("十{}", to_chinese_number((number % 10) as u64)),
                (2, true) => "二十".to_string(),
                (2, false) => format!("廿{}", to_chinese_number((number % 10) as u64)),
                _ => "".to_string(),
            }
        };

        Anno {
            year_number,
            month_number,
            chord_number,
            date,
            year_str: year_str(year_number),
            month_str: month_str.to_string(),
            chord_str: chord_str.to_string(),
            day_str: day_str(date),
            hour: 0,
            minute: 0,
            second: 0,
            flower: flower.to_string(),
            elemental: elemental.to_string(),
            imagery: imagery.to_string(),
        }
    }
}

#[cfg_attr(target_family = "wasm", wasm_bindgen(getter_with_clone))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Anno {
    pub year_number: u64,
    pub month_number: u8,
    pub chord_number: u8,
    pub date: u8,
    pub year_str: String,
    pub month_str: String,
    pub chord_str: String,
    pub day_str: String,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub flower: String,
    pub elemental: String,
    pub imagery: String,
}
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
impl Anno {
    /// 从现实时间获取Anno，从 KITTEN_DAY 2017-04-25 00:00:00
    /// 开始，时间流速为现实的72倍
    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    pub fn get_anno() -> Anno {
        let unix = Local::now();
        let wta_unix: u64 = 72 * (unix.timestamp() as u64 - *KITTEN_TIME)
            + (unix.timestamp_micros() % 1000000 * 72 / 1000000) as u64;
        Anno::from_timestamp(wta_unix)
    }

    /// 从时间戳获取Anno，从世界树元年开始
    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    pub fn from_timestamp(timestamp: u64) -> Anno {
        let day = Day(timestamp / SECONDS_PER_DAY as u64);
        let seconds_today: u32 = (timestamp % SECONDS_PER_DAY as u64) as u32;
        let mut anno = day.get_day_anno();
        anno.hour = (seconds_today / 3600) as u8;
        anno.minute = ((seconds_today % 3600) / 60) as u8;
        anno.second = ((seconds_today % 3600) % 60) as u8;
        anno
    }

    /// 从天数获取Anno，天数从0开始，
    /// 0对应的是 世界树纪元元年雪月初一 0:00:00
    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    pub fn from_days(days: u64) -> Anno {
        let day = Day(days);
        day.get_day_anno()
    }

    /// 从月数获取Anno，月数从0开始，
    /// 0对应的是 世界树纪元元年雪月初一 0:00:00
    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    pub fn from_months(months: u64) -> Anno {
        Anno::from_days(Anno::from_months_to_days(months))
    }

    /// 从月数获取天数，月数从0开始
    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    pub fn from_months_to_days(months: u64) -> u64 {
        let month_cycle_count: u64 = (months + 1) / MONTH_CYCLE as u64;
        let current_cycle_month_count: u8 =
            (months + 1 - month_cycle_count * MONTH_CYCLE as u64) as u8;
        let current_cycle_day_count: u8 =
            (*MONTH_CYCLE_FIRSTDAY_DAY)[current_cycle_month_count as usize - 1];
        current_cycle_day_count as u64 + month_cycle_count * MONTH_CYCLE_DAY_COUNT as u64
    }

    /// 从年数获取Anno，年数从0开始，
    /// 0对应的是 世界树纪元元年雪月初一 0:00:00
    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    pub fn from_years(years: u64) -> Anno {
        Anno::from_months(Anno::from_years_to_months(years))
    }

    /// 从年数获取月数，年数从0开始
    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    pub fn from_years_to_months(years: u64) -> u64 {
        let year_cycle_count: u64 = (years + 1) / YEAR_CYCLE as u64;
        let current_cycle_year_count: u8 = (years + 1 - year_cycle_count * YEAR_CYCLE as u64) as u8;
        let current_cycle_month_count: u16 =
            (*YEAR_CYCLE_FIRSTMONTH_MONTH)[current_cycle_year_count as usize - 1];
        current_cycle_month_count as u64 + year_cycle_count * MONTH_CYCLE_DAY_COUNT as u64
    }

    /// 从年数获取天数，年数从0开始
    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    pub fn from_years_to_days(years: u64) -> u64 {
        Anno::from_months_to_days(Anno::from_years_to_months(years))
    }
    /// 将Anno转换为String格式，要注意的是，
    /// .to_string() 后Anno的所有权将会被转移走，
    /// 如果之后还要用到Anno，请 .clone() 后再
    /// .to_string()
    #[cfg_attr(target_family = "wasm", wasm_bindgen)]
    pub fn to_string(self) -> String {
        let fill_in = |number: u8| -> String {
            if number < 10 {
                return format!("0{}", number);
            }
            format!("{}", number)
        };

        format!(
            "{}{}{} {}:{}:{}",
            self.year_str,
            self.month_str,
            self.day_str,
            self.hour,
            fill_in(self.minute),
            fill_in(self.second)
        )
    }
}
