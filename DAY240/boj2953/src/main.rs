use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut answer = (0, 0);

    for i in 1..=5 {
        let (a, b, c, d) = next!(u8, u8, u8, u8);
        let sum = a + b + c + d;
        if sum > answer.1 {
            answer.0 = i;
            answer.1 = sum;
        }
    }

    print!("{} {}", answer.0, answer.1);
}