use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (x, y) = next!(i32, i32);
    let a = 100 - x;
    let b = 100 - y;
    let c = 100 - a - b;
    let d = a * b;
    let q = d / 100;
    let r = d % 100;

    print!("{a} {b} {c} {d} {q} {r}\n{} {}", c + q, r);
}