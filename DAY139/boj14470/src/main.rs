use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut ans = 0;
    let (a, b, c, d, e) = get!(i16, i16, i16, i16, i16);

    for _ in a..0 {
        ans += c;
    }

    if a <= 0 { ans += d };

    ans += (b - a.max(0)) * e;

    print!("{ans}");
}