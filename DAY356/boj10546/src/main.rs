use std::collections::HashSet;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut set = HashSet::new();

    for name in (0..n*2-1).map(|_| next!()) {
        if !set.insert(name) {
            set.remove(name);
        }
    }

    print!("{}", set.iter().next().unwrap());
}