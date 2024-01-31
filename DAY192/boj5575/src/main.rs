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

    for _ in 0..3 {
        let (start_h, start_m, start_s) = next!(i8, i8, i8);
        let (end_h, end_m, end_s) = next!(i8, i8, i8);
        let (mut result_h, mut result_m, mut result_s) = (0, 0, 0);

        result_s += end_s - start_s;
        if result_s < 0 {
            result_s = 60 + result_s;
            result_m -= 1;
        }

        result_m += end_m - start_m;
        if result_m < 0 {
            result_m = 60 + result_m;
            result_h -= 1;
        }

        result_h += end_h - start_h;
        if result_h < 0 {
            result_h = 24 + result_h;
        }

        let _ = writeln!(output, "{result_h} {result_m} {result_s}");
    }

    print!("{output}");
}