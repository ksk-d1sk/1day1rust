use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(u32);
    for (mut a, mut b, k) in (0..t).map(|_| next!(u32, u32, u32)) {
        let mut check = false;
        let mut answer = 0;

        while a > 0 && b > 0 {
            if a.min(99) * k == b.min(99) || a.min(99) == b.min(99) * k {
                check = true;
                break;
            }

            if a.min(b) >= 99 {
                let t = a.min(b) - 98;
                answer += t;
                a -= t;
                b -= t;
            } else {
                answer += 1;
                a -= 1;
                b -= 1;
            }
        }

        let _ = if check {
            writeln!(output, "{answer}")
        } else {
            writeln!(output, "-1")
        };
    }

    print!("{output}");
}