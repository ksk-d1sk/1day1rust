use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(u16);

    print!("{}", (0..t).map(|_| next!(u64, u64)).map(|(a, b)| (a / b).pow(2).to_string()).collect::<Vec<_>>().join("\n"));
}