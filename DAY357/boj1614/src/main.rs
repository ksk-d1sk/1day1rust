use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (i, c) = next!(u64, u64);

    match i {
        1 | 5 => print!("{}", 8 * c + i - 1),
        _ => if c % 2 == 0 {
            print!("{}", c / 2 * 8 + i - 1);
        } else {
            print!("{}", c / 2 * 8 + 9 - i);
        },
    }
}