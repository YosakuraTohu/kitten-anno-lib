use chrono::NaiveDateTime;
use lazy_static::lazy_static;

use crate::*;

pub(crate) static SECONDS_PER_DAY: u32 = 85653;
pub(crate) static KITTEN_DAY: &str = "2017-04-25T00:00:00";
lazy_static! {
    pub(crate) static ref KITTEN_TIME: u64 =
        NaiveDateTime::parse_from_str(KITTEN_DAY, "%Y-%m-%dT%H:%M:%S",)
            .unwrap()
            .and_utc()
            .timestamp() as u64;
}

pub(crate) static COMMON_YEAR_MONTH_COUNT: u8 = 27; // 平年的月数
pub(crate) static COMMON_MONTH_DAY_COUNT: u8 = 20; // 小月的天数
pub(crate) static CHORD_DAY_COUNT: u8 = 9; // 琴弦的天数
pub(crate) const YEAR_CYCLE: u8 = 29; // 闰年周期的年数
pub(crate) static CYCLE_LEAP_YEAR_COUNT: u8 = 10; // 每周期的闰年数
pub(crate) const MONTH_CYCLE: u8 = 10; // 大月周期的月数
pub(crate) static CYCLE_GREATER_MONTH_COUNT: u8 = 3; // 每周期的大月数
pub(crate) static YEAR_CYCLE_MONTH_COUNT: u16 =
    YEAR_CYCLE as u16 * COMMON_YEAR_MONTH_COUNT as u16 + CYCLE_LEAP_YEAR_COUNT as u16; // 闰年周期的月数
pub(crate) static MONTH_CYCLE_DAY_COUNT: u8 =
    MONTH_CYCLE * COMMON_MONTH_DAY_COUNT + CYCLE_GREATER_MONTH_COUNT; // 闰年周期的月数
#[cfg(feature = "std")]
pub(crate) static ARR_NUMBER_STRING: [&str; 10] =
    ["〇", "一", "二", "三", "四", "五", "六", "七", "八", "九"]; // 数字对应的字符

lazy_static! {
    pub(crate) static ref YEAR_CYCLE_FIRSTMONTH_MONTH: [u16; YEAR_CYCLE as usize] =
        get_year_cycle_firstmonth_month();
    pub(crate) static ref MONTH_CYCLE_FIRSTDAY_DAY: [u8; MONTH_CYCLE as usize] =
        get_month_cycle_firstday_day();
}

#[cfg(feature = "std")]
pub(crate) static MEANING_OF_CHORD: [&str; 9] = [
    "折纸", "赤空", "玉兰", "水光", "风荧", "玄冰", "月海", "日珥", "星灯",
];

#[cfg(feature = "std")]
pub(crate) static MEANING_OF_MONTH: &[[&str; 4]] = &[
    ["寂月", "死亡", "祈歌", "烟花"],
    ["雪月", "风雪", "飘荡", "山茶"],
    ["海月", "海洋", "深沉", "金花茶"],
    ["夜月", "暗夜", "虚乏", "墨兰"],
    ["彗月", "流星", "陨落", "腊梅"],
    ["凉月", "寒冰", "凝聚", "迷迭香"],
    ["芷月", "凛冬", "休憩", "茶花"],
    ["茸月", "河流", "苏醒", "春兰"],
    ["雨月", "雨露", "降临", "油菜花"],
    ["花月", "繁花", "盛开", "拟南芥"],
    ["梦月", "梦幻", "轨迹", "郁金香"],
    ["音月", "韵律", "共鸣", "风信子"],
    ["晴月", "云朵", "弥散", "紫罗兰"],
    ["岚月", "和春", "离去", "鸢尾"],
    ["萝月", "生命", "吟唱", "矢车菊"],
    ["苏月", "森林", "幽郁", "虞美人"],
    ["茜月", "田野", "丰饶", "栀子"],
    ["梨月", "明昼", "迷离", "薰衣草"],
    ["荷月", "湖泊", "静谧", "莲花"],
    ["茶月", "火焰", "灼烈", "满天星"],
    ["茉月", "炎夏", "告别", "茉莉"],
    ["铃月", "城市", "回响", "紫菀"],
    ["信月", "星辰", "守序", "桔梗"],
    ["瑶月", "时间", "归来", "素馨"],
    ["风月", "天空", "呓语", "桂花"],
    ["叶月", "大地", "呼唤", "芙蓉"],
    ["霜月", "山脉", "厚重", "菊花"],
    ["奈月", "清秋", "消逝", "油茶"],
];
