fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let n: u64 = input.trim().parse().unwrap();

    print!(
        "{}\n3",
        n * n * n
    );
}