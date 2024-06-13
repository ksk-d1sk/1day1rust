use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u32);
    let sizes = Vec::from_iter((0..6).map(|_| next!(u32)));
    let (t, p) = next!(u32, u32);
    let sum_t = sizes.iter().fold(0, |acc, &size| acc + (size + t - 1) / t);

    print!("{sum_t}\n{} {}", n / p, n % p);
}