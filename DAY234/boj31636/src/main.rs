use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    // let input = include_str!("input.txt");
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    next!();
    let s = next!();

    if s.contains("ooo") {
        print!("Yes");
    } else {
        print!("No");
    }
}