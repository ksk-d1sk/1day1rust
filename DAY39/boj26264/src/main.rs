use std::io::Read;
use std::cmp::Ordering::*;

fn main() {
    // let mut f = std::fs::File::open("input.txt").unwrap();
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    // f.read_to_string(&mut input).unwrap();

    let str = input.split_ascii_whitespace().skip(1).next().unwrap().as_bytes();
    let mut count_b = 0;
    let mut count_s = 0;
    let mut i = 0;

    while let Some(c) = str.get(i) {
        if *c == b'b' {
            count_b += 1;
            i += 7;
        } else {
            count_s += 1;
            i += 8;
        }
    }

    print!(
        "{}",
        match count_b.cmp(&count_s) {
            Greater => "bigdata?",
            Equal => "bigdata? security!",
            Less => "security!"
        }
    )
}
