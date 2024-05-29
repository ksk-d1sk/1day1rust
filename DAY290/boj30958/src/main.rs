use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut alphabet = [0; 26];

    for b in input.bytes() {
        if matches!(b, b'a'..=b'z') {
            alphabet[(b - b'a') as usize] += 1;
        }
    }

    print!("{}", alphabet.iter().max().unwrap());
}