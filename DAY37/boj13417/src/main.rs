use std::io::Read;
use std::collections::VecDeque;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines().skip(1);

    while let Some(str_n) = input.next() {
        let n: usize = str_n.parse().unwrap();
        let mut deque = VecDeque::with_capacity(n);

        for c in input.next()
                      .unwrap()
                      .split_ascii_whitespace()
                      .map(|e| e.as_bytes()[0] as char)
        {
            if let Some(front_c) = deque.front() {
                if c > *front_c {
                    deque.push_back(c);
                } else {
                    deque.push_front(c);
                }
            } else {
                deque.push_back(c);
            }
        }

        for c in deque {
            output.push(c);
        }
        output.push('\n');
    }

    print!("{output}");
}