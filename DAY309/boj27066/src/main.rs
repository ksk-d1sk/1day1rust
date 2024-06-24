use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut i = n;
    let mut v = Vec::from_iter((0..n).map(|_| next!(f64)));
    let mut j = 1;

    v.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

    let mut answer = v[i / 2];

    while i > 1 {
        i -= 1;
        j += 1;
        v[i - 1] = v[i - 1] + v[i];

        if i / 2 < n - j {
            answer = answer.max(v[i / 2]);
        } else {
            answer = answer.max(v[i / 2] / j as f64);
        }
    }

    print!("{answer}");
}