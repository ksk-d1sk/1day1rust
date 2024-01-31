use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (a1, p1) = next!(f64, f64);
    let (r1, p2) = next!(f64, f64);

    if r1 * r1 * std::f64::consts::PI * p1 > a1 * p2 {
        print!("Whole pizza");
    } else {
        print!("Slice of pizza");
    }
}