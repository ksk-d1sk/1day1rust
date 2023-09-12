use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let s = "WelcomeToSMUPC";
    let n = (get!(usize) - 1) % s.len();

    print!("{}", &s[n..(n + 1)]);
}