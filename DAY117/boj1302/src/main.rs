use std::collections::HashMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next().unwrap() };
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(usize);
    let mut map = HashMap::with_capacity(n);
    let mut max = 1;

    for _ in 0..n {
        map.entry(get!()).and_modify(|e| {
            *e += 1;
            max = max.max(*e);
        }).or_insert(1_usize);
    }

    print!(
        "{}",
        map.into_iter()
            .filter_map(|(k, v)| (v == max).then_some(k))
            .min()
            .unwrap()
    );
}
