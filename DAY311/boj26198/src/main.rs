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

    let t = next!(u16);
    for line in (0..t).map(|_| next!()) {
        let mut answer = 0;

        for c in line.bytes() {
            match c {
                b'I' => answer += 1,
                b'V' => answer += 5,
                b'X' => answer += 10,
                b'L' => answer += 50,
                b'C' => answer += 100,
                b'D' => answer += 500,
                b'M' => answer += 1000,
                _ => {},
            }
        }

        let _ = writeln!(output, "{answer}");
    }

    print!("{output}");
}