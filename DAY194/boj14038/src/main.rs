use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let tokens = input.split_ascii_whitespace();

    print!(
        "{}",
        match tokens.filter(|&game| game == "W").count() {
            5.. => 1,
            3.. => 2,
            1.. => 3,
            _   => -1,
        }
    );
}