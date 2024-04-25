use std::cmp::Ordering::*;
use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    for (r, e, c) in (0..n).map(|_| next!(i32, i32, i32)) {
        let _ = match r.cmp(&(e - c)) {
            Less => writeln!(output, "advertise"),
            Equal => writeln!(output, "does not matter"),
            Greater => writeln!(output, "do not advertise"),
        };
    }

    print!("{output}");
}