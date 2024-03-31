use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (d1, d2, d3) = next!(i32, i32, i32);

    let a = (d1 + d2 - d3) * 5;
    let b = d1 * 10 - a;
    let c = d2 * 10 - a;

    if a > 0 && b > 0 && c > 0 {
        print!(
            "1\n{}.{} {}.{} {}.{}",
            a / 10,
            a % 10,
            b / 10,
            b % 10,
            c / 10,
            c % 10
        );
    } else {
        print!("-1");
    }
}