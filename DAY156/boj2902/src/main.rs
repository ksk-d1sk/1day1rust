use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();
    let mut check = true;

    for c in input.chars() {
        if check {
            check = false;
            output.push(c);
        } else if c == '-' {
            check = true;
        }
    }

    print!("{output}");
}
