use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m, k) = next!(u32, u32, u32);
    let (a, b, c) = next!(u32, u32, u32);
    let mut v = [(a * n, "Joffrey"), (b * m, "Robb"), (c * k, "Stannis")];

    v.sort_unstable_by(|a, b| {
        if a.0 != b.0 {
            b.0.cmp(&a.0)
        } else {
            a.1.cmp(b.1)
        }
    });

    print!("{} ", v[0].1);

    if v[0].0 == v[1].0 {
        print!("{} ", v[1].1);
    }

    if v[0].0 == v[2].0 {
        print!("{}", v[2].1);
    }
}