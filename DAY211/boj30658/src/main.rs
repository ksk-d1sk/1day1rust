use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().trim().parse::<$t>().unwrap()),+) };
    }

    let mut arr = [0; 100];
    loop {
        let n = next!(usize);
        if n == 0 { break }

        for (i, a) in (0..n).map(|i| (i, next!(u8))) {
            arr[i] = a;
        }

        for i in (0..n).rev() {
            let _ = writeln!(output, "{}", arr[i]);
        }
        let _ = writeln!(output, "0");
    }

    print!("{output}");
}