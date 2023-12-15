use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut bytes = input.trim().bytes();
    let mut check = bytes.next() == Some(b')');
    let mut answer = 10;

    for b in bytes {
        if (b == b')') ^ check {
            answer += 10;
            check = b == b')';
        } else {
            answer += 5;
        }
    }

    print!("{answer}");
}