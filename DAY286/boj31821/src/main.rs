use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u8);
    let arr = Vec::from_iter((0..n).map(|_| next!(u32)));
    let m = next!(u8);

    print!("{}", (0..m).map(|_| arr[next!(usize) - 1]).sum::<u32>());
}