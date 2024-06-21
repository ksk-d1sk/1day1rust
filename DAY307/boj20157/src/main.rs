use std::collections::HashMap;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut map = HashMap::new();

    for (x, y) in (0..n).map(|_| next!(i32, i32)) {
        map.entry(slope(x, y)).and_modify(|e| *e += 1).or_insert(1);
    }

    print!("{}", map.values().max().unwrap());
}

fn slope(x: i32, y: i32) -> (i32, i32) {
    if x == 0 {
        if y > 0 {
            (1, 1)
        } else {
            (0, 0)
        }
    } else if y == 0 {
        if x > 0 {
            (1, 0)
        } else {
            (0, 1)
        }
    } else {
        let abs_x = x.abs();
        let abs_y = y.abs();
        let gcd = euclidean(abs_x, abs_y);
        (x / gcd, y / gcd)
    }
}

fn euclidean(a: i32, b: i32) -> i32 {
    fn gcd(a: i32, b: i32) -> i32 {
        let m = a % b;
        if m == 0 {
            b
        } else {
            gcd(b, m)
        }
    }

    if a > b {
        gcd(a, b)
    } else {
        gcd(b, a)
    }
}