use std::io;
use std::fmt::Write;
use std::collections::VecDeque;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let mut deque: VecDeque<(usize, i16)> = input
        .skip(1)
        .flat_map(str::parse)
        .enumerate()
        .collect();

    while let Some((num, i)) = deque.pop_front() {
        let _ = write!(output, "{} ", num + 1);

        if deque.is_empty() {
            break;
        }

        if i > 0 {
            for _ in 1..i {
                let tmp = deque.pop_front().unwrap();
                deque.push_back(tmp);
            }
        } else {
            for _ in 0..(-i) {
                let tmp = deque.pop_back().unwrap();
                deque.push_front(tmp);
            }
        }
    }

    print!("{output}");
}