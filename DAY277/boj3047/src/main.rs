use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut arr = Vec::from_iter((0..3).map(|_| next!(u8)));

    arr.sort_unstable();

    for i in next!().bytes().map(|b| (b - b'A') as usize) {
        print!("{} ", arr[i]);
    }
}