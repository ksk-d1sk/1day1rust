use std::cmp::Reverse;
use std::fmt::Write;
use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut input = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(usize);
    let mut v: Vec<_> = (0..n).map(|_| get!(usize, u8, u16)).collect();
    let mut appear = vec![0; n + 1];
    let mut count = 0;

    v.sort_unstable_by_key(|k| Reverse(k.2));

    for (cu, st, _) in v {
        if count >= 3 { break }

        if appear[cu] < 2 {
            appear[cu] += 1;
            count += 1;
            let _ = writeln!(output, "{} {}", cu, st);
        }
    }

    print!("{output}");
}
