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
    for _ in 0..t {
        let n = next!(u8);
        let mut name = "";
        let mut max_l = 0;

        for (nm, l) in (0..n).map(|_| (next!(), next!(u32))) {
            if l > max_l {
                name = nm;
                max_l = l;
            }
        }

        let _ = writeln!(output, "{name}");
    }

    print!("{output}");
}