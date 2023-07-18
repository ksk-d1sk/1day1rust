fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut sum = 0;

    for bottle in input.split_ascii_whitespace().flat_map(str::parse::<u16>) {
        sum += bottle * 5;
    }

    print!("{sum}");
}
