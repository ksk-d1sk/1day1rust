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
        let v: Vec<_> = (0..10).map(|_| next!(u8)).collect();
        let mack = v.contains(&18);
        let zack = v.contains(&17);

        for elem in v {
            let _ = write!(output, "{elem} ");
        }

        let answer = if mack && zack { "both" } else
                     if mack         { "mack" } else
                     if zack         { "zack" } else
                                     { "none" };

        let _ = writeln!(output, "\n{answer}\n");
    }

    print!("{output}");
}