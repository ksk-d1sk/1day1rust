use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut v = vec![false, true, false, false];

    for b in next!().bytes() {
        match b {
            b'A' => (v[1], v[2]) = (v[2], v[1]),
            b'B' => (v[2], v[3]) = (v[3], v[2]),
            b'C' => (v[1], v[3]) = (v[3], v[1]),
            _ => unreachable!(),
        }
    }

    print!("{}", v.iter().enumerate().filter_map(|(i, b)| b.then_some(i)).next().unwrap());
}