use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut answer = 0;
    let mut people = 0;

    for (a, b) in (0..10).map(|_| next!(u16, u16)) {
        people = people + b - a;
        answer = answer.max(people);
    }

    print!("{answer}");
}