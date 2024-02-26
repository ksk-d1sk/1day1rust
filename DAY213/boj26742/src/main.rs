use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let s = next!();
    let mut c = 0;
    let mut b = 0;

    for i in s.bytes() {
        if i == b'C' {
            c += 1;
        } else {
            b += 1;
        }
    }

    print!("{}", c / 2 + b / 2);
}