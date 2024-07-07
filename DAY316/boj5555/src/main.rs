use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let s = next!();
    let n = next!(u8);
    let mut answer = 0_u8;

    for ns in (0..n).map(|_| next!()) {
        if format!("{ns}{ns}").contains(s) {
            answer += 1;
        }
    }

    print!("{answer}");
}