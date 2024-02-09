use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let lines = input.lines();
    let mut output = String::new();

    for line in lines.skip(1) {
        let mut count = 0_usize;

        for b in line.bytes() {
            match (b, count % 3) {
                (b'p' | b'P', 0) => count += 1,
                (b'l' | b'L', 1) => count += 1,
                (b'u' | b'U', 2) => count += 1,
                _ => {},
            }
        }

        let _ = writeln!(output, "{}", count / 3);
    }

    print!("{output}");
}