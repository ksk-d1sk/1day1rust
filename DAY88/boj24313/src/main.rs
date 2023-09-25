use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (a, b) = get!(i16, i16);
    let c = get!(i16);
    let n = get!(i16);

    print!("{}", u8::from(a * n + b <= c * n && a <= c));
}
