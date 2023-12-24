use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, a, b, c) = get!(u64, u64, u64, u64);

    print!(
        "{}",
        facto(n) / (facto(a) * facto(b) * facto(c))
    );
}

fn facto(n: u64) -> u64 {
    let mut result = 1;

    for i in 2..=n {
        result *= i;
    }
    
   result
}