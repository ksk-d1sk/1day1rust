use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(usize);
    for (m, n, x, y) in (0..t).map(|_| next!(i32, i32, i32, i32)) {
        let mut answer = -1;
        let mut k = x;
        let l = lcm(m, n);
        let y = if y == n { 0 } else { y };

        while k <= l {
            if k % n == y {
                answer = k;
                break;
            }

            k += m;
        }

        let _ = writeln!(output, "{answer}");
    }

    print!("{output}");
}

fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a.max(b), a.min(b))
}

fn gcd(a: i32, b: i32) -> i32 {
    let m = a % b;
    if m == 0 {
        b
    } else {
        gcd(b, m)
    }
}