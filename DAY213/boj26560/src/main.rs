use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.lines();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    for s in (0..n).map(|_| next!()) {
        if s.as_bytes()[s.len() - 1] == b'.' {
            println!("{s}");
        } else {
            println!("{s}.");
        }
    }
}