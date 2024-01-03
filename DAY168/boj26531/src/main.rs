use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (a, b) = get!(i16, i16);
    let (k, x) = get!(i16, i16);
    let (l, r) = (a.max(k - x), b.min(k + x));
    let res = r - l + 1;

    if res > 0 {
        print!("{res}");
    } else {
        print!("IMPOSSIBLE");
    }
}