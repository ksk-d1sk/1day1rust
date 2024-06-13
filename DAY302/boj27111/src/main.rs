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
    let mut answer = 0;
    let mut set = HashSet::new();

    for (a, b) in (0..n).map(|_| next!(u32, u8)) {
        if b == 1 {
            if !set.insert(a) {
                answer += 1;
            }
        } else {
            if !set.remove(&a) {
                answer += 1;
            }
        }
    }

    answer += set.len();

    print!("{answer}");
}