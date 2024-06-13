use kitten_anno_lib::Anno;
#[cfg(feature = "wasmbind")]
use wasm_bindgen_test::*;

#[test]
#[cfg_attr(feature = "wasmbind", wasm_bindgen_test)]
fn main() {
    #[cfg(feature = "std")]
    let anno = Anno::get_anno();
    let anno_t = Anno::from_timestamp(11451419198101919811);
    let anno_ce = Anno::from_common_era(159047490355605064);
    #[cfg(feature = "std")]
    let anno_s = anno.to_string();
    #[cfg(feature = "std")]
    let anno_t_s = anno_t.to_string();

    assert_eq!(anno_t.timestamp, 11451419198101919811);
    assert_eq!(anno_t.year.number, 240849365727);
    assert_eq!(anno_t.month.number, 15);
    assert_eq!(anno_t.chord.number, 7);
    assert_eq!(anno_t.day.number, 15);
    #[cfg(feature = "std")]
    assert_eq!(anno_t.year.str, "世界树纪元二四〇八四九三六五七二七年");
    #[cfg(feature = "std")]
    assert_eq!(anno_t.month.str, "苏月");
    #[cfg(feature = "std")]
    assert_eq!(anno_t.chord.str, "月海");
    #[cfg(feature = "std")]
    assert_eq!(anno_t.day.str, "十五");
    assert_eq!(anno_t.hms.hour, 11);
    assert_eq!(anno_t.hms.minute, 31);
    assert_eq!(anno_t.hms.second, 33);
    #[cfg(feature = "std")]
    assert_eq!(anno_t.month.flower, "虞美人");
    #[cfg(feature = "std")]
    assert_eq!(anno_t.month.elemental, "森林");
    #[cfg(feature = "std")]
    assert_eq!(anno_t.month.imagery, "幽郁");

    assert_eq!(anno_ce.timestamp, 11451419198101919808);
    assert_eq!(anno_ce.to_common_era(), 159047490355605064);

    #[cfg(feature = "std")]
    assert_eq!(
        anno_t_s,
        "世界树纪元二四〇八四九三六五七二七年苏月十五\u{3000}11:31:33\u{3000}月海"
    );

    #[cfg(feature = "std")]
    println!("{:#?}", anno);
    #[cfg(feature = "std")]
    println!("{:#?}", anno_t);
    #[cfg(feature = "std")]
    println!("{}", anno_s);
    #[cfg(feature = "std")]
    println!("{}", anno_t_s);
}
