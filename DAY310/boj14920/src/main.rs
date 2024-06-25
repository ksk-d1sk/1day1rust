use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut n = next!(usize);
    let mut answer = 1_usize;

    while n != 1 {
        answer += 1;
        if n & 1 == 1 {
            n = n * 3 + 1;
        } else {
            n /= 2;
        }
    }

    print!("{answer}");
}