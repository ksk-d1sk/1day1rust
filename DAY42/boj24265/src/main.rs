fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let n: u64 = input.trim().parse().unwrap();

    print!(
        "{}\n2",
        (1..n).sum::<u64>()
    )
}