use std::io::*;
use std::fmt::Write;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(u16);
    for (a, b) in (0..t).map(|_| next!(u8, u8)) {
        let mut answer = 0;

        answer += match a {
            1 => 5_000_000,
            2..=3 => 3_000_000,
            4..=6 => 2_000_000,
            7..=10 => 500_000,
            11..=15 => 300_000,
            16..=21 => 100_000,
            _ => 0,
        };

        answer += match b {
            1 => 5_120_000,
            2..=3 => 2_560_000,
            4..=7 => 1_280_000,
            8..=15 => 640_000,
            16..=31 => 320_000,
            _ => 0,
        };

        let _ = writeln!(output, "{answer}");
    }

    print!("{output}");
}