use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (sh, _, sm, _ ,ss) = next!(u32, char, u32, char, u32);
    let (eh, _, em, _ ,es) = next!(u32, char, u32, char, u32);

    let start = sh * 60 * 60 + sm * 60 + ss;
    let end   = eh * 60 * 60 + em * 60 + es;

    if start < end {
        print!("{}", end - start);
    } else {
        print!("{}", 24 * 60 * 60 + end - start);
    }
}