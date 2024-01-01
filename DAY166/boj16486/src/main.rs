use std::f64::consts::PI;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    // let buf = include_str!("input.txt");
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (d1, d2) = get!(f64, f64);
    
    println!("{}", d1 * 2.0 + d2 * 2.0 * PI);
}
