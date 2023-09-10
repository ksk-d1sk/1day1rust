use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
      () => { input.next().unwrap() };
      ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    for _ in 0..get!(u8) {
        let mut answer = 1;
        let mut map = HashMap::new();
        for _ in 0..get!(u8) {
            get!();
            map.entry(get!()).and_modify(|e| *e += 1).or_insert(1_usize);
        }

        for (_, v) in map {
            answer *= v + 1;
        }

        let _ = writeln!(output, "{}", answer - 1);
    }

    print!("{output}");
}
