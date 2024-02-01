use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (s, m, l) = next!(u32, u32, u32);

    if s + m * 2 + l * 3 >= 10 {
        print!("happy");
    } else {
        print!("sad");
    }
}