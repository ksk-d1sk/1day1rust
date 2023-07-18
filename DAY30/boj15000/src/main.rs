fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    print!("{}", input.to_uppercase());
}
