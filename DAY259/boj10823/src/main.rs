use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut answer = 0;
    let mut number = 0;

    for b in input.bytes() {
        match b {
            b'0'..=b'9' => {
                number = number * 10 + (b - b'0') as u32;
            }
            b',' => {
                answer += number;
                number = 0;
            }
            _ => {},
        }
    }

    answer += number;

    print!("{answer}");
}