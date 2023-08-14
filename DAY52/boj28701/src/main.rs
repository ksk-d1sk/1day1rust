fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let a1 = n * (n + 1) / 2;
    let a2 = a1 * a1;

    println!("{0}\n{1}\n{1}", a1, a2);
}