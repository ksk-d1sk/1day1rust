use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (wc, hc, ws, hs) = next!(i16, i16, i16, i16);

    if ws <= wc - 2 && hs <= hc - 2 {
        print!("1");
    } else {
        print!("0");
    }
}