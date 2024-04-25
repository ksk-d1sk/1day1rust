use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::io::*;

#[derive(PartialEq, Eq, Ord)]
struct Heap(RefCell<Vec<u32>>);

impl Heap {
    fn new(v: Vec<u32>) -> Self {
        Self(RefCell::new(v))
    }

    fn pop(&self) {
        self.0.borrow_mut().pop();
    }

    fn last(&self) -> Option<u32> {
        self.0.borrow().last().copied()
    }
}

impl PartialOrd for Heap {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if !self.0.borrow().is_empty() && !other.0.borrow().is_empty() {
            Some(other.0.borrow().last().cmp(&self.0.borrow().last()))
        } else if self.0.borrow().is_empty() {
            Some(std::cmp::Ordering::Less)
        } else if other.0.borrow().is_empty() {
            Some(std::cmp::Ordering::Greater)
        } else {
            None
        }
    }
}

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (_, m) = next!(usize, usize);
    let mut bh = BinaryHeap::with_capacity(m);
    let mut i = 1;
    let mut check = true;

    for _ in 0..m {
        let k = next!(usize);
        bh.push(Heap::new(Vec::from_iter((0..k).map(|_| next!(u32)))));
    }

    while let Some(heap) = bh.pop() {
        if heap.last().is_none() {
            break;
        } else if heap.last() == Some(i) {
            i += 1;
            heap.pop();
            bh.push(heap);
        } else {
            check = false;
            break;
        }
    }

    if check {
        print!("Yes");
    } else {
        print!("No");
    }
}