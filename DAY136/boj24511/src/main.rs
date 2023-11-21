use std::collections::VecDeque;
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
    let data_s: Vec<_> = (0..n).map(|_| get!(u8)).collect();
    let mut stack_q: VecDeque<_> = data_s.iter()
        .zip((0..n).map(|_| get!(u32)))
        .filter_map(|(ds, v)| {
            if *ds == 0 {
                Some(v)
            } else {
                None
            }
        })
        .collect();

    let m = get!(usize);
    (0..m).map(|_| get!(u32)).for_each(|e| stack_q.push_front(e));

    for _ in 0..m {
        let _ = write!(output, "{} ", stack_q.pop_back().unwrap());
    }

    print!("{output}");
}