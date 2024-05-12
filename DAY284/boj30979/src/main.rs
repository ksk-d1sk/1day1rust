use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(u32);
    let n = next!(u8);

    if t > (0..n).map(|_| next!(u32)).sum() {
        print!("Padaeng_i Cry");
    } else {
        print!("Padaeng_i Happy");
    }
}