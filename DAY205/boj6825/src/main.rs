use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let weight = next!(f64);
    let height = next!(f64);

    match (weight / (height * height) * 10.0) as u32 {
        250.. => print!("Overweight"),
        185.. => print!("Normal weight"),
        _ => print!("Underweight"),
    }
}