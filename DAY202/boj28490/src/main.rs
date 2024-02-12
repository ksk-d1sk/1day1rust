use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut max = 0;

    for (h, w) in (0..n).map(|_| next!(u32, u32)) {
        max = max.max(h * w);
    }

    print!("{max}");
}