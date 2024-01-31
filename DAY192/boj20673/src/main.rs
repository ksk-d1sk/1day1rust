use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (p, q) = next!(i16, i16);

    if p <= 50 && q <= 10 {
        print!("White");
    } else if q > 30 {
        print!("Red");
    } else {
        print!("Yellow");
    }
}