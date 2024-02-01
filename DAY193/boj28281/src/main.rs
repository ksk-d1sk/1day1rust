use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, x) = next!(u32, u32);
    let v: Vec<_> = (0..n).map(|_| next!(u32) * x).collect();

    print!("{}", v.windows(2).map(|slice| slice[0] + slice[1]).min().unwrap());
}