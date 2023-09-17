use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(i64);
    print!("{}\n3", solve(n));
}

fn solve(n: i64) -> i64 {
    let mut sum = 0;
    for i in 1..=(n - 2) {
        sum += (n - 1 - i) * i;
    }

    sum
}
