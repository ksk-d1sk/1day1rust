use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut v = Vec::from_iter((0..n).map(|_| (next!(u64), 0)));

    for i in 0..n {
        v[i].1 = next!(u64);
    }

    v.sort_unstable_by_key(|k| k.1);

    print!(
        "{}",
        v.into_iter()
            .zip(0..)
            .map(|(x, i)| x.0 + x.1 * i)
            .sum::<u64>()
    );
}