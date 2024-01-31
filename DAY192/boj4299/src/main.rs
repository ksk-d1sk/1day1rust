use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (a, b) = next!(i16, i16);

    if a + b < 0 || a - b < 0 || (a + b) & 1 == 1{
        print!("-1");
    } else {
        let x = (a + b) / 2;
        let y = a - x;
        print!("{x} {y}");
    }
}