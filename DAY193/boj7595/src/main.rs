use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let tokens = input.split_ascii_whitespace().flat_map(str::parse::<u8>);
    let mut output = String::new();

    for n in tokens.take_while(|&n| n != 0) {
        for i in 0..n {
            for _ in 0..=i {
                output.push('*');
            }
            output.push('\n');
        }
    }

    print!("{output}");
}