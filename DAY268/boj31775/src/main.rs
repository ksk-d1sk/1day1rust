use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut l = false;
    let mut k = false;
    let mut p = false;

    for c in (0..3).map(|_| next!().as_bytes()[0]) {
        match c {
            b'l' => l = true,
            b'k' => k = true,
            b'p' => p = true,
            _ => {},
        }
    }

    if l && k && p {
        print!("GLOBAL");
    } else {
        print!("PONIX");
    }
}