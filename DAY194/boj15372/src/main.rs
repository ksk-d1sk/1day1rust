use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    print!(
        "{}",
        (0..next!(u32))
            .map(|_| format!("{}", next!(u64).pow(2)))
            .collect::<Vec<_>>()
            .join("\n")
    );
}