use std::collections::BTreeMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(usize);
    let mut map = BTreeMap::new();
    let mut max = 0;
    let mut answer = 0;

    for _ in 0..n {
        map.entry(get!(i64)).and_modify(|e| *e += 1).or_insert(1);
    }

    for (k, v) in map {
        if v > max {
            answer = k;
            max = v;
        }
    }

    print!("{answer}");
}