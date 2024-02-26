use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (h, l) = next!(u8, u8);
    let (a, b) = next!(u8, u8);

    if (h >= (a + 1) / 2 && l >= b) || (h >= (b + 1) / 2 && l >= a) {
        print!("YES");
    } else {
        print!("NO");
    }
}