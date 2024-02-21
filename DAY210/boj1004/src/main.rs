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

    let t = next!(usize);
    for _ in 0..t {
        let (x1, y1) = next!(i32, i32);
        let (x2, y2) = next!(i32, i32);
        let mut count = 0;

        let n = next!(u8);
        for (cx, cy, r) in (0..n).map(|_| next!(i32, i32, i32)) {
            let check1 = r * r <= (cx - x1).abs().pow(2) + (cy - y1).abs().pow(2);
            let check2 = r * r <= (cx - x2).abs().pow(2) + (cy - y2).abs().pow(2);

            if check1 ^ check2 {
                count += 1;
            }
        }

        let _ = writeln!(output, "{count}");
    }

    print!("{output}");
}