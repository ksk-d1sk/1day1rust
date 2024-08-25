use std::collections::HashSet;
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

    let mut v = [next!(), next!(), next!(), next!()];
    let mut set = HashSet::new();

    v.sort_unstable();

    for i in 0..4 {
        for j in 0..4 {
            let p = format!("{} {}", v[i], v[j]);
            if !set.contains(&p) {
                let _ = writeln!(output, "{p}");
                set.insert(p);
            }
        }
    }

    print!("{output}");
}