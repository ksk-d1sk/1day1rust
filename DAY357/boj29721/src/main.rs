use std::collections::HashSet;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, k) = next!(usize, usize);
    let mut set = HashSet::new();

    for (x, y) in (0..k).map(|_| next!(usize, usize)) {
        set.insert((x, y));
        for (nx, ny) in [
            (x + 2, y), (x, y + 2),
            (x - 2, y), (x, y - 2),
        ] {
            if nx - 1 < n && ny - 1 < n {
                set.insert((nx, ny));
            }
        }
    }

    print!("{}", set.len() - k);
}