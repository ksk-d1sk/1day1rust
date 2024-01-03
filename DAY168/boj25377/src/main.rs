use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(u8);
    let answer = (0..n)
        .map(|_| get!(i16, i16))
        .filter_map(|(a, b)| (a <= b).then_some(b))
        .min()
        .unwrap_or(-1);

    print!("{answer}");
}