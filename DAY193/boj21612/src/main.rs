use std::cmp::Ordering::*;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let b = next!(u16);
    let p = b * 5 - 400;

    print!(
        "{}\n{}",
        p,
        match p.cmp(&100) {
            Greater => -1,
            Equal => 0,
            Less => 1,
        }
    );
}