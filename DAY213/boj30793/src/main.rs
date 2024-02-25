use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let p = next!(f64);
    let r = next!(f64);
    let v = p / r;

    if v < 0.2 {
        print!("weak");
    } else if v < 0.4 {
        print!("normal");
    } else if v < 0.6 {
        print!("strong");
    } else {
        print!("very strong");
    }
}