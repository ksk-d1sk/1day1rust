use std::collections::BinaryHeap;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u8);
    let mut first = next!(u32);
    let mut heap = BinaryHeap::from_iter((1..n).map(|_| next!(u32)));
    let mut answer = 0;

    while let Some(x) = heap.pop() {
        if first > x {
            break;
        }

        first += 1;
        answer += 1;

        heap.push(x - 1);
    }

    print!("{answer}");
}