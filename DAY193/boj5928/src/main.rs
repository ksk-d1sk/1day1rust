use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (d, h, m) = next!(i32, i32, i32);
    let temp = (d * 24 * 60 + h * 60 + m) - (11 * 24 * 60 + 11 * 60 + 11);

    if temp < 0  {
        print!("-1");
    } else {
        print!("{temp}");
    }
}