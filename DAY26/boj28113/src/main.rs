fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ( $(input.next().unwrap().parse::<$t>().unwrap()),+ ) };
    }

    let (_, a, b) = get!(u32, u32, u32);

    print!(
        "{}",
        if a < b {
            "Bus"
        } else
        if a == b {
            "Anything"
        } else {
            "Subway"
        }
    )
}
