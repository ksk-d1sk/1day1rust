use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let x = next!();
    let y = next!();

    match x.parse::<i32>() {
        Ok(a) => match y.parse::<i32>() {
            Ok(b) => print!("{}", a - b),
            Err(_) => print!("NaN"),
        },
        Err(_) => print!("NaN"),
    }
}