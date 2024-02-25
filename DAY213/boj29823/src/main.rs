use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    next!();
    let mut n = 0_i32;
    let mut e = 0_i32;
    let mut s = 0;
    let mut w = 0;

    for b in next!().bytes() {
        match b {
            b'N' => n += 1,
            b'E' => e += 1,
            b'S' => s += 1,
            b'W' => w += 1,
            _ => unreachable!(),
        }
    }

    print!("{}", (n - s).abs() + (e - w).abs());
}