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

    for _ in 0..n {
        let v: Vec<_> = (0..3).map(|_| next!(u8)).collect();
        let count = v.iter().filter(|x| **x >= 10).count();
        let answer = match count {
            1 => "double",
            2 => "double-double",
            3 => "triple-double",
            _ => "zilch",
        };

        for elem in v {
            let _ = write!(output, "{elem} ");
        }

        let _ = writeln!(output, "\n{answer}\n");
    }

    print!("{output}");
}