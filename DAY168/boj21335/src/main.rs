use std::{io, f64::consts::PI};

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let a = get!(f64);

    print!("{}", (a / PI).sqrt() * PI * 2.0);
}