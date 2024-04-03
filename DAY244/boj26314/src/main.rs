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

    for name in (0..n).map(|_| next!()) {
        let count = name
            .chars()
            .filter(|b| matches!(b, 'a' | 'e' | 'i' | 'o' | 'u'))
            .count();

        let _ = writeln!(
            output,
            "{name}\n{}",
            (count > name.len() / 2) as u8
        );
    }

    print!("{output}");
}