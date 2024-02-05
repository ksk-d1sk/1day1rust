use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (a, b, c) = next!(i32, i32, i32);
    let mut temp = 1;

    while temp <= b {
        temp *= 10;
    }

    print!(
        "{}\n{}",
        a + b - c,
        a * temp as i32 + b - c,
    );
}