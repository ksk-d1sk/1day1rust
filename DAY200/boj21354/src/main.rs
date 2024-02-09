use std::cmp::Ordering::*;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (a, b) = next!(u32, u32);
    match (a * 7).cmp(&(b * 13)) {
        Greater => print!("Axel"),
        Equal => print!("lika"),
        Less => print!("Petra"),
    }
}