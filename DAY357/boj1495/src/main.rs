use std::collections::HashSet;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, s, m) = next!(usize, i32, i32);
    let mut set = HashSet::from([s]);

    for x in (0..n).map(|_| next!(i32)) {
        let mut new = HashSet::new();

        for p in set.iter() {
            if p + x <= m {
                new.insert(p + x);
            }

            if p - x >= 0 {
                new.insert(p - x);
            }
        }

        set = new;
    }

    print!("{}", set.iter().max().unwrap_or(&-1));
}