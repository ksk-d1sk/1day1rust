use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut two = 0;
    let mut e = 0;

    next!();

    for b in next!().bytes() {
        if b == b'2' {
            two += 1;
        } else {
            e += 1;
        }
    }

    if two > e {
        print!("2");
    } else if two < e {
        print!("e");
    } else {
        print!("yee");
    }
}