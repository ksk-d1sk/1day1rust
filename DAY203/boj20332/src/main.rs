use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u32);
    let sum: u32 = (0..n).map(|_| next!(u32)).sum();

    if sum % 3 == 0 {
        print!("yes");
    } else {
        print!("no");
    }
}