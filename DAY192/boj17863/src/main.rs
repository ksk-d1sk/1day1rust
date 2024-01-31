use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    if n / 10000 == 555 {
        print!("YES");
    } else {
        print!("NO");
    }
}