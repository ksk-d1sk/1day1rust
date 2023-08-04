use std::io;
use std::fmt::Write;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    for mut c in input.skip(1).flat_map(str::parse::<u16>) {
        let _ = writeln!(
            output,
            "{} {} {} {}",
            {let tmp = c / 25; c %= 25; tmp},
            {let tmp = c / 10; c %= 10; tmp},
            {let tmp = c / 5; c %= 5; tmp},
            c / 1,
        );
    }

    print!("{output}");
}