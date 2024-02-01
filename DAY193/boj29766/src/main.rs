use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let s = next!();
    let mut i = 0;
    let mut count = 0;
    let dksh = [b'D', b'K', b'S', b'H'];

    for b in s.bytes() {
        if b == dksh[i] {
            if i == 3 {
                count += 1;
                i = 0;
            } else {
                i += 1;
            }
        } else if b == b'D' {
            i = 1;
        } else {
            i = 0;
        }
    }

    print!("{count}");
}