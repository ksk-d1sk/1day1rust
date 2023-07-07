fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<f64>);
    let mut get = || input.next().unwrap();
    print!("{}", ((get() + 1.0) * (get() + 1.0) / (get() + 1.0) - 1.0).floor());
}
