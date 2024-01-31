use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (a, x, b, y, t) = next!(u32, u32, u32, u32, u32);

    print!(
        "{} {}",
        a + t.saturating_sub(30) * x * 21,
        b + t.saturating_sub(45) * y * 21,
    );
}