use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(u64, u64);

    if n == 1 || m == 1 {
        print!("{}", n * m);
    } else {
        print!("{}", (n + m - 2) * 2);
    }
}