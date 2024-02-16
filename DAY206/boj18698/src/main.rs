use std::fmt::Write;
use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(u8);
    for s in (0..t).map(|_| next!()) {
        let mut count = 0;
        s.bytes().any(|b| b == b'D' || {count += 1; false});
        let _ = writeln!(output, "{count}");
    }

    print!("{output}");
}