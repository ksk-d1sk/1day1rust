use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().trim().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(i32, i32);
    if n - 7 > 0 {
        print!("{}", n - 7);
    } else {
        print!("{}", m + 7);
    }
}