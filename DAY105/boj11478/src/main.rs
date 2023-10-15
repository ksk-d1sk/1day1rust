use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next().unwrap() };
    }

    let s = get!();
    let mut sum = 0;
    let mut set = HashSet::with_capacity(s.len());

    for i in 0..=s.len() {
        for j in 0..(s.len() - i) {
            set.insert(&s[j..=(j + i)]);
        }

        sum += set.len();
        set.clear();
    }

    print!("{sum}");
}