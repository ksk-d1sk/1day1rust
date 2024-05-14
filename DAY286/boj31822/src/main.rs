use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let a = &next!()[0..5];
    let n = next!(u8);
    let mut count = 0;

    for b in (0..n).map(|_| &next!()[0..5]) {
        if a == b {
            count += 1;
        }
    }

    print!("{count}");
}