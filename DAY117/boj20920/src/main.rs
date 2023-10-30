use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        () => { input.next().unwrap() };
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = get!(usize, usize);
    let mut map = HashMap::with_capacity(n);

    for word in (0..n).map(|_| get!()).filter(|s| s.len() >= m) {
        map.entry(word)
            .and_modify(|e| *e += 1)
            .or_insert(1_usize);
    }

    let mut v: Vec<_> = map.into_iter().collect();
    
    v.sort_unstable_by(|a, b| {
        if a.1 == b.1 {
            let al = a.0.len();
            let bl = b.0.len();

            if al ==  bl {
                a.0.cmp(b.0)
            } else {
                bl.cmp(&al)
            }
        } else {
            b.1.cmp(&a.1)
        }
    });

    for (word, _) in v {
        let _ = writeln!(output, "{word}");
    }

    print!("{output}");
}