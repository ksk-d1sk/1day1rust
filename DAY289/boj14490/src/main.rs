use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.trim().split(':');

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(u32, u32);
    let gcd = if n > m {
        euclidean(n, m)
    } else {
        euclidean(m, n)
    };

    print!("{}:{}", n / gcd, m / gcd);
}

fn euclidean(a: u32, b: u32) -> u32 {
    let m = a % b;
    if m == 0 {
        b
    } else {
        euclidean(b, m)
    }
}