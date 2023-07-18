fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();

    print!("{t:V>v$}{t:I>i$}", t = "", v = n / 5, i = n % 5);
}