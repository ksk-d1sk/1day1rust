use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    for (a, b) in (0..get!(usize)).map(|_| get!(u64, u64)) {
        let _ = writeln!(output, "{}", a * b / gcd(a, b));
    }

    print!("{output}");
}

fn gcd(a: u64, b: u64) -> u64 {
    let r = a % b;
    if r == 0 {
        b
    } else {
        gcd(b, r)
    }
}
