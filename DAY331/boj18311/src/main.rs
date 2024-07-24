use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, mut k) = next!(usize, i64);
    let mut v = vec![0];
    let mut answer = None;

    v.extend((0..n).map(|_| next!(i64)));

    for i in 0..=n {
        k -= v[i];
        if answer.is_none() && k < 0 {
            answer = Some(i);
        }
    }

    for i in (0..=n).rev() {
        k -= v[i];
        if answer.is_none() && k < 0 {
            answer = Some(i);
        }
    }

    print!("{}", answer.unwrap());
}