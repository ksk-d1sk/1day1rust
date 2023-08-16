use std::io;
use std::cmp::Ordering::*;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse);

    macro_rules! get {
        () => { input.next().unwrap() };
    }

    let a = sum_time(get!(), get!(), get!());
    let b = sum_time(get!(), get!(), get!());

    print!(
        "{}",
        match a.cmp(&b) {
            Greater => "Max",
            Equal   => "Draw",
            Less    => "Mel",
        }
    );
}

fn sum_time(a: u16, b: u16, c: u16) -> u16 {
    a * 3 + b * 20 + c * 120
}
