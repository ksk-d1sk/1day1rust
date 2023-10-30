use std::collections::HashSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(usize);
    let v: Vec<_> = (0..n).map(|_| get!(i32)).collect();
    let set: HashSet<_> = v.iter().collect();
    let mut candidate = Vec::with_capacity(n / 2);
    let mut cnt = 0;

    for i in 1..=(n / 2) {
        candidate.push(v[i] - v[0]);
    }

    for i in candidate {
        let mut check = true;

        for &j in v.iter() {
            if !set.contains(&(j - i)) && !set.contains(&(j + i)) {
                check = false;
                break;
            }
        }

        if check {
            cnt += 1;
            let _ = write!(output, "{} ", i);
        }
    }

    print!("{cnt}\n{output}");
}