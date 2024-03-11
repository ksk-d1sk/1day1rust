use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $t:ty ) => { tokens.next().unwrap().parse::<$t>().unwrap() };
    }

    let n = next!(u32);
    let sum: u32 = (0..n)
        .map(|_| next!(u32))
        .sum();

    print!("{}", sum + 1 - n);
}