use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (x1, y1, r1) = next!(i64, i64, i64);
    let (x2, y2, r2) = next!(i64, i64, i64);

    if (x1 - x2).abs().pow(2) + (y1 - y2).abs().pow(2) < (r1 + r2).pow(2) {
        print!("YES");
    } else {
        print!("NO");
    }
}