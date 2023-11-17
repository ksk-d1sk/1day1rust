use std::collections::HashSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    for (a, b, c) in (0_u32..).map(|_| get!(u16, u16, u16)) {
        if a + b + c == 0 { break }

        let mut v = vec![a, b, c];
        v.sort_unstable();

        let _ = writeln!(
            output,
            "{}",
            if v[0] + v[1] > v[2] {
                let set: HashSet<_> = v.into_iter().collect();
                match set.len() {
                    1 => "Equilateral",
                    2 => "Isosceles",
                    3 => "Scalene",
                    _ => unreachable!(),
                }
            } else {
                "Invalid"
            }
        );
    }

    print!("{output}");
}
