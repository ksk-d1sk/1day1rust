fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: u8 = input.trim().parse().unwrap();

    print!("{}", 1 << n);
}
