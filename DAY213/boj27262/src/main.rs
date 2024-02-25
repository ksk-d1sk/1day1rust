use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().trim().parse::<$t>().unwrap()),+) };
    }

    let (n, k, a, b) = next!(u32, u32, u32, u32); 

    print!("{} {}", (k + n - 2) * b, (n - 1) * a);
}