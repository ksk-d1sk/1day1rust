use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let v = Vec::from_iter((0..n).map(|_| next!(u32)));
    let mut answer = v[0];

    for window in v.windows(2) {
        if window[1] - window[0] != 1 {
            answer += window[1];
        }
    }

    print!("{answer}");
}