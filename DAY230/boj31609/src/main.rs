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

    let n = next!(u8);
    let mut arr = [false; 10];

    for i in (0..n).map(|_| next!(usize)) {
        arr[i] = true;
    }

    for i in arr
        .into_iter()
        .enumerate()
        .filter_map(|(i, x)| x.then_some(i))
    {
        let _ = writeln!(output, "{i}");
    }

    print!("{output}");
}