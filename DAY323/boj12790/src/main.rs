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

    let t = next!(u16);
    for (a, b, c, d, e, f, g, h) in (0..t).map(|_| next!(i32, i32, i32, i32, i32, i32, i32, i32)) {
        let _ = writeln!(
            output,
            "{}",
            1.max(a + e) + 5 * 1.max(b + f) + 2 * 0.max(c + g) + 2 * (d + h)
        );
    }

    print!("{output}");
}