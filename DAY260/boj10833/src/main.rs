use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u8);
    let mut answer = 0;

    for (a, b) in (0..n).map(|_| next!(u32, u32)) {
        answer += b % a;
    }

    print!("{answer}");
}