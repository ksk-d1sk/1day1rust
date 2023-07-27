fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (a, b) = get!(u64, u64);

    print!("{}", a * b / euclidean(a, b));
}

fn euclidean(a: u64, b: u64) -> u64 {
    let r = a % b;
    if r != 0 {
        euclidean(b, r)
    } else {
        b
    }
}