use std::collections::{HashSet, LinkedList};

fn main() {
    let mut v: Vec<i32> = (0..5).map(|i| 1 << i).collect();
    v.extend([31, 57, 99, 163]);
    assert_eq!(v, [1, 2, 4, 8, 16, 31, 57, 99, 163]);

    let v: Vec<i32> = from_iter(1..10);
    let hset: HashSet<i32> = from_iter(1..15);
    let list: LinkedList<i32> = from_iter((1..10).map(|i| i * i).rev());
}

fn from_iter<C, I>(iter: I) -> C
where
    C: Default + Extend<I::Item>,
    I: Iterator,
{
    let mut collection: C = Default::default();
    collection.extend(iter);
    collection
}