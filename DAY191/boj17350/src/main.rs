use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let check = input.split_ascii_whitespace().skip(1).any(|s| s == "anj");

    if check {
        print!("뭐야;");
    } else {
        print!("뭐야?");
    }
}