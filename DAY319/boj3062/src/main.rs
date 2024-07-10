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
    for n in (0..t).map(|_| next!()) {
        let a: u32 = n.parse().unwrap();
        let b: u32 = n.chars().rev().collect::<String>().parse().unwrap();
        let c = (a + b).to_string();
        let v = c.as_bytes();
        let mut i = 0;
        let mut j = v.len() - 1;
        let mut check = true;

        while i < j {
            if v[i] != v[j] {
                check = false;
                break;
            }
            i += 1;
            j -= 1;
        }

        let _ = if check {
            writeln!(output, "YES")
        } else {
            writeln!(output, "NO")
        };
    }

    print!("{output}");
}