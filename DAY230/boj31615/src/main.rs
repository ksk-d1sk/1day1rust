use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (a, b) = next!(u16, u16);
    let mut c = a + b;
    let mut digit = 0;

    while c > 0 {
        digit += 1;
        c /= 10;
    }

    print!("{digit}");
}
