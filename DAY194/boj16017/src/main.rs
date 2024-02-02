use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (a, b, c, d) = next!(u8, u8, u8, u8);

    if (a == 8 || a == 9) && b == c && (d == 8 || d == 9) {
        print!("ignore");
    } else {
        print!("answer");
    }
}