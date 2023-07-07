fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<f64>);
    let mut get = || input.next().unwrap();
    print!("{}", u8::from(get() * (100.0 - get()) < 10000.0));
}
