use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u16);
    let mut ans_x = 0;
    let mut ans_y = i16::MAX;

    for (x, y) in (0..n).map(|_| next!(i16, i16)) {
        if y < ans_y {
            ans_x = x;
            ans_y = y;
        }
    }

    print!("{ans_x} {ans_y}");
}