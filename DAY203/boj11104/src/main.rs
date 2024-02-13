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

    let n = next!(u16);
    for s in (0..n).map(|_| next!()) {
        let _ = writeln!(
            output,
            "{}",
            s.bytes()
                .rev()
                .zip(0..)
                .filter_map(|(bit, i)| (bit == b'1').then_some(2_u32.pow(i)))
                .sum::<u32>()
        );
    }

    print!("{output}");
}