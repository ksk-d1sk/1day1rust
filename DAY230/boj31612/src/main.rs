use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut answer = 0;

    next!();
    for b in next!().bytes() {
        answer += match b {
            b'j' | b'i' => 2,
            b'o' => 1,
            _ => unreachable!(),
        };
    }

    print!("{answer}");
}