use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.lines();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(usize);
    for _ in 0..t {
        let _ = writeln!(output, "{}", next!().split_ascii_whitespace().flat_map(str::parse::<u32>).sum::<u32>());
    }

    print!("{output}");
}