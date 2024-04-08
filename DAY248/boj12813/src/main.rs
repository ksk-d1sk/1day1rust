use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
    }

    let a: Vec<_> = next!().bytes().map(|b| b == b'1').collect();
    let b: Vec<_> = next!().bytes().map(|b| b == b'1').collect();
    let trans = |b: bool| if b { '1' } else { '0' };

    let _ = writeln!(output, "{}", a.iter().zip(b.iter()).map(|(l, r)| trans(l & r)).collect::<String>());
    let _ = writeln!(output, "{}", a.iter().zip(b.iter()).map(|(l, r)| trans(l | r)).collect::<String>());
    let _ = writeln!(output, "{}", a.iter().zip(b.iter()).map(|(l, r)| trans(l ^ r)).collect::<String>());
    let _ = writeln!(output, "{}", a.iter().map(|a| trans(!a)).collect::<String>());
    let _ = writeln!(output, "{}", b.iter().map(|b| trans(!b)).collect::<String>());

    print!("{output}");
}