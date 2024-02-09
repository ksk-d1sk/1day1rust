use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let lines = input.lines();
    let mut output = String::new();

    for s in lines.skip(1) {
        let bytes = s.bytes();
        let mut count = 0;
        let mut togle = false;

        for (bit, i) in bytes.zip(1_usize..) {
            if i % 8 == 0 {
                if !togle && bit == b'1' || togle && bit == b'0' {
                    count += 1;
                }

                togle = false;
            } else {
                if bit == b'1' {
                    togle = !togle;
                }
            }
        }

        let _ = writeln!(output, "{count}");
    }

    print!("{output}");
}