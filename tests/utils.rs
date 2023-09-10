use kitten_anno_lib::Anno;
#[cfg(target_family = "wasm")]
use wasm_bindgen_test::*;

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn main() {
    let anno = Anno::get_anno();
    let anno_t = Anno::from_timestamp(11451419198101919811);
    let anno_d = Anno::from_days(0);
    let anno_m = Anno::from_months(55);
    let anno_y = Anno::from_years(2);
    let anno_s = anno.clone().to_string();
    let anno_t_s = anno_t.clone().to_string();
    let anno_d_s = anno_d.clone().to_string();
    let anno_m_s = anno_m.clone().to_string();
    let anno_y_s = anno_y.clone().to_string();

    assert_eq!(anno_t.year_number, 240849365727);
    assert_eq!(anno_t.month_number, 15);
    assert_eq!(anno_t.chord_number, 7);
    assert_eq!(anno_t.date, 15);
    assert_eq!(anno_t.year_str, "世界树纪元二四〇八四九三六五七二七年");
    assert_eq!(anno_t.month_str, "苏月");
    assert_eq!(anno_t.chord_str, "月海");
    assert_eq!(anno_t.day_str, "十五");
    assert_eq!(anno_t.hour, 11);
    assert_eq!(anno_t.minute, 31);
    assert_eq!(anno_t.second, 33);
    assert_eq!(anno_t.flower, "虞美人");
    assert_eq!(anno_t.elemental, "森林");
    assert_eq!(anno_t.imagery, "幽郁");

    assert_eq!(
        anno_t_s,
        "世界树纪元二四〇八四九三六五七二七年苏月十五 11:31:33"
    );
    assert_eq!(anno_d_s, "世界树纪元元年雪月初一 0:00:00");
    assert_eq!(anno_m_s, "世界树纪元三年雪月初一 0:00:00");
    assert_eq!(anno_y_s, "世界树纪元三年雪月初一 0:00:00");

    println!("{:#?}", anno);
    println!("{:#?}", anno_t);
    println!("{:#?}", anno_d);
    println!("{:#?}", anno_m);
    println!("{:#?}", anno_y);
    println!("{}", anno_s);
    println!("{}", anno_t_s);
    println!("{}", anno_d_s);
    println!("{}", anno_m_s);
    println!("{}", anno_y_s);
}
