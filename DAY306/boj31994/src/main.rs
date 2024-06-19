use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut answer = ("", 0);

    for _ in 0..7 {
        let name = next!();
        let n = next!(u8);

        if n > answer.1 {
            answer = (name, n);
        }
    }

    print!("{}", answer.0);
}