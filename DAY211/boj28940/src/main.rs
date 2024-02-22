use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (w, h) = next!(i32, i32);
    let (n, a, b) = next!(i32, i32, i32);
    let x = w / a;
    let y = h / b;

    if x == 0 || y == 0 {
        print!("-1");
    } else {
        print!("{}", (n + x * y - 1) / (x * y));
    }
}