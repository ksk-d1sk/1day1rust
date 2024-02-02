use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (r, c) = next!(usize, usize);

    print!(
        "{}",
        (0..r).map(|_| (0..c).map(|_| '*').collect::<String>()).collect::<Vec<_>>().join("\n")
    );
}