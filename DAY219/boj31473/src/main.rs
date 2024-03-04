use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.trim_end().split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u16);
    let a: u32 = (0..n).map(|_| next!(u32)).sum();
    let b: u32 = (0..n).map(|_| next!(u32)).sum();

    print!("{b} {a}");
}