use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, u64);
    let mut v = Vec::from_iter((0..n).map(|_| next!(u64)));
    let total = v.iter().sum::<u64>();
    let mut sum = 0;
    let mut cnt = 1;

    v.sort_unstable_by(|a, b| b.cmp(a));

    for x in v {
        cnt += 1;
        sum += x;

        if sum >= (total + 1) / 2 {
            break;
        }
    }

    print!("{}", m / cnt);
}