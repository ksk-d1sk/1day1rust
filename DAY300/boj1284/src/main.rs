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

    for n in (0..).map(|_| next!()) {
        if n == "0" {
            break;
        }

        let mut answer = 2;
        let arr = n.as_bytes();

        answer += arr.len() - 1;

        for &b in arr {
            if b == b'1' {
                answer += 2;
            } else if b == b'0' {
                answer += 4;
            } else {
                answer += 3;
            }
        }

        let _ = writeln!(output, "{answer}");
    }

    print!("{output}");
}