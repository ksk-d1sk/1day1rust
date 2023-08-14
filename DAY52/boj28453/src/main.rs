use std::io;
use std::fmt::Write;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    for i in input.skip(1).flat_map(str::parse::<u16>) {
        let _ = write!(
            output, "{} ",
            if      i == 300 { 1 }
            else if i >= 275 { 2 }
            else if i >= 250 { 3 }
            else             { 4 }
        );
    }

    print!("{output}");
}
