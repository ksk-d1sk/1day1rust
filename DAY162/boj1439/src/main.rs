use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.trim();

    let a = input.split("1").filter(|s| !s.is_empty()).count();
    let b = input.split("0").filter(|s| !s.is_empty()).count();

    print!("{}", a.min(b));
}
