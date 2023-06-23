fn main() {
    let _error_message = "too many pets".to_string();

    assert_eq!(
        format!("{} {:02} {:02} N", 24, 5, 23),
        "24 05 23 N"
    );

    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(", "), "veni, vidi, vici");

    assert!("ONE".to_lowercase() == "one");
    assert!("peanut".contains("nut"));
    assert_eq!("      str\r\n\r\n".trim(), "str");

    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with("v"));
    }
}
