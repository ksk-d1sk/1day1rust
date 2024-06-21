use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (c, k, p) = next!(u32, u32, u32);
    let mut answer = 0;

    for i in 1..=c {
        answer += i * k + i * i * p;
    }

    print!("{answer}");
}