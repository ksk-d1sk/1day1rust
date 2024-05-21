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

    let t = next!(u8);
    for i in 1..=t {
        let (a, b, c) = next!(u32, u32, u32);
        let (m1, m2) = if a > b { (a, b) } else { (b, a) };
        let (m1, m3) = if m1 > c { (m1, c) } else { (c, m1) };
        let _ = writeln!(
            output,
            "Case #{i}: {}",
            if m1 >= m2 + m3 {
                "invalid!"
            } else if a == b && b == c {
                "equilateral"
            } else if a == b || b == c || c == a{
                "isosceles"
            } else {
                "scalene"
            }
        );
    }

    print!("{output}");
}