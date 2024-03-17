use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let m = next!(u16);
    let a = m.min(30) * 10 / 2;
    let b = m.saturating_sub(30) * 15;
    let c = a + b;

    print!("{}.{}", c / 10, c % 10);
}
