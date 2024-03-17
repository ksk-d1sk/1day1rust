use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    next!();
    let mut s = next!().bytes();
    let c = s.next().unwrap();

    if s.all(|b| b == c) {
        print!("Yes");
    } else {
        print!("No");
    }
}