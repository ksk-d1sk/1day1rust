use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let s = next!().as_bytes();
    let t = next!().as_bytes();
    let (k, l) = (s.len(), t.len());
    let (a, b) = if k > l { (k, l) } else { (l, k) };
    let gcd = euclidean(a, b);
    let lcm = k * l / gcd;
    let mut check = true;

    for i in 0..lcm {
        if s[i % k] != t[i % l] {
            check = false;
            break;
        }
    }

    print!("{}", check as u8);
}

fn euclidean(a: usize, b: usize) -> usize {
    let m = a % b;
    if m == 0 {
        b
    } else {
        euclidean(b, m)
    }
}