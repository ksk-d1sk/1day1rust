use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut v: Vec<u8> = (1..=20).collect();

    for (a, b) in (0..10).map(|_| get!(usize, usize)) {
        v[(a - 1)..b].reverse();
    }

    for elem in v {
        print!("{elem} ");
    }
}
