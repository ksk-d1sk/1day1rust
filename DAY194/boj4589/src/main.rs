use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::from("Gnomes:\n");

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u8);
    for (a, b, c) in (0..n).map(|_| next!(u8, u8, u8)) {
        if a < b && b < c || a > b && b > c {
            output.push_str("Ordered\n");
        } else {
            output.push_str("Unordered\n");
        }
    }

    print!("{output}");
}