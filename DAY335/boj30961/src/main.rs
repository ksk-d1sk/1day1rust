use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u32);
    let mut answer = 0;
    let mut v: Vec<_> = (0..n)
        .map(|_| {
            let temp = next!(u64);
            answer ^= temp * temp;
            temp
        })
        .collect();

    v.sort_unstable();

    for window in v.windows(2) {
        answer ^= window[0] * window[1];
    }

    print!("{answer}");
}