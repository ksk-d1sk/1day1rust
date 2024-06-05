use std::collections::HashMap;
use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut map: HashMap<&str, usize> = HashMap::from_iter((0..n).map(|_| (next!(), 0)));

    for name in tokens {
        map.entry(name).and_modify(|cnt| *cnt += 1);
    }

    let mut v: Vec<_> = map.into_iter().collect();

    v.sort_unstable_by(|a, b| {
        if a.1 == b.1 {
            a.0.cmp(b.0)
        } else {
            b.1.cmp(&a.1)
        }
    });

    for (name, cnt) in v {
        let _ = writeln!(output, "{name} {cnt}");
    }

    print!("{output}");
}