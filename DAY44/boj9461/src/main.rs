use std::io;
use std::fmt::Write;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let mut p = [0_u64; 101];

    p[1] = 1;
    p[2] = 1;

    for i in  3..=100 {
        p[i] = p[i - 2] + p[i - 3];
    }

    for n in input.skip(1).flat_map(str::parse::<usize>) {
        let _ = writeln!(output, "{}", p[n]);
    }

    print!("{output}");
}
