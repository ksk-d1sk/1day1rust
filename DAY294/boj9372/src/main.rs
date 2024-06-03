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
    for _ in 0..t {
        let (n, m) = next!(u16, u16);
        let _ = (0..m).map(|_| {next!(); next!()}).all(|_| true);
        let _ = writeln!(output, "{}", n - 1);
    }

    print!("{output}");
}