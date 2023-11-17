use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::io;

fn main() {
    let mut heap: BinaryHeap<_> = io::read_to_string(io::stdin()).unwrap()
        .split_ascii_whitespace()
        .flat_map(str::parse::<u16>)
        .map(|e| Reverse(e))
        .collect();
    
    let a = heap.pop().unwrap().0;
    let b = heap.pop().unwrap().0;
    let c = heap.pop().unwrap().0;

    print!(
        "{}",
        if c >= a + b {
            (a + b) * 2 - 1
        } else {
            a + b + c
        }
    );
}
