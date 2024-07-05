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

    let m = next!(u32);
    let mut sum = 0;
    let mut xor = 0;

    for _ in 0..m {
        let q = next!();
        let _ = match q {
            "1" => {
                let x = next!(u64);
                sum += x;
                xor ^= x;
                Ok(())
            }
            "2" => {
                let x = next!(u64);
                sum -= x;
                xor ^= x;
                Ok(())
            }
            "3" => {
                writeln!(output, "{sum}")
            }
            "4" => {
                writeln!(output, "{xor}")
            }
            _ => unreachable!(),
        };
    }

    print!("{output}");
}