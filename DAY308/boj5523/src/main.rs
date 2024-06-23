use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u32);
    let mut a_win = 0;
    let mut b_win = 0;

    for (a, b) in (0..n).map(|_| next!(u8, u8)) {
        if a > b {
            a_win += 1;
        } else if a < b {
            b_win += 1;
        }
    }

    print!("{a_win} {b_win}");
}