use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let s = next!();
    let arr = [b'K', b'O', b'R', b'E', b'A'];
    let mut i = 0;

    for b in s.bytes() {
        if arr[i % 5] == b {
            i += 1;
        }
    }

    print!("{}", i);
}