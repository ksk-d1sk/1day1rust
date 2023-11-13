use std::collections::{HashSet, BTreeSet, LinkedList, HashMap, BTreeMap};

fn main() {
    // FromIterator를 구현하는 타입으로 변환할 수 있다.

    let args: Vec<String> = std::env::args().collect();

    let args: HashSet<String> = std::env::args().collect();
    let args: BTreeSet<String> = std::env::args().collect();
    let args: LinkedList<String> = std::env::args().collect();

    let args: HashMap<String, usize> = std::env::args().zip(0..).collect();
    let args: BTreeMap<String, usize> = std::env::args().zip(0..).collect();

    println!("{:?}", (0..10).size_hint());
}
