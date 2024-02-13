use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let lines = input.lines();
    let mut output = String::new();

    for line in lines.skip(1) {
        let mut jaum = 0;
        let mut moum = 0;

        for c in line.chars() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' |
                'A' | 'E' | 'I' | 'O' | 'U' => moum += 1,
                ' ' => {},
                _ => jaum += 1,
            }
        }

        let _ = writeln!(output, "{jaum} {moum}");
    }

    print!("{output}");
}