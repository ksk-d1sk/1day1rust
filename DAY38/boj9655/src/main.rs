fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let n: u16 = input.trim().parse().unwrap();

    print!(
        "{}",
        if n & 1 == 0 {
            "CY"
        } else {
            "SK"
        }
    )
}