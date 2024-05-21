use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, u8);
    let score = Vec::from_iter((0..n).map(|_| next!(u16)));
    let mut v: Vec<_> = (0..m).map(|_| (
            next!(u32),
            (0..n).map(|i| (next!() == "O").then_some(score[i]).unwrap_or_default()).sum::<u16>()
        ))
        .collect();

    v.sort_unstable_by(|a, b| {
        if a.1 == b.1 {
            a.0.cmp(&b.0)
        } else {
            b.1.cmp(&a.1)
        }
    });

    print!("{} {}", v[0].0, v[0].1);
}