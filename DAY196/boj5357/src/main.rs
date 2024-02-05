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
    for s in (0..t).map(|_| next!()) {
        let mut v = s.chars().collect::<Vec<_>>();

        v.dedup();

        let _ = writeln!(
            output,
            "{}",
            &v.into_iter().collect::<String>()
        );
    }

    print!("{output}");
}