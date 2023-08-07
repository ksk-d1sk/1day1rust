use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace();

    let mut v = [0; 10];

    for i in input.flat_map(str::parse::<usize>) {
        v[i] += 1;
    }

    print!(
        "{}",
        v.iter().enumerate().filter(|e| *e.1 & 1 == 1).next().unwrap().0
    )
}
