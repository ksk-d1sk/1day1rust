use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(u8);
    for (a, b) in (0..t).map(|_| (next!(), next!())) {
        if a == b {
            output.push_str("OK\n");
        } else {
            output.push_str("ERROR\n");
        }
    }

    print!("{output}");
}