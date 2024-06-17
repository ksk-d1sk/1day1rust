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
        let mut sum = 0;
        let mut arr = [true; 26];

        for i in s.bytes().map(|b| (b - b'A') as usize) {
            arr[i] = false;
        }

        for (i, x) in (0..26).map(|i| (i, i + 65)) {
            sum += x * arr[i] as usize;
        }

        let _ = writeln!(output, "{sum}");
    }

    print!("{output}");
}