use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, mut l) = next!(u16, u32);
    let mut v = Vec::from_iter((0..n).map(|_| next!(u32)));

    v.sort_unstable();

    for h in v {
        if l < h {
            break;
        }

        l += 1;
    }

    print!("{l}");
}