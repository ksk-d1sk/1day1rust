use std::cmp::Ordering::*;
use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut input = input.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(usize);

    print!(
        "{}",
        match (0..n).map(|_| get!(i16)).sum::<i16>().cmp(&0) {
            Greater => "Right",
            Equal => "Stay",
            Less => "Left",
        }
    );
}
