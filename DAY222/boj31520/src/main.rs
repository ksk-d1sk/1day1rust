use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!().as_bytes();
    let mut check = true;

    for (i, b) in (0..n.len()).zip(b'1'..) {
        if n[i] != b {
            check = false;
            break;
        }
    }

    if check {
        print!("{}", n.len());
    } else {
        print!("-1");
    }
}