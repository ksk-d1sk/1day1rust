use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().trim().parse::<$t>().unwrap()),+) };
    }

    let s = next!();
    let mut arr = [true; 26];

    for b in s.bytes() {
        arr[(b - b'A') as usize] = false;
    }

    for (i, b) in arr.into_iter().enumerate() {
        if b {
            print!("{}", (i as u8 + b'A') as char);
        }
    }
}