use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let s = next!();
    let mut arr = [false; 26];
    let mut mobis = [true; 26];

    mobis[(b'M' - b'A') as usize] = false;
    mobis[(b'O' - b'A') as usize] = false;
    mobis[(b'B' - b'A') as usize] = false;
    mobis[(b'I' - b'A') as usize] = false;
    mobis[(b'S' - b'A') as usize] = false;

    for i in s.bytes().map(|x| (x - b'A') as usize) {
        arr[i] = true;
    }

    if arr.iter().zip(mobis).all(|(&a, b)| a || b) {
        print!("YES");
    } else {
        print!("NO");
    }
}