fn main() {
    let text = "
        12     23   45
        one two   000   0.1
              three   4.5    333
        B B K K B K K 10 TH ANNIVERSARY!!!
        LET THE BASS KICK       666       777
    ";

    println!("{:?}", get_number1(text));
    assert_eq!(get_number1(text), get_number2(text));
}

fn get_number1(string: &str) -> Vec<isize> {
    string.split_ascii_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect()
}

fn get_number2(string: &str) -> Vec<isize> {
    string.split_ascii_whitespace()
        .map(|w| w.parse())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect()
}
