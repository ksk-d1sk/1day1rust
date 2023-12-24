use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (x1, x2) = get!(i32, i32);
    let (a, b, c) = get!(i32, i32, i32);
    let (d, e) = get!(i32, i32);

    let r1 = (a / 3 * x1.pow(3)) + ((b - d) / 2 * x1.pow(2)) + ((c - e) * x1);
    let r2 = (a / 3 * x2.pow(3)) + ((b - d) / 2 * x2.pow(2)) + ((c - e) * x2);

    print!("{}", r2 - r1);
}