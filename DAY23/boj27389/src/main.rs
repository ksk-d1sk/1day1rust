fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    print!("{:.02}", input.trim().parse::<f64>().unwrap() / 4.0);
}
