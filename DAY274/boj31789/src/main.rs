use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u8);
    let (x, s) = next!(u32, u32);

    if (0..n).map(|_| next!(u32, u32)).any(|(c, p)| x >= c && p > s) {
        print!("YES");
    } else {
        print!("NO");
    }
}