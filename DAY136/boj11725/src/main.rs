use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

// 못품 GG
fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(usize);
    let mut v = vec![0; n];
    let mut visit = vec![true; n + 1];
    let input: Vec<_> = (0..(n - 1)).map(|_| get!(usize, usize)).collect();
    let mut queue = VecDeque::new();

    queue.push_back(1);

    while let Some(value) = queue.pop_front() {
        visit[value] = false;

        for &(a, b) in input.iter() {
            if value == a && visit[b] {
                v[b - 1] = value;
                queue.push_back(b);
            }

            if value == b && visit[a] {
                v[a - 1] = value;
                queue.push_back(a);
            }
        }
    }

    for i in &v[1..] {
        let _ = writeln!(output, "{}", i);
    }

    print!("{output}");
}