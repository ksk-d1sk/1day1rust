use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut a = next!(u64);
    let mut b = next!(u64);
    let mut i = 0;
    let mut answer = 0;

    while a != 0 || b != 0 {
        answer += (a % 10).max(b % 10) * 10_u64.pow(i);
        a /= 10;
        b /= 10;
        i += 1;
    }

    print!("{answer}");
}