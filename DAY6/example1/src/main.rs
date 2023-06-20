fn main() {
    assert_eq!(10_u8.checked_add(20), Some(30));

    assert_eq!(100_u8.checked_add(200), None);

    assert_eq!((-128_i8).checked_div(-1), None);

    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);

    assert_eq!(500_i16.wrapping_mul(500), -12144);

    assert_eq!(5_i16.wrapping_shl(17), 10);

    assert_eq!(32760_i16.saturating_add(10), 32767);
    assert_eq!((-32760_i16).saturating_sub(10), -32768);

    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));

    assert!((-1. / f32::INFINITY).is_sign_negative());
    assert_eq!(-f32::MIN, f32::MAX);
}
