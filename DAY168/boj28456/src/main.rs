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
    let mut v: Vec<VecDeque<_>> = (0..n)
        .map(|_| (0..n).map(|_| get!(u16)).collect())
        .collect();

    let q = get!(u8);
    for _ in 0..q {
        if get!(u8) == 1 {
            let idx = get!(usize) - 1;
            let temp = v[idx].pop_back().unwrap();
            v[idx].push_front(temp);
        } else {
            let clone_v = v.clone();

            for i in 0..n {
                for j in 0..n {
                    v[j][n - i - 1] = clone_v[i][j];
                }
            }
        }
    }

    for deque in v {
        for elem in deque {
            let _ = write!(output, "{elem} ");
        }
        output.push('\n');
    }

    print!("{output}");
}