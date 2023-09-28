use std::io;
use std::fmt::Write;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    for _ in 0..get!(usize) {
        let (a, b, x) = get!(i32, i32, i32);
        let _ = writeln!(output,  "{}", a * (x - 1) + b);
    }

    print!("{output}");
}