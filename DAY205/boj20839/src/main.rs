use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut answer = 'E';
    let (x, y, z) = next!(u8, u8, u8);
    let (a, b, c) = next!(u8, u8, u8);

    if z == c {
        answer = 'E';
        if y == b {
            answer = 'C';
            if x == a {
                answer = 'A'
            } else if (x + 1) / 2 <= a {
                answer = 'B';
            }
        } else if (y + 1) / 2 <= b {
            answer = 'D';
        }
    }

    print!("{answer}");
}