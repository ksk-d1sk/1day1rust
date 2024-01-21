use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next().unwrap() };
    }

    let mut v = [0_i16; 26];
    let a = get!().bytes();
    let b = get!().bytes();

    for c in a {
        v[(c - b'a') as usize] += 1;
    }

    for c in b {
        v[(c - b'a') as usize] -= 1;
    }

    print!(
        "{}",
        v.into_iter().fold(0, |acc, x| acc + x.abs())
    );
}