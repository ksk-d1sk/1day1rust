use std::collections::BinaryHeap;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut heap: BinaryHeap<u64> = input.split_ascii_whitespace()
        .flat_map(str::parse)
        .collect();

    heap.pop();

    print!("{}", heap.pop().unwrap());
}