use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (_, mut x, k) = next!(u32, u32, u32);
    for (a, b) in (0..k).map(|_| next!(u32, u32)) {
        if a == x {
            x = b;
        } else if b == x {
            x = a;
        }
    }

    print!("{x}");
}