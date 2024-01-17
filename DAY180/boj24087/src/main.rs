use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (s, a, b) = get!(u32, u32, u32);
    print!("{}", (s.saturating_sub(a) + b - 1) / b * 100 + 250);
}