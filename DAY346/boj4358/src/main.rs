use std::collections::BTreeMap;
use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut output = String::new();
    let tokens = input.lines();
    let mut total = 0.0;
    let mut map = BTreeMap::new();

    for name in tokens {
        total += 1.0;
        map.entry(name).and_modify(|e| *e += 1.0).or_insert(1.0);
    }

    for (name, cnt) in map {
        let _ = writeln!(output, "{name} {:.04}", 100.0 * cnt / total);
    }

    print!("{output}");
}