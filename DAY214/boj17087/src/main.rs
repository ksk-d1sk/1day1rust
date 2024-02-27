use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, s) = next!(usize, u32);
    let mut imoto: Vec<_> = (0..n).map(|_| next!(u32)).collect();

    imoto.push(s);
    imoto.sort_unstable();

    let v: Vec<_> = imoto.windows(2).map(|slice| slice[1] - slice[0]).collect();
    let mut gcd = v[0];

    for i in 1..n {
        gcd = euclid(gcd, v[i]);
    }

    print!("{gcd}");
}

fn euclid(a: u32, b: u32) -> u32 {
    let m = a % b;
    if m == 0 {
        b
    } else {
        euclid(b, m)
    }
}