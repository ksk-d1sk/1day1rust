use std::cmp::Ordering::*;
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

    let t = next!(u16);
    for _ in 0..t {
        let n = next!(u8);
        let mut a = 0;
        let mut b = 0;

        for (l, r) in (0..n).map(|_| (next!(), next!())) {
            match l {
                "R" => match r {
                    "P" => b += 1,
                    "S" => a += 1,
                    _ => {},
                }
                "P" => match r {
                    "R" => a += 1,
                    "S" => b += 1,
                    _ => {},
                }
                "S" => match r {
                    "R" => b += 1,
                    "P" => a += 1,
                    _ => {},
                }
                _ => unreachable!(),
            }
        }

        let _ = writeln!(
            output,
            "{}",
            match a.cmp(&b) {
                Greater => "Player 1",
                Equal => "TIE",
                Less => "Player 2",
            }
        );
    }

    print!("{output}");
}