use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u8);

    print!(
        "{}",
        (0..n)
            .map(|_| next!(u32, u8))
            .map(|(w, _)| match w {
                136 => 1000,
                142 => 5000,
                148 => 10000,
                154 => 50000,
                _ => unreachable!(),
            })
            .sum::<u32>()
    );
}