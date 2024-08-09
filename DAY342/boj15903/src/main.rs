use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(u16, u16);
    let mut pq = BinaryHeap::from_iter((0..n).map(|_| Reverse(next!(u64))));

    for _ in 0..m {
        let x = pq.pop().unwrap().0;
        let y = pq.pop().unwrap().0;
        let sum = Reverse(x + y);
        pq.push(sum);
        pq.push(sum);
    }

    print!("{}", pq.iter().map(|x| x.0).sum::<u64>());
}