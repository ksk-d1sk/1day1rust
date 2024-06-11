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

    let t = next!(usize);
    for (c, v) in (0..t).map(|_| next!(u16, u16)) {
        let _ = writeln!(output, "You get {} piece(s) and your dad gets {} piece(s).", c / v, c % v);
    }

    print!("{output}");
}