use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut answer = 0;

    for _ in 0..8 {
        for b in next!().bytes() {
            answer += match b {
                b'P' => 1,
                b'N' => 3,
                b'B' => 3,
                b'R' => 5,
                b'Q' => 9,
                b'p' => -1,
                b'n' => -3,
                b'b' => -3,
                b'r' => -5,
                b'q' => -9,
                _ => 0,
            }
        }
    }

    print!("{answer}");
}