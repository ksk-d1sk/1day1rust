use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next().unwrap() };
    }

    let n = get!();
    let mut count = [0; 9];
    let mut count69 = 0;

    for d in n.as_bytes().iter() {
        match d {
            b'6' | b'9' => count69 += 1,
            _ => count[(d - b'0') as usize] += 1,
        }
    }

    print!("{}", count.iter().chain(&[(count69 + 1) / 2]).max().unwrap());
}
