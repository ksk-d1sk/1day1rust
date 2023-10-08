fn main() {
    let slice: &[u8] = &[1, 2, 3, 4, 5];
    let u8_vec = slice.to_owned();
    assert_eq!(u8_vec, vec![1, 2, 3, 4, 5]);

    let str = "아 오락실 가서 사볼 한판 땡기고싶다";
    let string = str.to_owned();
    assert_eq!(string, String::from("아 오락실 가서 사볼 한판 땡기고싶다"));
}
