use std::collections::HashSet;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u16);
    let set: HashSet<_> = HashSet::from_iter((0..n).map(|_| next!()));
    let m = next!(u16);
    let mut answer = 0;

    for email in (0..m).map(|_| next!()) {
        if set.contains(email) {
            answer += 1;
        }
    }

    print!("{answer}");
}