use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, t) = next!(u8, u64);
    let mut answer = u64::MAX;

    for (s, i, c) in (0..n).map(|_| next!(u64, u64, u64)) {
        for j in 0..c {
            if let Some(x) = (s + i * j).checked_sub(t) {
                answer = answer.min(x);
            }
        }
    }

    if answer == u64::MAX {
        print!("-1");
    } else {
        print!("{answer}");
    }
}