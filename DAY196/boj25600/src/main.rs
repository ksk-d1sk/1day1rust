use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let  n = next!(u16);
    let mut max = 0;

    for (a, d, g) in (0..n).map(|_| next!(u32, u32, u32)) {
        max = max.max(
            a * (d + g) * if a == d + g { 2 } else { 1 }
        );
    }

    print!("{max}");
}