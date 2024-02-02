use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (l, r) = next!(u8, u8);

    if l == 0 && r == 0 {
        print!("Not a moose");
    } else if l == r {
        print!("Even {}", l * 2);
    } else {
        print!("Odd {}", l.max(r) * 2);
    }
}