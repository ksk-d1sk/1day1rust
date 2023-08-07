use std::io;
use std::collections::HashSet;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        () => { input.next().unwrap() };
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(usize);
    let mut v = Vec::with_capacity(n);
    let mut q_idx = 0;

    for i in 0..n {
        let s = get!();
        if s == "?" {
            q_idx = i as i32;
        }
        v.push(s);
    }

    let sw = v.get((q_idx - 1) as usize).map(|e| e.chars().last().unwrap());
    let ew = v.get((q_idx + 1) as usize).map(|e| e.chars().next().unwrap());

    let set: HashSet<_> = v.into_iter().collect();

    for candidate_word in input.skip(1) {
        let f = Some(candidate_word.chars().next().unwrap());
        let l = Some(candidate_word.chars().last().unwrap());

        if (sw.is_none() || sw == f) && (ew.is_none() || ew == l) && !set.contains(candidate_word) {
            print!("{}", candidate_word);
            break;
        }
    }
}
