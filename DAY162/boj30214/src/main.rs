use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (a, b) = get!(u8, u8);
    print!(
        "{}",
        if a >= (b + 1) / 2 { "E" } else { "H" }
    );
}