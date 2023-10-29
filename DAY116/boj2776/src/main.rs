use std::collections::HashSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    for _ in 0..get!(usize) {
        let set: HashSet<_> = (0..get!(u32)).map(|_| get!(i32)).collect();
        for _ in 0..get!(u32) {
            let _ = writeln!(output, "{}", u8::from(set.contains(&get!(i32))));
        }
    }

    print!("{output}");
}
